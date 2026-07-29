use std::path::Path;

use baad_utils::JsonError;

pub(crate) async fn json_load_string(path: &Path) -> Result<String, JsonError> {
    let bytes = baad_utils::file::load_file(path).await?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(JsonError::SerdeJson)?;
    serde_json::to_string(&value).map_err(JsonError::SerdeJson)
}

pub(crate) async fn json_save_string(path: &Path, json: &str) -> Result<(), JsonError> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(JsonError::SerdeJson)?;
    let pretty = serde_json::to_string_pretty(&value).map_err(JsonError::SerdeJson)?;
    baad_utils::file::create_parent_dir(path).await?;
    baad_utils::file::save_file(path, pretty.as_bytes()).await?;
    Ok(())
}
