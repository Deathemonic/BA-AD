use std::future::Future;
use std::process::exit;

use eyre::Result;
use tracing::error;

use crate::logging::config::{LoggingConfig, init_logging};

pub fn run<F>(f: F)
where
    F: FnOnce() -> Result<()>
{
    if let Err(e) = init_logging(LoggingConfig::default()) {
        error!("Failed to initialize logging: {}", e);
        exit(1);
    }

    if let Err(e) = f() {
        error!("{:?}", e);
        exit(1);
    }
}

pub async fn run_async<F, Fut>(f: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<()>>
{
    if let Err(e) = init_logging(LoggingConfig::default()) {
        error!("Failed to initialize logging: {}", e);
        exit(1);
    }

    if let Err(e) = f().await {
        error!("{:?}", e);
        exit(1);
    }
}
