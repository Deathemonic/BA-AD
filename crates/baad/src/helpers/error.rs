use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerConfigError {
    #[error("Teen build is only available for Global server")]
    TeenNotAvailable,

    #[error("Unsupported platform and build type combination")]
    UnsupportedCombination,
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Unable to set proxy")]
    Proxy
}

#[derive(Error, Debug)]
pub enum JsonError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    File(#[from] baad_core::error::FileError),

    #[error("Failed to convert file content to UTF-8")]
    InvalidUtf8,

    #[error("Failed to get file path")]
    PathError,
}

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Invalid regex pattern: {pattern}")]
    InvalidRegex { pattern: Box<str> },

    #[error("Invalid glob pattern: {pattern}")]
    InvalidGlob { pattern: Box<str> },
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    File(#[from] baad_core::error::FileError),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Catalog(#[from] CatalogError),

    #[error("Retry count cannot be zero")]
    RetryCountZero,

    #[error("Download limit cannot be zero")]
    DownloadLimitZero,

    #[error("No files matched filter criteria")]
    NoFilesMatched,
}

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    File(#[from] baad_core::error::FileError),

    #[error(transparent)]
    Catalog(#[from] bacy::error::CatalogError),

    #[error(transparent)]
    Apk(#[from] ApkError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Catalog URL is empty for {region} region")]
    EmptyCatalogUrl { region: Box<str> },

    #[error("Failed to deserialize catalog data")]
    DeserializationFailed,
}

#[derive(Error, Debug)]
pub enum ApkError {
    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    File(#[from] baad_core::error::FileError),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Glob(#[from] glob::PatternError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to extract version from server response")]
    VersionExtractionFailed,

    #[error("Failed to get download URL")]
    DownloadUrlExtractionFailed,

    #[error("Failed to get APK info")]
    ApkInfoFailed,
}
