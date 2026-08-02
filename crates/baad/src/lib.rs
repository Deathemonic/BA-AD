pub mod api;
pub mod catalog;
pub mod cdn;
pub mod download;
pub mod error;
pub mod strategy;

pub use baad_shared::consts::*;
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
#[cfg(feature = "logs")]
pub use baad_utils::config::{LoggingConfig, init_logging, init_logging_with_model};
#[cfg(feature = "logs")]
pub use baad_utils::progress::{DownloadProgressHandler, ProgressModel};
pub use baad_utils::{file, network};
pub use error::*;
pub use tracing::{debug, error, info, trace, warn};
