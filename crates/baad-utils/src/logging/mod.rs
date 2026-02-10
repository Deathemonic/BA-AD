mod handler;
pub mod sink;
mod writer;

pub mod config;

pub use config::{LoggingConfig, LoggingOutput, init_logging};
pub use sink::LoggingSink;
pub use writer::{AsyncMakeWriter, flush_logs};
