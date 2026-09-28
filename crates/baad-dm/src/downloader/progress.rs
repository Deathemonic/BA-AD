use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use baad_shared::{ProgressEvent, ProgressObserver};
use derive_more::Debug;

#[derive(Clone, Debug)]
pub struct ProgressTracker {
    filename: Arc<str>,
    total_bytes: u64,
    downloaded_bytes: Arc<AtomicU64>,
    #[debug(skip)]
    observer: Arc<dyn ProgressObserver>
}

impl ProgressTracker {
    pub fn new(filename: Arc<str>, total_bytes: u64, observer: Arc<dyn ProgressObserver>) -> Self {
        Self {
            filename,
            total_bytes,
            downloaded_bytes: Arc::new(AtomicU64::new(0)),
            observer
        }
    }

    pub fn add_bytes(&self, bytes: u64) -> u64 {
        let downloaded = self.downloaded_bytes.fetch_add(bytes, Ordering::Relaxed) + bytes;
        self.observer.on_event(ProgressEvent::Advance {
            id: Arc::clone(&self.filename),
            current: downloaded,
            total: self.total_bytes
        });
        downloaded
    }
}
