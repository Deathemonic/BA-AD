use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DownloadStatus {
    #[default]
    NotStarted,
    Success,
    Skipped(Arc<str>),
    Failed(Arc<str>),
    HashMismatch(Arc<str>)
}

pub enum DownloadEvent {
    Started { filename: Arc<str>, total_bytes: u64 },
    Progress { filename: Arc<str>, downloaded_bytes: u64, total_bytes: u64 },
    Completed { filename: Arc<str>, size: u64, status: DownloadStatus }
}

pub trait DownloadObserver: Send + Sync + 'static {
    fn on_event(&self, event: DownloadEvent);
}

pub struct NoopObserver;

impl DownloadObserver for NoopObserver {
    fn on_event(&self, _event: DownloadEvent) {}
}

static GLOBAL_OBSERVER: OnceLock<Arc<dyn DownloadObserver>> = OnceLock::new();

pub fn set_observer(observer: Arc<dyn DownloadObserver>) { let _ = GLOBAL_OBSERVER.set(observer); }

pub fn observer() -> Arc<dyn DownloadObserver> {
    GLOBAL_OBSERVER.get().cloned().unwrap_or_else(|| Arc::new(NoopObserver))
}
