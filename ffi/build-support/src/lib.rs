mod config;
mod constants;
mod mirror;
mod models;

use std::path::{Path, PathBuf};

use config::FAMILIES;
use constants::constants;
use mirror::transform;

pub fn build(family: &str) {
    let manifest =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let root = manifest.parent().expect("ffi directory").parent().expect("workspace root");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("build output"));
    generate(family, root, &out);
}

pub fn generate(family: &str, root: &Path, out: &Path) {
    let modules: Vec<Vec<shadow_ffi_diplomat::Module<'_>>> = FAMILIES
        .iter()
        .map(|(family, names)| {
            names
                .iter()
                .map(|name| shadow_ffi_diplomat::Module {
                    name,
                    public: !(*family == "baad-shared" && *name == "observer")
                })
                .collect()
        })
        .collect();
    let directories: Vec<_> =
        FAMILIES.iter().map(|(name, _)| PathBuf::from("ffi").join(name)).collect();
    let namespaces: Vec<_> =
        FAMILIES.iter().map(|(name, _)| format!("{}_ffi", name.replace('-', "_"))).collect();
    let umbrellas: Vec<_> =
        FAMILIES.iter().map(|(name, _)| format!("{}.h", name.replace('-', "_"))).collect();
    let families: Vec<_> = FAMILIES
        .iter()
        .enumerate()
        .map(|(i, (name, _))| shadow_ffi_diplomat::Family {
            name,
            namespace: &namespaces[i],
            directory: &directories[i],
            modules: &modules[i],
            umbrella: &umbrellas[i],
            runtime_in_umbrella: *name == "baad"
        })
        .collect();
    shadow_ffi_diplomat::generate(
        &shadow_ffi_diplomat::Config {
            root,
            families: &families,
            transform,
            additional: constants,
            unsafe_references_in_callbacks: true
        },
        family,
        out
    );
}
