use std::io;

use baad_utils::{FileError, JsonError, NetworkError};
use bacy::error::TableEncryptionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Invalid regex pattern: {pattern}")]
    InvalidRegex { pattern: Box<str> },

    #[error("Invalid glob pattern: {pattern}")]
    InvalidGlob { pattern: Box<str> }
}

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    File(#[from] FileError),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    MemoryPack(#[from] memorypack::MemoryPackError),

    #[error(transparent)]
    Download(#[from] baad_dm::Error),

    #[error(transparent)]
    TableEncryption(#[from] TableEncryptionError),

    #[error(transparent)]
    ServerConfig(#[from] baad_shared::ServerConfigError),

    #[error("Catalog URL is empty for {region} region")]
    EmptyCatalogUrl { region: Box<str> },

    #[error("Failed to deserialize catalog data")]
    DeserializationFailed
}
