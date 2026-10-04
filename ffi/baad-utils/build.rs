use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use shadow_ffi::{TypeMapping, read, text};
use shadow_ffi_diplomat::{Family, Module, constants, mirror};
use syn::{Fields, File, ItemConst, ItemStruct, Path as PathPath, parse_str, parse2};
const ENUMS: &[TypeMapping<'static>] = &[];
const RECORDS: &[TypeMapping<'static>] = &[
    TypeMapping {
        shadow: "BaadUtilsAlignedLine",
        source: "crates/baad-utils/src/formatter/line.rs",
        name: "AlignedLine",
        native_path: "baad_utils::formatter::AlignedLine"
    },
    TypeMapping {
        shadow: "BaadUtilsLoggingConfig",
        source: "crates/baad-utils/src/logging/config.rs",
        name: "LoggingConfig",
        native_path: "baad_utils::config::LoggingConfig"
    }
];

pub fn config() -> baad_ffi_build::Api {
    baad_ffi_build::Api {
        family: Family {
            name: "baad-utils",
            namespace: "baad_utils_ffi",
            directory: Path::new("ffi/baad-utils"),
            umbrella: "baad_utils.h",
            runtime_in_umbrella: false,
            modules: &[
                Module {
                    name: "adapter",
                    public: true
                },
                Module {
                    name: "utils",
                    public: true
                }
            ]
        },
        transform,
        additional: Some(additional)
    }
}

fn record(
    item: &mut ItemStruct,
    native: &ItemStruct,
    path: &str,
    _: &mut TokenStream,
    conversions: &mut TokenStream
) {
    if native.ident == "AlignedLine" {
        let ident = &item.ident;
        item.generics = native.generics.clone();
        let fields: Vec<_> = native
            .fields
            .iter()
            .cloned()
            .map(|mut field| {
                field.attrs.clear();
                if text(&field.ty) == "&'aLevel" {
                    field.ty = syn::parse_quote!(BaadUtilsLogLevel);
                } else if text(&field.ty) == "&'astr" {
                    field.ty = syn::parse_quote!(DiplomatUtf8StrSlice<'a>);
                }
                field
            })
            .collect();
        item.fields = Fields::Named(syn::parse_quote!({ # (# fields,) * }));
        let assignments = native.fields.iter().map(|field| {
            let field_name = &field.ident;
            if text(&field.ty) == "&'aLevel" {
                quote!(# field_name : value.# field_name.native_ref(),)
            } else if text(&field.ty) == "&'astr" {
                quote!(# field_name : value.# field_name.into(),)
            } else {
                quote!(# field_name : value.# field_name,)
            }
        });
        let path: PathPath = parse_str(path).expect("aligned record path");
        conversions.extend(quote! {

            impl <'a > From < ffi::# ident <'a >> for # path <'a > { fn
            from(value : ffi::# ident <'a >) -> Self { Self { # (# assignments) *
            } } }
        });
        return;
    }
    baad_ffi_build::scalar_record(item, native);
    let ident = &item.ident;
    let path: PathPath = parse_str(path).expect("native record path");
    let names: Vec<_> = item.fields.iter().map(|f| &f.ident).collect();
    conversions.extend(quote! {

        impl From <# path > for ffi::# ident { fn from(value : # path) -> Self {
        Self { # (# names : value.# names,) * } } } impl From < ffi::# ident >
        for # path { fn from(value : ffi::# ident) -> Self { Self { # (# names :
        value.# names,) * } } }
    });
}

fn transform(root: &Path, module: &str) -> File {
    let source = read(&root.join(format!("ffi/baad-utils/src/{module}.rs")));
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

fn constant(item: &ItemConst, path: &PathPath) -> Option<TokenStream> {
    use heck::ToSnakeCase;
    use quote::format_ident;
    if text(&item.ty) != "Style" {
        return None;
    }
    let ident = &item.ident;
    let method = format_ident!("{}", ident.to_string().to_snake_case());
    Some(quote!(

        pub fn # method() -> Box < crate ::utils::ffi::BaadUtilsStyle > {
        Box::new(crate ::utils::ffi::BaadUtilsStyle(# path::# ident)) }
    ))
}

fn additional(root: &Path) -> File {
    constants::constants(&constants::Constants {
        source: &root.join("crates/baad-utils/src/formatter/styles.rs"),
        native_path: "baad_utils::formatter::styles",
        name: "BaadUtilsConstants",
        custom: constant,
        extra_methods: TokenStream::new()
    })
}

fn main() { baad_ffi_build::build("baad-utils"); }
