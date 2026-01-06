use std::sync::Once;
use std::iter::successors;
use std::{error, fmt, panic, io};

use eyre::{EyreHandler, Report};
use thiserror::Error;
use tracing::{error, warn};

pub trait IntoEyreReport {
    fn into_eyre_report(self) -> Report;
}

impl IntoEyreReport for anyhow::Error {
    fn into_eyre_report(self) -> Report {
        let mut report = Report::msg(self.to_string());

        for source in successors(self.source(), |e| e.source()) {
            report = report.wrap_err(source.to_string());
        }

        report
    }
}

#[derive(Debug)]
pub struct TracingHandler;

impl TracingHandler {
    fn new() -> Self {
        Self
    }
}

impl EyreHandler for TracingHandler {
    fn debug(
        &self,
        error: &(dyn error::Error + 'static),
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        let mut prev_msg = error.to_string();
        error!("{}", error);

        for cause in successors(error.source(), |e| (*e).source()) {
            let cause_msg = cause.to_string();
            if cause_msg != prev_msg {
                error!("{}", cause);
            }
            prev_msg = cause_msg;
        }

        Ok(())
    }
}

pub fn log_recoverable_error(error: &Report, recovery_action: &str) {
    if let Some(cause) = error.source() {
        if cause.to_string() == error.to_string() {
            warn!(
                recovery = recovery_action,
                "Recoverable error, continuing: {}", error
            );
        } else {
            warn!(
                cause = %cause,
                recovery = recovery_action,
                "Recoverable error, continuing: {}", error
            );
        }
    } else {
        warn!(
            recovery = recovery_action,
            "Recoverable error, continuing: {}", error
        );
    }
}

pub fn install() -> Result<(), ConfigError> {
    eyre::set_hook(Box::new(|_| Box::new(TracingHandler::new())))
        .map_err(|e| ConfigError::External(Box::new(e)))?;

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        panic::set_hook(Box::new(|panic_info| {
            let msg = match panic_info.payload().downcast_ref::<&str>() {
                Some(s) => s.to_string(),
                None => match panic_info.payload().downcast_ref::<String>() {
                    Some(s) => s.clone(),
                    None => "Unknown panic".to_string(),
                },
            };

            let location = panic_info
                .location()
                .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
                .unwrap_or_default();

            error!(msg = %msg, location = %location, "Panic occurred");
        }));
    });

    Ok(())
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    External(Box<dyn error::Error + Send + Sync>),

    #[error("Failed to create app directories")]
    AppDirectoryCreationFailed,

    #[error("App name has already been set")]
    AppNameAlreadySet,

    #[error("Data directory has already been set")]
    DataDirAlreadySet,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    External(Box<dyn error::Error + Send + Sync>),

    #[error("Failed to initialize logging")]
    LoggingInitFailed,
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Unable to set proxy")]
    Proxy,
}

#[derive(Error, Debug)]
pub enum JsonError {
    #[error(transparent)]
    Io(#[from] FileError),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("Invalid UTF-8 in JSON file")]
    InvalidUtf8,
}