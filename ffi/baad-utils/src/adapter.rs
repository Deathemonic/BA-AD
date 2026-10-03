use std::path::Path;
use std::sync::Arc;

use baad_shared::{ProgressEvent, ProgressStatus, ProgressUnit};
use baad_utils::JsonError;
use serde_json::Value;

pub async fn json_load_string(path: &Path) -> Result<String, JsonError> {
    let value: Value = baad_utils::json::load(path).await?;
    serde_json::to_string(&value).map_err(JsonError::SerdeJson)
}

pub async fn json_save_string(path: &Path, json: &str) -> Result<(), JsonError> {
    let value: Value = serde_json::from_str(json).map_err(JsonError::SerdeJson)?;
    baad_utils::json::save(path, &value).await
}

pub fn progress_started(id: &str, label: &str, unit: ProgressUnit, total: u64) {
    baad_shared::observer().on_event(ProgressEvent::Started {
        id: Arc::from(id),
        label: Arc::from(label),
        unit,
        total
    });
}

pub fn progress_advance(id: &str, current: u64, total: u64) {
    baad_shared::observer().on_event(ProgressEvent::Advance {
        id: Arc::from(id),
        current,
        total
    });
}

pub fn progress_completed(id: &str) {
    baad_shared::observer().on_event(ProgressEvent::Completed {
        id: Arc::from(id),
        status: ProgressStatus::Success
    });
}

pub fn progress_failed(id: &str, reason: &str) {
    baad_shared::observer().on_event(ProgressEvent::Completed {
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

pub fn log_message(
    level: crate::utils::ffi::BaadUtilsLogLevel,
    success: bool,
    message: &str,
    value: Option<&str>
) {
    match (level, value) {
        (crate::utils::ffi::BaadUtilsLogLevel::Trace, None) => {
            baad_utils::trace!(message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Trace, Some(value)) => {
            baad_utils::trace!(value, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Debug, None) => {
            baad_utils::debug!(message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Debug, Some(value)) => {
            baad_utils::debug!(value, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Info, None) if success => {
            baad_utils::info!(success = true, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Info, Some(value)) if success => {
            baad_utils::info!(success = true, value, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Info, None) => {
            baad_utils::info!(message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Info, Some(value)) => {
            baad_utils::info!(value, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Warn, None) => {
            baad_utils::warn!(message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Warn, Some(value)) => {
            baad_utils::warn!(value, message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Error, None) => {
            baad_utils::error!(message);
        }
        (crate::utils::ffi::BaadUtilsLogLevel::Error, Some(value)) => {
            baad_utils::error!(value, message);
        }
    }
}

pub fn fields<'a>(
    names: &'a [diplomat_runtime::DiplomatStrSlice<'a>],
    values: &'a [diplomat_runtime::DiplomatStrSlice<'a>]
) -> Result<Vec<(&'a str, &'a str)>, String> {
    if names.len() != values.len() {
        return Err("Field names and values must have equal lengths".into());
    }
    names
        .iter()
        .zip(values)
        .map(|(name, value)| {
            Ok((
                std::str::from_utf8(name).map_err(|e| e.to_string())?,
                std::str::from_utf8(value).map_err(|e| e.to_string())?
            ))
        })
        .collect()
}

pub async fn fetch_with_proxy(
    proxy: &reqwest::Proxy,
    url: &str
) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::builder().proxy(proxy.clone()).build()?.get(url).send().await
}
