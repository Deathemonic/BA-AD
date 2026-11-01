use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shadow_ffi::{TypeMapping, native_enum, primitive, read, text};
use shadow_ffi_diplomat::cargo::CrateConfig;
use shadow_ffi_diplomat::{constants, mirror, models};
use syn::{Field, Fields, File, Item, ItemStruct, parse2};

const ENUMS: &[TypeMapping<'static>] = &[
    TypeMapping {
        shadow: "BaadSharedPlatform",
        source: "crates/baad-shared/src/platform.rs",
        name: "Platform",
        native_path: "baad_shared::Platform"
    },
    TypeMapping {
        shadow: "BaadSharedBuildType",
        source: "crates/baad-shared/src/platform.rs",
        name: "BuildType",
        native_path: "baad_shared::BuildType"
    },
    TypeMapping {
        shadow: "BaadSharedChinaMediaType",
        source: "crates/baad-shared/src/types.rs",
        name: "ChinaMediaType",
        native_path: "baad_shared::ChinaMediaType"
    },
    TypeMapping {
        shadow: "BaadSharedProgressUnit",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressUnit",
        native_path: "baad_shared::ProgressUnit"
    },
    TypeMapping {
        shadow: "BaadSharedProgressStatusKind",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressStatus",
        native_path: "baad_shared::ProgressStatus"
    },
    TypeMapping {
        shadow: "BaadSharedProgressEventKind",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressEvent",
        native_path: "baad_shared::ProgressEvent"
    },
    TypeMapping {
        shadow: "BaadSharedHashKind",
        source: "crates/baad-shared/src/types.rs",
        name: "HashValue",
        native_path: "baad_shared::HashValue"
    }
];
const RECORDS: &[TypeMapping<'static>] = &[];

fn main() -> Result<(), baad_ffi_build::Error> {
    baad_ffi_build::build(CrateConfig {
        additional: Some(additional),
        private_modules: &["observer"],
        ..CrateConfig::new(transform)
    })
}

fn custom_getter(field: &Field) -> Option<TokenStream> {
    let name = field.ident.as_ref().expect("named native field");
    let value = quote!(self.0.#name);

    match text(&field.ty).as_str() {
        "ChinaMediaType" => Some(quote! {
            pub fn #name(&self) -> crate::shared::ffi::BaadSharedChinaMediaType {
                #value.into()
            }
        }),
        "HashValue" => {
            let kind = format_ident!("{name}_kind");

            Some(quote! {
                pub fn #kind(&self) -> BaadSharedHashKind { (&#value).into() }

                pub fn #name(&self, output: &mut DiplomatWrite) {
                    crate::models::write_hash(&#value, output);
                }
            })
        }
        _ => None
    }
}

fn generate_models(root: &Path, items: &mut Vec<Item>, outside: &mut TokenStream) {
    models::generate_models(
        &models::Models {
            source: &root.join("crates/baad-shared/src/types.rs"),
            native_path: "baad_shared",
            prefix: "BaadShared",
            renames: &[("GlobalCatalog", "GlobalCatalogData")],
            strings: "BaadSharedStrings",
            custom_getter
        },
        items,
        outside
    );

    models::rename_rust_method(items, "hash", "write_hash");
}

const fn record(
    _: &mut ItemStruct,
    _: &ItemStruct,
    _: &str,
    _: &mut TokenStream,
    _: &mut TokenStream
) {
}

fn transform(root: &Path, module: &str) -> File {
    let source = read(&root.join(format!("ffi/baad-shared/src/{module}.rs")));
    let model_hook =
        if module == "models" { Some(generate_models as mirror::ModelHook) } else { None };
    let (mut source, mut conversions) = mirror::transform(root, source, &mirror::Config {
        enums: ENUMS,
        records: RECORDS,
        record,
        models: model_hook
    });

    if module == "models" {
        conversions.extend(hash_conversion(root));
    }

    if module == "progress" {
        conversions.extend(status_reason(root));
    }

    source.items.extend(parse2::<File>(conversions).expect("generated conversions").items);

    source
}

fn additional(root: &Path) -> File {
    constants::constants(&constants::Constants {
        source: &root.join("crates/baad-shared/src/consts.rs"),
        native_path: "baad_shared",
        name: "BaadSharedConstants",
        custom: |_, _| None,
        extra_methods: quote!(
            pub const fn library_version() -> &'static str { env!("CARGO_PKG_VERSION") }
        )
    })
}

fn hash_conversion(root: &Path) -> TokenStream {
    let native = native_enum(root, "crates/baad-shared/src/types.rs", "HashValue");
    let arms = native.variants.iter().map(|variant| {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => quote!(
                ffi::BaadSharedHashKind::#name => Ok(baad_shared::HashValue::#name),
            ),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0].ty;

                if text(ty) == "String" {
                    quote!(
                        ffi::BaadSharedHashKind::#name =>
                            Ok(baad_shared::HashValue::#name(value.into())),
                    )
                } else if primitive(ty) {
                    quote!(
                        ffi::BaadSharedHashKind::#name => value.parse::<#ty>()
                            .map(baad_shared::HashValue::#name)
                            .map_err(crate::error::error),
                    )
                } else {
                    panic!("unsupported native hash payload {}: {}", name, text(ty));
                }
            }
            _ => panic!("unsupported native hash variant {name}")
        }
    });

    quote! {
        pub fn hash(
            kind: ffi::BaadSharedHashKind,
            value: &str
        ) -> Result<baad_shared::HashValue, Box<crate::error::ffi::BaadError>> {
            match kind { #(#arms)* }
        }
    }
}

fn status_reason(root: &Path) -> TokenStream {
    let native = native_enum(root, "crates/baad-shared/src/observer.rs", "ProgressStatus");
    let mut empty = Vec::new();
    let mut reasons = Vec::new();

    for variant in &native.variants {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => empty.push(quote!(baad_shared::ProgressStatus::#name)),
            Fields::Unnamed(fields)
                if fields.unnamed.len() == 1 && {
                    let ty = text(&fields.unnamed[0].ty);
                    ty == "String"
                        || ty.starts_with("Cow<")
                        || ty.starts_with("Arc<")
                        || ty == "Box<str>"
                } =>
            {
                reasons.push(quote!(baad_shared::ProgressStatus::#name(reason)));
            }
            Fields::Unnamed(_) => empty.push(quote!(baad_shared::ProgressStatus::#name(..))),
            Fields::Named(_) => empty.push(quote!(baad_shared::ProgressStatus::#name { .. }))
        }
    }

    quote! {
        pub fn status_reason(status: &baad_shared::ProgressStatus) -> &str {
            match status {
                #(#reasons)|* => reason.as_ref(),
                #(#empty)|* => ""
            }
        }
    }
}
