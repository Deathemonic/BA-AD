use std::io;
use std::sync::Arc;

use tracing_subscriber::fmt::MakeWriter;

use crate::progress::{ProgressMakeWriter, ProgressModel, ProgressView};

pub trait LoggingSink {
    type Writer: for<'a> MakeWriter<'a> + Send + Sync + 'static;

    fn make_writer(&self) -> Self::Writer;
    fn is_view_sink(&self) -> bool;
}

impl LoggingSink for () {
    type Writer = fn() -> io::Stderr;

    fn make_writer(&self) -> Self::Writer { io::stderr }

    fn is_view_sink(&self) -> bool { false }
}

impl<M> LoggingSink for Arc<ProgressView<M>>
where
    M: ProgressModel + Send + Sync + 'static
{
    type Writer = ProgressMakeWriter<M>;

    fn make_writer(&self) -> Self::Writer { ProgressMakeWriter::new(self.clone()) }

    fn is_view_sink(&self) -> bool { true }
}
