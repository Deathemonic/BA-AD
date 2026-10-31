use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

use baad_shared::{ProgressEvent, ProgressStatus, ProgressUnit, observer};
use baad_utils::JsonError;
use baad_utils::file::{create_parent_dir, load_file, save_file};
use serde_json::Value;

fn intern_label(label: &str) -> &'static str {
    static LABELS: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();
    let labels = LABELS.get_or_init(|| Mutex::new(HashSet::new()));

    let Ok(mut guard) = labels.lock() else {
        return "";
    };
    if let Some(existing) = guard.get(label) {
        return existing;
    }
    let leaked: &'static str = String::from(label).leak();
    guard.insert(leaked);
    leaked
}

pub async fn json_load_string(path: &Path) -> Result<String, JsonError> {
    let bytes = load_file(path).await?;
    let value: Value = serde_json::from_slice(&bytes).map_err(JsonError::SerdeJson)?;
    serde_json::to_string(&value).map_err(JsonError::SerdeJson)
}

pub async fn json_save_string(path: &Path, json: &str) -> Result<(), JsonError> {
    let value: Value = serde_json::from_str(json).map_err(JsonError::SerdeJson)?;
    let pretty = serde_json::to_string_pretty(&value).map_err(JsonError::SerdeJson)?;
    create_parent_dir(path).await?;
    save_file(path, pretty.as_bytes()).await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

impl LogLevel {
    #[cfg(feature = "c-api")]
    pub(crate) const fn from_repr(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Trace),
            1 => Some(Self::Debug),
            2 => Some(Self::Info),
            3 => Some(Self::Warn),
            4 => Some(Self::Error),
            _ => None
        }
    }
}

#[cfg(feature = "c-api")]
pub const fn unit_from_repr(value: i32) -> Option<ProgressUnit> {
    match value {
        0 => Some(ProgressUnit::Bytes),
        1 => Some(ProgressUnit::Count),
        _ => None
    }
}

pub fn progress_started(id: &str, label: &str, unit: ProgressUnit, total: u64) {
    observer().on_event(ProgressEvent::Started {
        id: Arc::from(id),
        label: intern_label(label),
        unit,
        total
    });
}

pub fn progress_advance(id: &str, current: u64, total: u64) {
    observer().on_event(ProgressEvent::Advance {
        id: Arc::from(id),
        current,
        total
    });
}

pub fn progress_completed(id: &str) {
    observer().on_event(ProgressEvent::Completed {
        id: Arc::from(id),
        status: ProgressStatus::Success
    });
}

pub fn progress_failed(id: &str, reason: &str) {
    observer().on_event(ProgressEvent::Completed {
        id: Arc::from(id),
        status: ProgressStatus::Failed(String::from(reason).into())
    });
}

pub fn join_fields(fields: &[(&str, &str)]) -> String {
    let mut joined = String::new();
    for (index, (name, value)) in fields.iter().enumerate() {
        if index > 0 {
            joined.push_str(", ");
        }
        joined.push_str(name);
        joined.push('=');
        joined.push_str(value);
    }
    joined
}

pub fn render_fields(fields: &[(&str, &str)]) -> Option<String> {
    match fields {
        [] => None,
        [(_, value)] => Some((*value).to_owned()),
        fields => Some(join_fields(fields))
    }
}

pub fn log_message(level: LogLevel, success: bool, message: &str, value: Option<&str>) {
    match (level, value) {
        (LogLevel::Trace, None) => baad_utils::trace!(message),
        (LogLevel::Trace, Some(value)) => baad_utils::trace!(value, message),
        (LogLevel::Debug, None) => baad_utils::debug!(message),
        (LogLevel::Debug, Some(value)) => baad_utils::debug!(value, message),
        (LogLevel::Info, None) if success => baad_utils::info!(success = true, message),
        (LogLevel::Info, Some(value)) if success => {
            baad_utils::info!(success = true, value, message);
        }
        (LogLevel::Info, None) => baad_utils::info!(message),
        (LogLevel::Info, Some(value)) => baad_utils::info!(value, message),
        (LogLevel::Warn, None) => baad_utils::warn!(message),
        (LogLevel::Warn, Some(value)) => baad_utils::warn!(value, message),
        (LogLevel::Error, None) => baad_utils::error!(message),
        (LogLevel::Error, Some(value)) => baad_utils::error!(value, message)
    }
}
