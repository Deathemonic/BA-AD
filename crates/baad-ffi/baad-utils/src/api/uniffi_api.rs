//! UniFFI bindings for `baad-utils`. Excludes the process-exiting runners,
//! tracing plumbing and generic progress types; error enums and
//! `LoggingConfig` are generated into `shadow.rs`.

use std::collections::HashMap;
use std::path::PathBuf;

use super::core;
pub use super::core::LogLevel;

#[uniffi::export]
pub fn init_logging(config: LoggingConfig) -> Result<(), ConfigError> {
    baad_utils::config::init_logging(config.into()).map_err(Into::into)
}

#[uniffi::export]
pub fn flush_logs() { baad_utils::flush_logs(); }

#[uniffi::export]
pub fn log(level: LogLevel, success: bool, message: &str) {
    core::log_message(level, success, message, None);
}

#[uniffi::export]
pub fn log_with_field(level: LogLevel, success: bool, message: &str, name: &str, value: &str) {
    core::log_message(level, success, message, Some(&core::join_fields(&[(name, value)])));
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

    let rendered = match pairs.is_empty() {
        true => None,
        false => Some(core::join_fields(&pairs))
    };
    core::log_message(level, success, message, rendered.as_deref());
}

#[uniffi::export]
pub fn set_app_name(name: &str) -> Result<(), FileError> {
    baad_utils::file::set_app_name(name).map_err(Into::into)
}

#[uniffi::export]
pub fn set_data_dir(path: &str) -> Result<(), FileError> {
    baad_utils::file::set_data_dir(PathBuf::from(path)).map_err(Into::into)
}

#[uniffi::export]
pub fn data_dir() -> Result<String, FileError> {
    baad_utils::file::data_dir().map(|path| path.to_string_lossy().into_owned()).map_err(Into::into)
}

#[uniffi::export]
pub fn get_data_path(filename: &str) -> Result<String, FileError> {
    baad_utils::file::get_data_path(filename)
        .map(|path| path.to_string_lossy().into_owned())
        .map_err(Into::into)
}

#[uniffi::export]
pub fn filename_or(path: &str) -> String { String::from(baad_utils::file::filename_or(path)) }

#[uniffi::export(async_runtime = "tokio")]
pub async fn load_file(path: &str) -> Result<Vec<u8>, FileError> {
    baad_utils::file::load_file(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn save_file(path: &str, content: Vec<u8>) -> Result<(), FileError> {
    baad_utils::file::save_file(PathBuf::from(path).as_path(), &content).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn create_parent_dir(path: &str) -> Result<(), FileError> {
    baad_utils::file::create_parent_dir(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_output_dir(path: Option<String>) -> Result<String, FileError> {
    let dir = baad_utils::file::get_output_dir(path.map(PathBuf::from).as_deref()).await?;
    Ok(dir.to_string_lossy().into_owned())
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn is_dir_empty(path: &str) -> Result<bool, FileError> {
    baad_utils::file::is_dir_empty(PathBuf::from(path).as_path()).await.map_err(Into::into)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn clear_all(path: &str) -> Result<(), FileError> {
    baad_utils::file::clear_all(PathBuf::from(path).as_path()).await.map_err(Into::into)
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
    baad_utils::network::fetch_version(url).await.map_err(Into::into)
}

#[uniffi::export]
pub fn format_bytes(value: u64) -> String { baad_utils::formatter::HumanBytes(value).to_string() }

#[derive(Debug, Clone, uniffi::Record)]
pub struct TerminalSize {
    pub width: u64,
    pub height: u64
}

#[uniffi::export]
pub fn terminal_is_terminal() -> bool { baad_utils::progress::terminal::is_terminal() }

#[uniffi::export]
pub fn terminal_size() -> Option<TerminalSize> {
    baad_utils::progress::terminal::size().map(|(width, height)| TerminalSize {
        width: width as u64,
        height: height as u64
    })
}

include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
