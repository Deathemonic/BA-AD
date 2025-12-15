use std::io;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

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

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for AsyncMakeWriter {
    type Writer = NonBlocking;

    fn make_writer(&'a self) -> Self::Writer {
        self.writer.clone()
    }
}
