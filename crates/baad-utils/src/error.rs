use std::iter::successors;
use std::{error, io};

use eyre::Report;
use thiserror::Error;
use tracing::warn;

#[derive(Error, Debug)]
pub enum FileError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    External(Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to create app directories")]
    AppDirectoryCreationFailed,

    #[error("App name has already been set")]
    AppNameAlreadySet,

    #[error("Data directory has already been set")]
    DataDirAlreadySet
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Unable to set proxy")]
    Proxy,

    #[error("Failed to extract value from response")]
    ExtractionFailed
}

#[derive(Error, Debug)]
pub enum JsonError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    File(#[from] FileError),

    #[error("Failed to convert file content to UTF-8")]
    InvalidUtf8,

    #[error("Failed to get file path")]
    PathError
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    External(Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to initialize logging")]
    LoggingInitFailed
}

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Progress view mutex was poisoned")]
    MutexPoisoned,

    #[error("Progress view has already been finished")]
    AlreadyFinished,

    #[error("Invalid UTF-8 in progress message")]
    InvalidUtf8
}

pub trait IntoEyreReport {
    fn into_eyre_report(self) -> Report;
}

impl<E: error::Error + Send + Sync + 'static> IntoEyreReport for E {
    fn into_eyre_report(self) -> Report {
        let mut report = Report::msg(self.to_string());

        for source in successors(self.source(), |e| e.source()) {
            report = report.wrap_err(source.to_string());
        }

        report
    }
}

pub fn log_recoverable_error(error: &Report, recovery_action: &str) {
    if let Some(cause) = error.source() {
        if cause.to_string() == error.to_string() {
            warn!(recovery = recovery_action, "Recoverable error, continuing: {}", error);
        } else {
            warn!(
                cause = %cause,
                recovery = recovery_action,
                "Recoverable error, continuing: {}", error
            );
        }
    } else {
        warn!(recovery = recovery_action, "Recoverable error, continuing: {}", error);
    }
}
