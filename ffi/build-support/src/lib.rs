extern crate self as baad_ffi_build;
use std::env;
use std::path::{Path, PathBuf};

use shadow_ffi::{primitive, shadow_record};
use shadow_ffi_diplomat::{Config, Family, generate as generate_headers};
use syn::{Fields, File, ItemStruct, parse_quote};

#[allow(dead_code)]
#[path = "../../baad/build.rs"]
mod baad;

#[allow(dead_code)]
#[path = "../../baad-dm/build.rs"]
mod dm;

#[allow(dead_code)]
#[path = "../../baad-shared/build.rs"]
mod shared;

#[allow(dead_code)]
#[path = "../../baad-utils/build.rs"]
mod utils;

pub struct Api {
    pub family: Family<'static>,
    pub transform: fn(&Path, &str) -> File,
    pub additional: Option<fn(&Path) -> File>
}

fn apis() -> [Api; 4] { [shared::config(), utils::config(), dm::config(), baad::config()] }

fn transform(root: &Path, family: &str, module: &str) -> File {
    let api = apis().into_iter().find(|api| api.family.name == family).expect("API family");
    (api.transform)(root, module)
}

fn additional(root: &Path, family: &str) -> Option<File> {
    let api = apis().into_iter().find(|api| api.family.name == family).expect("API family");
    api.additional.map(|generate| generate(root))
}

pub fn scalar_record(item: &mut ItemStruct, native: &ItemStruct) {
    let mut record = shadow_record(native, &item.ident.to_string(), |field| primitive(&field.ty));
    for field in &mut record.fields {
        field.vis = parse_quote!(pub);
    }
    let Fields::Named(_) = &record.fields else { panic!("named record required") };
    item.fields = record.fields;
}

pub fn build(family: &str) {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let root = manifest.parent().expect("ffi directory").parent().expect("workspace root");
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("build output"));
    generate(family, root, &out);
}

pub fn generate(family: &str, root: &Path, out: &Path) {
    let families: Vec<_> = apis().into_iter().map(|api| api.family).collect();
    generate_headers(
        &Config {
            root,
            families: &families,
            transform,
            additional,
            unsafe_references_in_callbacks: true
        },
        family,
        out
    );
}
