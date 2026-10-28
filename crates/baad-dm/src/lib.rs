pub mod client;
pub mod download;
pub mod downloader;
pub mod error;
pub mod segment;
pub mod zip;

pub use baad_shared::{
    NoopObserver,
    Progress,
    ProgressEvent,
    ProgressObserver,
    ProgressStatus,
    observer,
    set_observer
};
pub use client::{HttpClientConfig, create_http_client};
pub use download::{
    Download,
    DownloadSource,
    HashType,
    OutputTransform,
    ServerSegment,
    Summary,
    TransformError,
    detect_hash_type,
    verify_hash
};
pub use downloader::{Downloader, DownloaderConfig};
pub use error::Error;
pub use zip::{ZipCache, ZipExtractor, ZipFileInfo, ZipIndex, ZipSource};
