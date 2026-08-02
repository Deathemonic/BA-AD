use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerConfigError {
    #[error("Teen build is only available for Global server")]
    TeenNotAvailable,

    #[error("Unsupported platform and build type combination")]
    UnsupportedCombination,

    #[error("Server does not support Windows platform. Use Android or iOS instead")]
    WindowsNotSupported
}
