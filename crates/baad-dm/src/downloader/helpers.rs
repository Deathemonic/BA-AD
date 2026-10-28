use std::fs::{File, TryLockError, metadata};
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use tokio::fs;
use tokio::sync::Semaphore;
use tokio::task::spawn_blocking;
use tokio::time::sleep;

use crate::client::{parse_accept_ranges, parse_content_length};
use crate::download::verify_hash;
use crate::error::Error;
use crate::zip::ZipCache;

const LOCK_RETRY_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug, Default)]
pub struct Cancellation {
    cancelled: AtomicBool
}

pub struct CancellationGuard {
    cancellation: Arc<Cancellation>,
    armed: bool
}

#[derive(Debug)]
pub struct ServerInfo {
    pub resumable: bool,
    pub content_length: Option<u64>,
    pub resolved_url: String
}

#[derive(Debug)]
pub struct SharedExecution {
    pub client: ClientWithMiddleware,
    pub requests: Arc<Semaphore>,
    pub transforms: Arc<Semaphore>,
    pub blocking: Arc<Semaphore>,
    pub zip_cache: ZipCache
}

impl Cancellation {
    pub fn cancel(&self) { self.cancelled.store(true, Ordering::Release); }

    pub fn check(&self) -> Result<(), Error> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(Error::Cancelled);
        }

        Ok(())
    }
}

impl CancellationGuard {
    pub const fn new(cancellation: Arc<Cancellation>) -> Self { Self { cancellation, armed: true } }

    pub const fn disarm(&mut self) { self.armed = false; }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        if self.armed {
            self.cancellation.cancel();
        }
    }
}

impl SharedExecution {
    pub fn new(client: ClientWithMiddleware, max_requests: usize, max_transforms: usize) -> Self {
        Self {
            client,
            requests: Arc::new(Semaphore::new(max_requests)),
            transforms: Arc::new(Semaphore::new(max_transforms)),
            blocking: Arc::new(Semaphore::new(max_requests)),
            zip_cache: ZipCache::new()
        }
    }

    pub async fn verify_file(
        &self,
        path: &Path,
        size: Option<u64>,
        hash: Option<&String>
    ) -> Result<bool, Error> {
        if hash.is_none() {
            return match fs::metadata(path).await {
                Ok(metadata) => Ok(size.is_none_or(|expected| metadata.len() == expected)),
                Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
                Err(error) => Err(error.into())
            };
        }

        let permit =
            self.blocking.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;
        let path = path.to_path_buf();
        let hash = hash.cloned();

        spawn_blocking(move || {
            let _permit = permit;
            verify_file(&path, size, hash.as_ref())
        })
        .await?
    }

    pub async fn inspect(
        &self,
        url: &Url,
        cancellation: &Cancellation
    ) -> Result<ServerInfo, Error> {
        cancellation.check()?;

        let _permit = self.requests.acquire().await.map_err(|_| Error::LimiterClosed)?;

        let response = self.client.head(url.clone()).send().await?;
        response.error_for_status_ref()?;

        Ok(ServerInfo {
            resumable: parse_accept_ranges(response.headers()),
            content_length: parse_content_length(response.headers()),
            resolved_url: response.url().to_string()
        })
    }
}

pub async fn lock_file(path: &Path, cancellation: &Cancellation) -> Result<Arc<File>, Error> {
    cancellation.check()?;
    ensure_parent_dir(path).await?;

    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .await?
        .into_std()
        .await;

    loop {
        cancellation.check()?;

        match file.try_lock() {
            Ok(()) => return Ok(Arc::new(file)),
            Err(TryLockError::WouldBlock) => sleep(LOCK_RETRY_INTERVAL).await,
            Err(TryLockError::Error(error)) => return Err(error.into())
        }
    }
}

pub async fn ensure_parent_dir(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    Ok(())
}

pub fn verify_file(path: &Path, size: Option<u64>, hash: Option<&String>) -> Result<bool, Error> {
    let metadata = match metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into())
    };

    if size.is_some_and(|expected| metadata.len() != expected) {
        return Ok(false);
    }

    verify_hash(path, hash)
}
