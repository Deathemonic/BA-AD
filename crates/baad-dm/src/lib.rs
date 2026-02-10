pub mod client;
pub mod download;
pub mod downloader;
pub mod error;
pub mod zip;

pub use baad_core::{
    DownloadEvent,
    DownloadObserver,
    DownloadStatus,
    NoopObserver,
    observer,
    set_observer
};
pub use client::{HttpClientConfig, create_http_client};
pub use download::{Download, HashType, Summary, detect_hash_type, verify_hash};
pub use downloader::{Downloader, DownloaderConfig};
pub use error::Error;
pub use zip::{ZipExtractor, ZipFileInfo};
