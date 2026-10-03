use std::path::Path;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shadow_ffi_diplomat::constants::Constants;

fn custom(item: &syn::ItemConst, path: &syn::Path) -> Option<TokenStream> {
    if shadow_ffi::text(&item.ty) != "Style" {
        return None;
    }
    let ident = &item.ident;
    let method = format_ident!("{}", ident.to_string().to_snake_case());
    Some(
        quote!(pub fn #method() -> Box<crate::utils::ffi::BaadUtilsStyle> { Box::new(crate::utils::ffi::BaadUtilsStyle(#path::#ident)) })
    )
}

pub(crate) fn constants(root: &Path, family: &str) -> Option<syn::File> {
    if family == "baad" {
        return Some(shadow_ffi_diplomat::constants::flags(
            &root.join("crates/baad/src/download/resource.rs"),
            "baad_native::download",
            "BaadResourceCategory"
        ));
    }
    let (file, path, name) = match family {
        "baad-shared" => ("crates/baad-shared/src/consts.rs", "baad_shared", "BaadSharedConstants"),
        "baad-utils" => (
            "crates/baad-utils/src/formatter/styles.rs",
            "baad_utils::formatter::styles",
            "BaadUtilsConstants"
        ),
        _ => return None
    };
    let extra_methods = if family == "baad-shared" {
        quote!(
            pub const fn library_version() -> &'static str { env!("CARGO_PKG_VERSION") }
        )
    } else {
        TokenStream::new()
    };
    Some(shadow_ffi_diplomat::constants::constants(&Constants {
        source: &root.join(file),
        native_path: path,
        name,
        custom,
        extra_methods
    }))
}
