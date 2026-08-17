//! `UniFFI` bindings for `baad-utils`. Excludes the process-exiting runners,
//! tracing plumbing and generic progress types; error enums and
//! `LoggingConfig` are generated into `shadow.rs`.
//!
//! `UniFFI` arguments cross the boundary by value, so exported functions cannot
//! take records or maps by reference.
#![allow(clippy::needless_pass_by_value)]

use std::collections::HashMap;
use std::path::PathBuf;

use baad_utils::config::init_logging as init_logging_native;
use baad_utils::file::{
    clear_all as clear_all_native,
    create_parent_dir as create_parent_dir_native,
    data_dir as data_dir_native,
    filename_or as filename_or_native,
    get_data_path as get_data_path_native,
    get_output_dir as get_output_dir_native,
    is_dir_empty as is_dir_empty_native,
    load_file as load_file_native,
    save_file as save_file_native,
    set_app_name as set_app_name_native,
    set_data_dir as set_data_dir_native
};
use baad_utils::formatter::HumanBytes;
use baad_utils::network::fetch_version as fetch_version_native;
use baad_utils::progress::terminal;

use super::core;
pub use super::core::LogLevel;

#[uniffi::export]
pub fn init_logging(config: LoggingConfig) -> Result<(), ConfigError> {
    init_logging_native(config.into()).map_err(Into::into)
}

#[uniffi::export]
pub fn flush_logs() { baad_utils::flush_logs(); }

#[uniffi::export]
pub fn log(level: LogLevel, success: bool, message: &str) {
    core::log_message(level, success, message, None);
}

#[uniffi::export]
pub fn log_with_field(level: LogLevel, success: bool, message: &str, name: &str, value: &str) {
    let rendered = core::render_fields(&[(name, value)]);
    core::log_message(level, success, message, rendered.as_deref());
}

#[uniffi::export]
pub fn log_with_fields(
    level: LogLevel,
    success: bool,
    message: &str,
    fields: HashMap<String, String>
) {
    let mut pairs: Vec<(&str, &str)> =
        fields.iter().map(|(name, value)| (name.as_str(), value.as_str())).collect();
    pairs.sort_unstable_by_key(|(name, _)| *name);

    let rendered = core::render_fields(&pairs);
    core::log_message(level, success, message, rendered.as_deref());
}

#[uniffi::export]
pub fn set_app_name(name: &str) -> Result<(), FileError> {
    set_app_name_native(name).map_err(Into::into)
}

#[uniffi::export]
pub fn set_data_dir(path: &str) -> Result<(), FileError> {
    set_data_dir_native(PathBuf::from(path)).map_err(Into::into)
}

#[uniffi::export]
pub fn data_dir() -> Result<String, FileError> {
    data_dir_native().map(|path| path.to_string_lossy().into_owned()).map_err(Into::into)
}

#[uniffi::export]
pub fn get_data_path(filename: &str) -> Result<String, FileError> {
    get_data_path_native(filename)
        .map(|path| path.to_string_lossy().into_owned())
        .map_err(Into::into)
}

#[uniffi::export]
pub fn filename_or(path: &str) -> String { String::from(filename_or_native(path)) }

#[uniffi::export(async_runtime = "tokio")]
pub async fn load_file(path: &str) -> Result<Vec<u8>, FileError> {
    load_file_native(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn save_file(path: &str, content: Vec<u8>) -> Result<(), FileError> {
    save_file_native(PathBuf::from(path).as_path(), &content).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn create_parent_dir(path: &str) -> Result<(), FileError> {
    create_parent_dir_native(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_output_dir(path: Option<String>) -> Result<String, FileError> {
    let dir = get_output_dir_native(path.map(PathBuf::from).as_deref()).await?;
    Ok(dir.to_string_lossy().into_owned())
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn is_dir_empty(path: &str) -> Result<bool, FileError> {
    is_dir_empty_native(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn clear_all(path: &str) -> Result<(), FileError> {
    clear_all_native(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn json_load(path: &str) -> Result<String, JsonError> {
    core::json_load_string(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn json_save(path: &str, json: &str) -> Result<(), JsonError> {
    core::json_save_string(PathBuf::from(path).as_path(), json).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn fetch_version(url: &str) -> Result<String, NetworkError> {
    fetch_version_native(url).await.map_err(Into::into)
}

#[uniffi::export]
pub fn format_bytes(value: u64) -> String { HumanBytes(value).to_string() }

#[derive(Debug, Clone, uniffi::Record)]
pub struct TerminalSize {
    pub width: u64,
    pub height: u64
}

#[uniffi::export]
pub fn terminal_is_terminal() -> bool { terminal::is_terminal() }

#[uniffi::export]
pub fn terminal_size() -> Option<TerminalSize> {
    terminal::size().map(|(width, height)| TerminalSize {
        width: width as u64,
        height: height as u64
    })
}

include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
