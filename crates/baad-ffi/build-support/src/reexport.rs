use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Item, Type};

use crate::config::{Config, Reexport};
use crate::syntax::{parse_file, source_dir};

pub(crate) fn render_reexport(config: &Config, reexport: &Reexport, file: &str) -> TokenStream {
    let items = reexport_items(config, reexport, &source_dir(reexport.dir).join(file));
    let core_path = source_dir(reexport.dir).join("core.rs");
    let core_module = core_path.exists().then(|| {
        let items = reexport_items(config, reexport, &core_path);
        quote! {
            mod core {
                #(#items)*
            }
        }
    });

    let module = format_ident!(
        "reexport_{}_{}",
        reexport.name,
        file.trim_end_matches(".rs").trim_end_matches("_api")
    );

    quote! {
        #[allow(unused_imports, dead_code, unsafe_op_in_unsafe_fn)]
        mod #module {
            use super::*;
            #core_module
            #(#items)*
        }
        #[allow(unused_imports)]
        pub use #module::*;
    }
}

fn reexport_items(config: &Config, reexport: &Reexport, path: &Path) -> Vec<TokenStream> {
    println!("cargo:rerun-if-changed={}", path.display());
    parse_file(path)
        .items
        .iter()
        .filter(|item| reexported(reexport, item))
        .map(|item| rename_symbols(config, reexport, quote! { #item }))
        .collect()
}

fn reexported(reexport: &Reexport, item: &Item) -> bool {
    match item {
        Item::Macro(_) => false,
        Item::Use(item) => match &item.tree {
            syn::UseTree::Path(path) => path.ident != "super" && path.ident != "crate",
            _ => true
        },
        Item::Fn(item) => !reexport_skipped(reexport, &item.sig.ident),
        Item::Struct(item) => !reexport_skipped(reexport, &item.ident),
        Item::Enum(item) => !reexport_skipped(reexport, &item.ident),
        Item::Const(item) => !reexport_skipped(reexport, &item.ident),
        Item::Impl(item) => {
            impl_target(item).map(|ident| !reexport_skipped(reexport, &ident)).unwrap_or(false)
        }
        _ => false
    }
}

fn impl_target(item: &syn::ItemImpl) -> Option<syn::Ident> {
    match item.self_ty.as_ref() {
        Type::Path(path) => path.path.segments.last().map(|segment| segment.ident.clone()),
        _ => None
    }
}

fn reexport_skipped(reexport: &Reexport, ident: &syn::Ident) -> bool {
    let name = ident.to_string();
    let bare_fn = name.strip_prefix(reexport.c_prefix).and_then(|rest| rest.strip_prefix('_'));
    let bare_type = name.strip_prefix(reexport.c_types_prefix);

    reexport
        .skip
        .iter()
        .any(|skip| name == *skip || bare_fn == Some(skip) || bare_type == Some(*skip))
}

fn rename_symbols(config: &Config, reexport: &Reexport, tokens: TokenStream) -> TokenStream {
    let text = tokens
        .to_string()
        .replace(&format!("{}_", reexport.c_prefix), &format!("{}_", config.c_prefix))
        .replace(reexport.c_types_prefix, config.c_types_prefix);
    text.parse().unwrap_or_else(|error| panic!("reexport rename ({}): {error}", reexport.name))
}
