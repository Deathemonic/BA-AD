use std::fs::{self, File, TryLockError};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::id as process_id;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use tokio::fs as async_fs;
use tokio::time::sleep;
use tracing::warn;

use crate::error::Error;
use crate::execution::Cancellation;

pub const IO_BUFFER_SIZE: usize = 256 * 1024;
const LOCK_RETRY_INTERVAL: Duration = Duration::from_millis(50);

static TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

pub async fn lock_file(path: &Path, cancellation: &Cancellation) -> Result<Arc<File>, Error> {
    cancellation.check()?;
    ensure_parent_dir(path).await?;

    let file = async_fs::OpenOptions::new()
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
        async_fs::create_dir_all(parent).await?;
    }

    Ok(())
}

pub fn remove_temporary(path: &Path) {
    if let Err(error) = fs::remove_file(path)
        && error.kind() != ErrorKind::NotFound
    {
        warn!(path = %path.display(), cause = %error, "Failed to remove temporary output");
    }
}

pub fn sibling_path(destination: &Path, marker: &str) -> PathBuf {
    let id = TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
    let name = destination.file_name().and_then(|name| name.to_str()).unwrap_or("download");

    destination.with_file_name(format!(".{name}.{marker}-{}-{id}", process_id()))
}
