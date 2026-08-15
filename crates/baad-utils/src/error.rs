use std::error::Error as StdError;
use std::io;

#[cfg(feature = "logs")]
use eyre::Report;
use thiserror::Error;
#[cfg(feature = "logs")]
use tracing::warn;

#[derive(Error, Debug)]
pub enum FileError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    External(Box<dyn StdError + Send + Sync>),

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
    External(Box<dyn StdError + Send + Sync>),

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

#[cfg(feature = "logs")]
pub trait IntoEyreReport {
    fn into_eyre_report(self) -> Report;
}

#[cfg(feature = "logs")]
impl<E: StdError + Send + Sync + 'static> IntoEyreReport for E {
    fn into_eyre_report(self) -> Report {
        let mut report = Report::msg(self.to_string());
        let mut current = self.source();

        while let Some(source) = current {
            report = report.wrap_err(source.to_string());
            current = source.source();
        }

        report
    }
}

#[cfg(feature = "logs")]
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
