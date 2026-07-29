use std::path::{Path, PathBuf};
use std::{env, fs};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum, ItemStruct, Type};

use crate::config::{Config, Source};

pub(crate) fn parse_file(path: &Path) -> syn::File {
    let content =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    syn::parse_file(&content).unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

pub(crate) fn source_dir(relative: &str) -> PathBuf {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest.join(relative)
}

pub(crate) fn write_rust(path: &Path, tokens: TokenStream) {
    let file: syn::File = syn::parse2(tokens)
        .unwrap_or_else(|error| panic!("generated tokens parse ({}): {error}", path.display()));
    fs::write(path, prettyplease::unparse(&file))
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

pub(crate) fn normalized(ty: &Type) -> String { quote!(#ty).to_string().replace(' ', "") }

pub(crate) fn rewrite_struct(config: &Config, item: &ItemStruct) -> ItemStruct {
    let mut cloned = item.clone();
    for field in &mut cloned.fields {
        field.ty = rewrite_type(config, &field.ty);
    }
    cloned
}

pub(crate) fn rewrite_enum(config: &Config, item: &ItemEnum) -> ItemEnum {
    let mut cloned = item.clone();
    for variant in &mut cloned.variants {
        for field in &mut variant.fields {
            field.ty = rewrite_type(config, &field.ty);
        }
    }
    cloned
}

fn rewrite_type(config: &Config, ty: &Type) -> Type {
    let mut text = normalized(ty);
    let mut changed = false;

    for custom in config.custom_types {
        if !custom.pattern.is_empty() && text.contains(custom.pattern) {
            text = text.replace(custom.pattern, custom.alias);
            changed = true;
        }
    }

    if !changed {
        return ty.clone();
    }
    syn::parse_str(&text).unwrap_or_else(|error| panic!("rewrite type {text}: {error}"))
}

pub(crate) fn strip_struct(item: &ItemStruct) -> ItemStruct {
    let mut cloned = item.clone();
    cloned.attrs.clear();
    if let Fields::Named(fields) = &mut cloned.fields {
        for field in &mut fields.named {
            field.attrs.clear();
        }
    }
    cloned
}

pub(crate) fn strip_enum(item: &ItemEnum) -> ItemEnum {
    let mut cloned = item.clone();
    cloned.attrs.retain(|attr| attr.path().is_ident("repr"));
    for variant in &mut cloned.variants {
        variant.attrs.clear();
        for field in &mut variant.fields {
            field.attrs.clear();
        }
    }
    cloned
}

pub(crate) fn enum_is_fieldless(item: &ItemEnum) -> bool {
    item.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit))
}

pub(crate) fn enum_is_ffi_safe(item: &ItemEnum) -> bool {
    item.variants
        .iter()
        .all(|variant| variant.fields.iter().all(|field| type_is_ffi_safe(&field.ty)))
}

pub(crate) fn struct_is_ffi_safe(item: &ItemStruct) -> bool {
    item.fields.iter().all(|field| type_is_ffi_safe(&field.ty))
}

pub(crate) fn type_is_ffi_safe(ty: &Type) -> bool {
    const HOSTILE: &[&str] =
        &["&", "Arc<", "Box<", "dyn", "Url", "StatusCode", "HeaderMap", "Proxy", "Path", "Error"];
    let text = normalized(ty);
    HOSTILE.iter().all(|marker| !text.contains(marker))
}

pub(crate) fn fields_are_named(fields: &Fields) -> bool { matches!(fields, Fields::Named(_)) }

pub(crate) fn is_public(vis: &syn::Visibility) -> bool { matches!(vis, syn::Visibility::Public(_)) }

pub(crate) fn skipped(source: &Source, ident: &syn::Ident) -> bool {
    source.skip_types.contains(&ident.to_string().as_str())
}

pub(crate) fn rename(source: &Source, ident: &mut syn::Ident) {
    let name = ident.to_string();
    if let Some(entry) = source.renames.iter().find(|entry| entry.source == name) {
        *ident = format_ident!("{}", entry.target);
    }
}

pub(crate) fn crate_path(text: &str) -> syn::Path {
    syn::parse_str(text).unwrap_or_else(|error| panic!("crate path {text}: {error}"))
}
