use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;
use simdutf8::basic::from_utf8;

use crate::error::JsonError;
use crate::file;

pub async fn load<T: DeserializeOwned>(path: &Path) -> Result<T, JsonError> {
    let bytes = file::load_file(path).await?;
    let json_data = from_utf8(&bytes).map_err(|_| JsonError::InvalidUtf8)?;
    Ok(serde_json::from_str(json_data)?)
}

pub async fn save<T: Serialize>(path: &Path, data: &T) -> Result<(), JsonError> {
    let json_data = serde_json::to_string_pretty(data)?;
    file::create_parent_dir(path).await?;
    file::save_file(path, json_data.as_bytes()).await?;
    Ok(())
}

pub async fn update<T, F>(path: &Path, updater: F) -> Result<(), JsonError>
where
    T: DeserializeOwned + Serialize + Default,
    F: FnOnce(&mut T)
{
    let mut data = load::<T>(path).await.unwrap_or_default();
    updater(&mut data);
    save(path, &data).await
}
