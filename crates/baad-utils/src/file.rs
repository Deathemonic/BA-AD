use crate::error::FileError;

use once_cell::sync::{Lazy, OnceCell};
use platform_dirs::AppDirs;
use std::env;
use std::path::{Path, PathBuf};
use tokio::fs;

static APP_NAME: OnceCell<String> = OnceCell::new();
static DATA_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn set_app_name(name: &str) -> Result<(), FileError> {
    APP_NAME
        .set(name.to_string())
        .map_err(|_| FileError::AppNameAlreadySet)
}

pub fn set_data_dir(path: PathBuf) -> Result<(), FileError> {
    DATA_DIR
        .set(path.clone())
        .map_err(|_| FileError::DataDirAlreadySet)
}

fn app_name() -> &'static str {
    APP_NAME.get().map(|s| s.as_str()).unwrap_or("baad")
}

static APP_DIRS: Lazy<Result<AppDirs, FileError>> =
    Lazy::new(|| AppDirs::new(Some(app_name()), true).ok_or(FileError::AppDirectoryCreationFailed));

pub fn data_dir() -> Result<&'static Path, FileError> {
    if let Some(path) = DATA_DIR.get() {
        return Ok(path.as_path());
    }

    APP_DIRS
        .as_ref()
        .map(|dirs| dirs.data_dir.as_path())
        .map_err(|_| FileError::AppDirectoryCreationFailed)
}

pub fn get_data_path(filename: &str) -> Result<PathBuf, FileError> {
    let data_dir = data_dir()?;
    Ok(data_dir.join(filename))
}

pub async fn load_file(path: &Path) -> Result<Vec<u8>, FileError> {
    Ok(fs::read(path).await?)
}

pub async fn save_file(path: &Path, content: &[u8]) -> Result<(), FileError> {
    fs::write(path, content).await?;
    Ok(())
}

pub async fn create_parent_dir(path: &Path) -> Result<(), FileError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(())
}

pub async fn get_output_dir(path: Option<PathBuf>) -> Result<PathBuf, FileError> {
    let output_dir = match path {
        Some(path) => path,
        None => env::current_dir()?.join("output"),
    };

    fs::create_dir_all(&output_dir).await?;
    Ok(output_dir)
}

pub async fn is_dir_empty(path: &Path) -> Result<bool, FileError> {
    Ok(!path.exists()
        || path
            .read_dir()
            .map_or(true, |mut entries| entries.next().is_none()))
}

pub async fn clear_all(dir: &Path) -> Result<(), FileError> {
    if dir.exists() {
        fs::remove_dir_all(dir).await?;
        fs::create_dir_all(dir).await?;
    }

    Ok(())
}
