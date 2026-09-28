use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressUnit {
    #[default]
    Bytes,
    Count
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ProgressStatus {
    #[default]
    NotStarted,
    Success,
    Skipped(Arc<str>),
    Failed(Arc<str>),
    HashMismatch(Arc<str>)
}

pub enum ProgressEvent {
    Started { id: Arc<str>, label: &'static str, unit: ProgressUnit, total: u64 },
    Advance { id: Arc<str>, current: u64, total: u64 },
    Completed { id: Arc<str>, status: ProgressStatus }
}

pub trait ProgressObserver: Send + Sync + 'static {
    fn on_event(&self, event: ProgressEvent);
}

pub struct NoopObserver;

impl ProgressObserver for NoopObserver {
    fn on_event(&self, _event: ProgressEvent) {}
}

static GLOBAL_OBSERVER: OnceLock<Arc<dyn ProgressObserver>> = OnceLock::new();

pub fn set_observer(observer: Arc<dyn ProgressObserver>) { let _ = GLOBAL_OBSERVER.set(observer); }

pub fn observer() -> Arc<dyn ProgressObserver> {
    GLOBAL_OBSERVER.get().cloned().unwrap_or_else(|| Arc::new(NoopObserver))
}

#[derive(Clone)]
pub struct Progress {
    inner: Arc<ProgressInner>
}

struct ProgressInner {
    id: Arc<str>,
    total: AtomicU64,
    current: AtomicU64,
    active: AtomicBool,
    observer: Arc<dyn ProgressObserver>
}

impl Progress {
    pub fn start(
        id: impl Into<Arc<str>>,
        label: &'static str,
        unit: ProgressUnit,
        total: u64
    ) -> Self {
        let id = id.into();
        let observer = observer();

        observer.on_event(ProgressEvent::Started {
            id: Arc::clone(&id),
            label,
            unit,
            total
        });

        Self {
            inner: Arc::new(ProgressInner {
                id,
                total: AtomicU64::new(total),
                current: AtomicU64::new(0),
                active: AtomicBool::new(true),
                observer
            })
        }
    }

    pub fn advance(&self, delta: u64) {
        let current = self.inner.current.fetch_add(delta, Ordering::Relaxed) + delta;
        let total = self.inner.total.load(Ordering::Relaxed);

        self.inner.observer.on_event(ProgressEvent::Advance {
            id: Arc::clone(&self.inner.id),
            current,
            total
        });
    }

    pub fn set_total(&self, total: u64) { self.inner.total.store(total, Ordering::Relaxed); }

    pub fn finish(self) { self.inner.complete(ProgressStatus::Success); }

    pub fn complete(self, status: ProgressStatus) { self.inner.complete(status); }

    pub fn fail(self, reason: impl Into<Arc<str>>) {
        self.inner.complete(ProgressStatus::Failed(reason.into()));
    }
}

impl ProgressInner {
    fn complete(&self, status: ProgressStatus) {
        if self.active.swap(false, Ordering::SeqCst) {
            self.observer.on_event(ProgressEvent::Completed {
                id: Arc::clone(&self.id),
                status
            });
        }
    }
}

impl Drop for ProgressInner {
    fn drop(&mut self) { self.complete(ProgressStatus::Success); }
}
