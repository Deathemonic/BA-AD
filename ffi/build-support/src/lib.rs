mod error;

use std::env;
use std::path::Path;

pub use error::Error;
use shadow_ffi_diplomat::cargo::{self, CrateConfig};

pub fn build(mut config: CrateConfig<'_>) -> Result<(), Error> {
    let manifest = env::var("CARGO_MANIFEST_DIR")?;
    config.root = Path::new(&manifest).join("../..").canonicalize()?;
    config.unsafe_references_in_callbacks = true;
    cargo::build(&config)?;

    Ok(())
}
