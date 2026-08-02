use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use baad_shared::{DownloadEvent, DownloadObserver};
use derive_more::Debug;

#[derive(Clone, Debug)]
pub struct ProgressTracker {
    filename: Arc<str>,
    total_bytes: u64,
    downloaded_bytes: Arc<AtomicU64>,
    #[debug(skip)]
    observer: Arc<dyn DownloadObserver>
}

impl ProgressTracker {
    pub fn new(filename: Arc<str>, total_bytes: u64, observer: Arc<dyn DownloadObserver>) -> Self {
        Self {
            filename,
            total_bytes,
            downloaded_bytes: Arc::new(AtomicU64::new(0)),
            observer
        }
    }

    pub fn add_bytes(&self, bytes: u64) -> u64 {
        let downloaded = self.downloaded_bytes.fetch_add(bytes, Ordering::Relaxed) + bytes;
        self.observer.on_event(DownloadEvent::Progress {
            filename: Arc::clone(&self.filename),
            downloaded_bytes: downloaded,
            total_bytes: self.total_bytes
        });
        downloaded
    }
}
