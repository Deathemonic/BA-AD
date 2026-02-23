use std::env;
use std::path::{Path, PathBuf};

use tokio::fs;

use crate::error::FileError;

pub async fn load_file(path: &Path) -> Result<Vec<u8>, FileError> { Ok(fs::read(path).await?) }

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

pub async fn get_output_dir(path: Option<&Path>) -> Result<PathBuf, FileError> {
    let output_dir =
        path.map_or_else(|| env::current_dir().map(|d| d.join("output")), |p| Ok(p.to_path_buf()))?;
    fs::create_dir_all(&output_dir).await?;
    Ok(output_dir)
}

pub async fn is_dir_empty(path: &Path) -> Result<bool, FileError> {
    Ok(!path.exists() || path.read_dir().map_or(true, |mut entries| entries.next().is_none()))
}

pub async fn clear_all(dir: &Path) -> Result<(), FileError> {
    if dir.exists() {
        fs::remove_dir_all(dir).await?;
        fs::create_dir_all(dir).await?;
    }

    Ok(())
}
