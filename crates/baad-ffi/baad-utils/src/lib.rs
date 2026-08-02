mod api;

#[cfg(feature = "uniffi")]
pub use api::uniffi_api::*;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
