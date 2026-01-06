use crate::error::JsonError;
use crate::file;

use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub async fn load<T: DeserializeOwned>(path: &Path) -> Result<T, JsonError> {
    let bytes = file::load_file(path).await?;
    let json_data = String::from_utf8(bytes).map_err(|_| JsonError::InvalidUtf8)?;
    Ok(serde_json::from_str(&json_data)?)
}

pub async fn save<T: Serialize>(path: &Path, data: &T) -> Result<(), JsonError> {
    let json_data = serde_json::to_string_pretty(data)?;
    file::create_parent_dir(path).await?;
    file::save_file(path, json_data.as_bytes()).await?;
    Ok(())
}

