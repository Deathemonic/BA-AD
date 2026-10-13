use std::io;

use bacy::error::HashError;
use reqwest_middleware::reqwest::{Error as ReqwestError, StatusCode};
use thiserror::Error;
use tokio::task::JoinError;

use crate::download::TransformError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hash(#[from] HashError),

    #[error("Invalid URL '{url}': {reason}")]
    InvalidUrl { url: Box<str>, reason: Box<str> },

    #[error(transparent)]
    Http(#[from] ReqwestError),

    #[error(transparent)]
    HttpMiddleware(#[from] reqwest_middleware::Error),

    #[error("HTTP error: {0}")]
    HttpStatus(StatusCode),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Archive error: {0}")]
    Archive(Box<str>),

    #[error("Unsupported compression method: {0}")]
    UnsupportedCompression(u16),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: Box<str>, actual: Box<str> },

    #[error("Unsupported hash format: {0}")]
    UnsupportedHash(Box<str>),

    #[error("Invalid download: {0}")]
    InvalidDownload(Box<str>),

    #[error("Output transform failed: {0}")]
    Transform(#[source] TransformError),

    #[error("Download worker failed: {0}")]
    Worker(#[from] JoinError),

    #[error("Download cancelled")]
    Cancelled,

    #[error("Concurrency limiter closed")]
    LimiterClosed,

    #[error("Server does not support range requests")]
    RangeNotSupported,

    #[error("Download failed: {0}")]
    DownloadFailed(Box<str>),

    #[error("Stream error at {downloaded} bytes")]
    Stream {
        downloaded: u64,
        #[source]
        source: Box<Self>
    },

    #[error("Concurrency limits must be nonzero")]
    ZeroConcurrency
}
