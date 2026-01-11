use std::{io, result};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid URL '{url}': {reason}")]
    InvalidUrl { url: Box<str>, reason: Box<str> },

    #[error(transparent)]
    Http(#[from] reqwest_middleware::reqwest::Error),

    #[error(transparent)]
    HttpMiddleware(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("archive error: {message}")]
    Archive {
        message: Box<str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>
    },

    #[error("Unsupported compression method: {0}")]
    UnsupportedCompression(u16),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: Box<str>, actual: Box<str> },

    #[error("Server does not support range requests")]
    RangeNotSupported,

    #[error("Download failed: {0}")]
    DownloadFailed(Box<str>)
}
