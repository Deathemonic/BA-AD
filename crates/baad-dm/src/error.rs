use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hash(#[from] bacy::error::HashError),

    #[error("Invalid URL '{url}': {reason}")]
    InvalidUrl { url: Box<str>, reason: Box<str> },

    #[error(transparent)]
    Http(#[from] reqwest_middleware::reqwest::Error),

    #[error(transparent)]
    HttpMiddleware(#[from] reqwest_middleware::Error),

    #[error("HTTP error: {0}")]
    HttpStatus(reqwest_middleware::reqwest::StatusCode),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Archive error: {0}")]
    Archive(Box<str>),

    #[error("Unsupported compression method: {0}")]
    UnsupportedCompression(u16),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: Box<str>, actual: Box<str> },

    #[error("Server does not support range requests")]
    RangeNotSupported,

    #[error("Download failed: {0}")]
    DownloadFailed(Box<str>),

    #[error("Stream error at {downloaded} bytes")]
    Stream {
        downloaded: u64,
        #[source]
        source: Box<Error>
    }
}
