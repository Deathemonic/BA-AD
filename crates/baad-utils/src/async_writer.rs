use std::io;
use std::sync::Mutex;

use tracing_subscriber::fmt::MakeWriter;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

static LOG_GUARD: Mutex<Option<WorkerGuard>> = Mutex::new(None);

#[derive(Clone)]
pub struct AsyncMakeWriter {
    writer: NonBlocking,
}

impl AsyncMakeWriter {
    pub fn new() -> (Self, WorkerGuard) {
        let (non_blocking, guard) = tracing_appender::non_blocking(io::stdout());
        (Self { writer: non_blocking }, guard)
    }
}

impl<'a> MakeWriter<'a> for AsyncMakeWriter {
    type Writer = NonBlocking;

    fn make_writer(&'a self) -> Self::Writer {
        self.writer.clone()
    }
}

pub fn store_guard(guard: WorkerGuard) {
    if let Ok(mut g) = LOG_GUARD.lock() {
        *g = Some(guard);
    }
}

pub fn flush_logs() {
    if let Ok(mut g) = LOG_GUARD.lock() {
        g.take();
    }
}
