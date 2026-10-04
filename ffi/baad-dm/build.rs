use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use shadow_ffi::{TypeMapping, read};
use shadow_ffi_diplomat::{Family, Module, mirror};
use syn::{File, ItemStruct, Path as PathPath, parse_str, parse2};
const ENUMS: &[TypeMapping<'static>] = &[TypeMapping {
    shadow: "BaadDmHashType",
    source: "crates/baad-dm/src/download/hash.rs",
    name: "HashType",
    native_path: "baad_dm::HashType"
}];
const RECORDS: &[TypeMapping<'static>] = &[
    TypeMapping {
        shadow: "BaadDmZipFileInfo",
        source: "crates/baad-dm/src/zip/types.rs",
        name: "ZipFileInfo",
        native_path: "baad_dm::ZipFileInfo"
    },
    TypeMapping {
        shadow: "BaadDmDownloaderConfig",
        source: "crates/baad-dm/src/downloader/config.rs",
        name: "DownloaderConfig",
        native_path: "baad_dm::DownloaderConfig"
    }
];

pub fn config() -> baad_ffi_build::Api {
    baad_ffi_build::Api {
        family: Family {
            name: "baad-dm",
            namespace: "baad_dm_ffi",
            directory: Path::new("ffi/baad-dm"),
            umbrella: "baad_dm.h",
            runtime_in_umbrella: false,
            modules: &[
                Module {
                    name: "adapter",
                    public: true
                },
                Module { name: "dm", public: true }
            ]
        },
        transform,
        additional: None
    }
}

fn record(
    item: &mut ItemStruct,
    native: &ItemStruct,
    path: &str,
    _: &mut TokenStream,
    conversions: &mut TokenStream
) {
    baad_ffi_build::scalar_record(item, native);
    let ident = &item.ident;
    let path: PathPath = parse_str(path).expect("native record path");
    let names: Vec<_> = item.fields.iter().map(|f| &f.ident).collect();
    if native.ident == "ZipFileInfo" {
        conversions.extend(quote! {

            impl From <# path > for ffi::# ident { fn from(value : # path) ->
            Self { Self { # (# names : value.# names,) * } } } impl From < ffi::#
            ident > for # path { fn from(value : ffi::# ident) -> Self { Self { #
            (# names : value.# names,) * } } }
        });
    } else {
        conversions.extend(quote! {

            impl From <# path <'_ >> for ffi::# ident { fn from(value : # path
            <'_ >) -> Self { Self { # (# names : value.# names,) * } } } impl
            ffi::# ident { pub fn native <'a > (& self, directory : &'a
            std::path::Path, proxy : Option < reqwest::Proxy >) -> # path <'a > {
            # path::builder().directory(directory).maybe_proxy(proxy) # (.#
            names(self.# names)) * .build() } }
        });
    }
}

fn transform(root: &Path, module: &str) -> File {
    let source = read(&root.join(format!("ffi/baad-dm/src/{module}.rs")));
    let model_hook = None;
    let (mut source, conversions) = mirror::transform(root, source, &mirror::Config {
        enums: ENUMS,
        records: RECORDS,
        record,
        models: model_hook
    });
    source.items.extend(parse2::<File>(conversions).expect("generated conversions").items);
    source
}

fn main() { baad_ffi_build::build("baad-dm"); }
