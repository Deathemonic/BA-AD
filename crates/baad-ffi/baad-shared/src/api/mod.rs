#[cfg(feature = "c-api")]
pub mod c_api;

#[cfg(any(feature = "c-api", feature = "uniffi"))]
pub(crate) mod observer;

#[cfg(feature = "uniffi")]
pub mod uniffi_api;
