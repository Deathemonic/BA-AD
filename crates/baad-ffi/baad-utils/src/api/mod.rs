#[cfg(feature = "c-api")]
pub mod c_api;

#[cfg(any(feature = "c-api", feature = "uniffi"))]
pub mod core;

#[cfg(feature = "uniffi")]
pub mod uniffi_api;
