use std::iter::successors;

pub use baad_shared::{ConfigError, FileError, JsonError, NetworkError, ProgressError};
use eyre::Report;
use tracing::warn;

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
