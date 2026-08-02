use std::path::{Path, PathBuf};
use std::sync::{LazyLock, OnceLock};

use platform_dirs::AppDirs;

use crate::error::FileError;

static APP_NAME: OnceLock<String> = OnceLock::new();
static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_app_name(name: &str) -> Result<(), FileError> {
    APP_NAME.set(name.into()).map_err(|_| FileError::AppNameAlreadySet)
}

pub fn set_data_dir(path: PathBuf) -> Result<(), FileError> {
    DATA_DIR.set(path).map_err(|_| FileError::DataDirAlreadySet)
}

fn app_name() -> &'static str { APP_NAME.get().map(|s| s.as_str()).unwrap_or("baad") }

static APP_DIRS: LazyLock<Result<AppDirs, FileError>> = LazyLock::new(|| {
    AppDirs::new(Some(app_name()), true).ok_or(FileError::AppDirectoryCreationFailed)
});

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
