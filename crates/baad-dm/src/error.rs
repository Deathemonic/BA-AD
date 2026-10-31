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
    ZeroConcurrency,

    #[error("Segment list must not be empty")]
    EmptySegments,

    #[error("ZIP extraction does not support segment sources")]
    ZipSegmentsNotSupported,

    #[error("ZIP extraction does not support output transforms")]
    ZipTransformNotSupported,

    #[error("Segment transfer size overflow")]
    SegmentSizeOverflow,

    #[error("Transfer did not start")]
    TransferNotStarted,

    #[error(
        "Transfer verification failed: expected size {expected_size:?}, hash {expected_hash:?}"
    )]
    TransferVerificationFailed { expected_size: Option<u64>, expected_hash: Option<Box<str>> },

    #[error("Output verification failed: expected size {expected_size:?}, hash {expected_hash:?}")]
    OutputVerificationFailed { expected_size: Option<u64>, expected_hash: Option<Box<str>> },

    #[error("Output exceeds expected size {expected}")]
    OutputSizeExceeded { expected: u64 },

    #[error("Output size mismatch: expected {expected}, got {actual}")]
    OutputSizeMismatch { expected: u64, actual: u64 },

    #[error("Invalid or mismatched Content-Range")]
    InvalidRangeResponse,

    #[error("Encoded byte-range representation")]
    EncodedRangeResponse,

    #[error("Unexpected full-download response")]
    UnexpectedDownloadResponse,

    #[error("Response size overflow")]
    ResponseSizeOverflow,

    #[error("Response size differs from expected size")]
    ResponseSizeMismatch,

    #[error("Server size differs from expected size")]
    ServerSizeMismatch,

    #[error("Output path must be a safe relative file path")]
    InvalidOutputPath,

    #[error("ZIP extraction requires a target member")]
    MissingZipTarget,

    #[error("ZIP extraction requires a URL source")]
    MissingZipUrl,

    #[error("ZIP member exceeds extraction limits")]
    ZipMemberTooLarge,

    #[error("ZIP member is out of bounds")]
    ZipMemberOutOfBounds,

    #[error("Invalid local file header")]
    InvalidZipLocalHeader,

    #[error("ZIP range exceeds fetch limits")]
    ZipRangeTooLarge,

    #[error("ZIP range body size differs from requested size")]
    ZipRangeSizeMismatch,

    #[error("ZIP expanded size overflow")]
    ZipExpandedSizeOverflow,

    #[error("ZIP expanded size mismatch")]
    ZipExpandedSizeMismatch,

    #[error("Could not determine ZIP file size")]
    UnknownZipSize,

    #[error("Invalid or missing End of Central Directory")]
    InvalidZipEocd,

    #[error("ZIP central directory exceeds limit")]
    ZipDirectoryTooLarge,

    #[error("ZIP central directory is out of bounds")]
    ZipDirectoryOutOfBounds,

    #[error("Invalid or truncated central directory")]
    InvalidZipDirectory
}
