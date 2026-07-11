pub mod api;
pub mod catalog;
pub mod cdn;
pub mod download;
pub mod error;
pub mod strategy;

pub use baad_shared::{
    BuildType,
    DownloadEvent,
    DownloadObserver,
    DownloadStatus,
    NoopObserver,
    Platform,
    client,
    set_client,
    set_observer
};
pub use baad_utils::config::{LoggingConfig, init_logging};
pub use baad_utils::file;
pub use error::*;
pub use tracing::{debug, error, info, trace, warn};
