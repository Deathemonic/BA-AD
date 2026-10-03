use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shadow_ffi_diplomat::models::{Models, generate_models as generate};
use syn::Item;

fn custom_getter(field: &syn::Field) -> Option<TokenStream> {
    let name = field.ident.as_ref().expect("named native field");
    let value = quote!(self.0.#name);
    match shadow_ffi::text(&field.ty).as_str() {
        "ChinaMediaType" => Some(
            quote!(pub fn #name(&self) -> crate::shared::ffi::BaadSharedChinaMediaType { #value.into() })
        ),
        "HashValue" => {
            let kind = format_ident!("{name}_kind");
            Some(quote! {
                pub fn #kind(&self) -> BaadSharedHashKind { (&#value).into() }
                #[allow(clippy::should_implement_trait)]
                pub fn #name(&self, output: &mut DiplomatWrite) { crate::models::write_hash(&#value, output); }
            })
        }
        _ => None
    }
}

pub(crate) fn generate_models(root: &Path, items: &mut Vec<Item>, outside: &mut TokenStream) {
    generate(
        &Models {
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
}
