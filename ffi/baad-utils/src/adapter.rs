use std::path::Path;
use std::str::from_utf8;
use std::sync::Arc;

use reqwest::{Client, Error as HttpError, Proxy, Response};
use serde_json::{Value, from_str, to_string};

use crate::utils::ffi::BaadUtilsLogLevel;

pub async fn json_load_string(path: &Path) -> Result<String, baad_utils::JsonError> {
    let value: Value = baad_utils::json::load(path).await?;
    to_string(&value).map_err(baad_utils::JsonError::SerdeJson)
}

pub async fn json_save_string(path: &Path, json: &str) -> Result<(), baad_utils::JsonError> {
    let value: Value = from_str(json).map_err(baad_utils::JsonError::SerdeJson)?;
    baad_utils::json::save(path, &value).await
}

pub fn progress_started(id: &str, label: &str, unit: baad_shared::ProgressUnit, total: u64) {
    baad_shared::observer().on_event(baad_shared::ProgressEvent::Started {
        id: Arc::from(id),
        label: Arc::from(label),
        unit,
        total
    });
}

pub fn progress_advance(id: &str, current: u64, total: u64) {
    baad_shared::observer().on_event(baad_shared::ProgressEvent::Advance {
        id: Arc::from(id),
        current,
        total
    });
}

pub fn progress_completed(id: &str) {
    baad_shared::observer().on_event(baad_shared::ProgressEvent::Completed {
        id: Arc::from(id),
        status: baad_shared::ProgressStatus::Success
    });
}

pub fn progress_failed(id: &str, reason: &str) {
    baad_shared::observer().on_event(baad_shared::ProgressEvent::Completed {
        id: Arc::from(id),
        status: baad_shared::ProgressStatus::Failed(String::from(reason).into())
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

pub fn log_message(level: BaadUtilsLogLevel, success: bool, message: &str, value: Option<&str>) {
    match (level, value) {
        (BaadUtilsLogLevel::Trace, None) => {
            baad_utils::trace!(message);
        }
        (BaadUtilsLogLevel::Trace, Some(value)) => {
            baad_utils::trace!(value, message);
        }
        (BaadUtilsLogLevel::Debug, None) => {
            baad_utils::debug!(message);
        }
        (BaadUtilsLogLevel::Debug, Some(value)) => {
            baad_utils::debug!(value, message);
        }
        (BaadUtilsLogLevel::Info, None) if success => {
            baad_utils::info!(success = true, message);
        }
        (BaadUtilsLogLevel::Info, Some(value)) if success => {
            baad_utils::info!(success = true, value, message);
        }
        (BaadUtilsLogLevel::Info, None) => {
            baad_utils::info!(message);
        }
        (BaadUtilsLogLevel::Info, Some(value)) => {
            baad_utils::info!(value, message);
        }
        (BaadUtilsLogLevel::Warn, None) => {
            baad_utils::warn!(message);
        }
        (BaadUtilsLogLevel::Warn, Some(value)) => {
            baad_utils::warn!(value, message);
        }
        (BaadUtilsLogLevel::Error, None) => {
            baad_utils::error!(message);
        }
        (BaadUtilsLogLevel::Error, Some(value)) => {
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
                from_utf8(name).map_err(|e| e.to_string())?,
                from_utf8(value).map_err(|e| e.to_string())?
            ))
        })
        .collect()
}

pub async fn fetch_with_proxy(proxy: &Proxy, url: &str) -> Result<Response, HttpError> {
    Client::builder().proxy(proxy.clone()).build()?.get(url).send().await
}
