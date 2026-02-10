pub mod api;
pub mod catalog;
pub mod cdn;
pub mod download;
pub mod strategy;

pub use baad_core::{
    BuildType,
    DownloadEvent,
    DownloadObserver,
    DownloadStatus,
    NoopObserver,
    Platform,
    set_observer
};
pub use baad_utils::config::{LoggingConfig, init_logging};
pub use baad_utils::{debug, error, file, info, trace, warn};
