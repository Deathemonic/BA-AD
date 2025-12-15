pub mod async_writer;
pub mod config;
pub mod error;
pub mod file;
pub mod formatter;

pub use error::IntoEyreReport;

mod utils;

pub use tracing::{debug, error, info, trace, warn};
pub use utils::{run, run_async};