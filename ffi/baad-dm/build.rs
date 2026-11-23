use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use shadow_ffi::{TypeMapping, read};
use shadow_ffi_diplomat::cargo::CrateConfig;
use shadow_ffi_diplomat::mirror;
use syn::{File, ItemStruct, Path as PathPath, parse_quote, parse_str, parse2};

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

fn main() -> Result<(), baad_ffi_build::Error> {
    baad_ffi_build::build(CrateConfig::new(transform))
}

fn record(
    item: &mut ItemStruct,
    native: &ItemStruct,
    path: &str,
    _: &mut TokenStream,
    conversions: &mut TokenStream
) {
    let mut scalar = native.clone();
    let nonzero: Vec<_> = native
        .fields
        .iter()
        .filter(
            |field| matches!(&field.ty, syn::Type::Path(path) if path.path.is_ident("NonZeroUsize"))
        )
        .map(|field| &field.ident)
        .collect();

    for field in &mut scalar.fields {
        if nonzero.contains(&&field.ident) {
            field.ty = parse_quote!(usize);
        }
    }

    mirror::scalar_record(item, &scalar);
    let ident = &item.ident;
    let path: PathPath = parse_str(path).expect("native record path");
    let names: Vec<_> = item.fields.iter().map(|f| &f.ident).collect();

    if native.ident == "ZipFileInfo" {
        conversions.extend(quote! {

            impl From<#path> for ffi::#ident {
                fn from(value: #path) -> Self { Self { #(#names: value.#names,)* } }
            }

            impl From<ffi::#ident> for #path {
                fn from(value: ffi::#ident) -> Self { Self { #(#names: value.#names,)* } }
            }
        });
    } else {
        let from_fields = names.iter().map(|name| {
            if nonzero.contains(name) {
                quote!(#name: value.#name.get())
            } else {
                quote!(#name: value.#name)
            }
        });
        let native_fields = names.iter().map(|name| {
            if nonzero.contains(name) {
                quote!(#name: std::num::NonZeroUsize::new(self.#name).ok_or(baad_dm::Error::ZeroConcurrency)?)
            } else {
                quote!(#name: self.#name)
            }
        });

        conversions.extend(quote! {
            impl From<#path<'_>> for ffi::#ident {
                fn from(value: #path<'_>) -> Self { Self { #(#from_fields,)* } }
            }

            impl ffi::#ident {
                pub fn native<'a>(&self, directory: &'a std::path::Path, proxy: Option<reqwest::Proxy>) -> Result<#path<'a>, baad_dm::Error> {
                    Ok(#path {
                        #(#native_fields,)*
                        ..#path::builder().directory(directory).maybe_proxy(proxy).build()
                    })
                }
            }
        });
    }
}

fn transform(root: &Path, module: &str) -> File {
    let source = read(&root.join(format!("ffi/baad-dm/src/{module}.rs")));
    let (mut source, conversions) = mirror::transform(root, source, &mirror::Config {
        enums: ENUMS,
        records: RECORDS,
        record,
        models: None
    });
    source.items.extend(parse2::<File>(conversions).expect("generated conversions").items);

    source
}
