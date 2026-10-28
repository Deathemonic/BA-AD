use std::fs::metadata;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;

use tokio::fs;
use tokio::sync::Semaphore;
use tokio::task::spawn_blocking;

use crate::download::verify_hash;
use crate::error::Error;

pub async fn verify_file(
    blocking: &Arc<Semaphore>,
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

    let permit = blocking.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;
    let path = path.to_path_buf();
    let hash = hash.cloned();

    spawn_blocking(move || {
        let _permit = permit;

        verify_file_sync(&path, size, hash.as_ref())
    })
    .await?
}

pub fn verify_file_sync(
    path: &Path,
    size: Option<u64>,
    hash: Option<&String>
) -> Result<bool, Error> {
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
