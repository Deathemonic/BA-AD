use std::path::Path;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Item;

use crate::source::{primitive, read, text};

pub(crate) fn constants(root: &Path, family: &str) -> Option<syn::File> {
    if family == "baad" {
        return Some(resource_flags(root));
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
    let name = format_ident!("{name}");
    let path: syn::Path = syn::parse_str(path).expect("constant native path");
    let mut methods = TokenStream::new();
    for item in read(&root.join(file)).items {
        let Item::Const(item) = item else { continue };
        if !matches!(item.vis, syn::Visibility::Public(_)) {
            continue;
        }
        let ident = &item.ident;
        let method = format_ident!("{}", ident.to_string().to_snake_case());
        match text(&item.ty).as_str() {
            "&str" => methods.extend(quote!(pub const fn #method() -> &'static str { #path::#ident })),
            "&[u8]" => methods.extend(quote!(pub const fn #method() -> &'static [u8] { #path::#ident })),
            "&[&str]" => {
                let len = format_ident!("{method}_len");
                let at = format_ident!("{method}_at");
                methods.extend(quote! {
                    pub const fn #len() -> usize { #path::#ident.len() }
                    pub fn #at(index: usize) -> Option<&'static str> { #path::#ident.get(index).copied() }
                });
            }
            "Style" => methods.extend(quote!(pub fn #method() -> Box<crate::utils::ffi::BaadUtilsStyle> { Box::new(crate::utils::ffi::BaadUtilsStyle(#path::#ident)) })),
            _ if primitive(&item.ty) => {
                let ty = &item.ty;
                methods.extend(quote!(pub const fn #method() -> #ty { #path::#ident }));
            }
            _ => panic!("unsupported public constant {}: {}; add a boundary adapter", item.ident, text(&item.ty)),
        }
    }
    if family == "baad-shared" {
        methods.extend(quote!(
            pub const fn library_version() -> &'static str { env!("CARGO_PKG_VERSION") }
        ));
    }
    Some(
        syn::parse2(quote! {
            #[diplomat::bridge]
            pub mod ffi {
                #[diplomat::opaque]
                pub struct #name;
                impl #name { #methods }
            }
        })
        .expect("generated constants")
    )
}

struct Flags {
    name: syn::Ident,
    storage: syn::Type,
    variants: Vec<syn::Ident>
}

impl syn::parse::Parse for Flags {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Visibility>()?;
        input.parse::<syn::Token![struct]>()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let storage = input.parse()?;
        let body;
        syn::braced!(body in input);
        let mut variants = Vec::new();
        while !body.is_empty() {
            body.call(syn::Attribute::parse_outer)?;
            body.parse::<syn::Token![const]>()?;
            variants.push(body.parse()?);
            body.parse::<syn::Token![=]>()?;
            body.parse::<syn::Expr>()?;
            body.parse::<syn::Token![;]>()?;
        }
        Ok(Self { name, storage, variants })
    }
}

fn resource_flags(root: &Path) -> syn::File {
    let items = read(&root.join("crates/baad/src/download/resource.rs")).items;
    let tokens = items
        .into_iter()
        .find_map(|item| match item {
            Item::Macro(item) if item.mac.path.is_ident("bitflags") => Some(item.mac.tokens),
            _ => None
        })
        .expect("native resource flags");
    let flags: Flags = syn::parse2(tokens).expect("parse native resource flags");
    let name = flags.name;
    let storage = flags.storage;
    let methods = flags.variants.iter().map(|flag| {
        let method = format_ident!("{}", flag.to_string().to_snake_case());
        quote!(pub const fn #method() -> #storage { baad_native::download::#name::#flag.bits() })
    });
    syn::parse2(quote! {
        #[diplomat::bridge]
        pub mod ffi {
            #[diplomat::opaque]
            pub struct BaadResourceCategory;
            impl BaadResourceCategory { #(#methods)* }
        }
    })
    .expect("generated resource flags")
}
