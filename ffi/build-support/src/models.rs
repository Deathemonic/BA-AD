use std::collections::BTreeMap;
use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Item, Type};

use crate::source::{primitive, read, text};

fn wrapper(name: &str) -> syn::Ident {
    format_ident!("BaadShared{}", if name == "GlobalCatalog" { "GlobalCatalogData" } else { name })
}

fn getter(field: &syn::Field, names: &BTreeMap<String, syn::Ident>) -> TokenStream {
    let name = field.ident.as_ref().expect("named native field");
    let ty = &field.ty;
    let value = quote!(self.0.#name);
    let t = text(ty);
    let len = format_ident!("{name}_len");
    let at = format_ident!("{name}_at");
    let keys = format_ident!("{name}_keys");
    let get = format_ident!("{name}_get");
    if primitive(ty) {
        return quote!(pub fn #name(&self) -> #ty { #value });
    }
    if t == "String" || t.starts_with("Cow<") || t == "&'staticstr" {
        return quote!(pub fn #name<'a>(&'a self) -> &'a str { &#value });
    }
    if t == "Option<String>" {
        let has = format_ident!("has_{name}");
        return quote! {
            pub fn #name<'a>(&'a self) -> &'a str { #value.as_deref().unwrap_or("") }
            pub fn #has(&self) -> bool { #value.is_some() }
        };
    }
    if t == "Option<Vec<String>>" {
        let has = format_ident!("has_{name}");
        return quote! {
            pub fn #has(&self) -> bool { #value.is_some() }
            pub fn #len(&self) -> usize { #value.as_ref().map_or(0, Vec::len) }
            pub fn #at<'a>(&'a self, index: usize) -> Option<&'a str> { #value.as_ref().and_then(|v| v.get(index)).map(String::as_str) }
        };
    }
    if let Some(inner) = t.strip_prefix("Vec<").and_then(|t| t.strip_suffix('>')) {
        let length = quote!(pub fn #len(&self) -> usize { #value.len() });
        if inner == "String" {
            return quote! { #length pub fn #at<'a>(&'a self, index: usize) -> Option<&'a str> { #value.get(index).map(String::as_str) } };
        }
        if inner == "HashMap<String,String>" {
            return quote! {
                #length
                pub fn #keys(&self, index: usize) -> Option<Box<BaadSharedStrings>> {
                    #value.get(index).map(|map| Box::new(BaadSharedStrings(map.keys().cloned().collect())))
                }
                pub fn #get<'a>(&'a self, index: usize, key: &str) -> Option<&'a str> {
                    #value.get(index).and_then(|map| map.get(key)).map(String::as_str)
                }
            };
        }
        if let Some(wrapped) = names.get(inner) {
            return quote! { #length pub fn #at<'a>(&'a self, index: usize) -> Option<&'a #wrapped> { #value.get(index).map(#wrapped::borrow) } };
        }
        let inner: Type = syn::parse_str(inner).expect("vector type");
        if primitive(&inner) {
            return quote!(pub fn #name<'a>(&'a self) -> &'a [#inner] { &#value });
        }
    }
    if let Some(inner) = t.strip_prefix("HashMap<String,").and_then(|t| t.strip_suffix('>')) {
        let wrapped = names.get(inner).unwrap_or_else(|| panic!("unsupported map value {inner}"));
        return quote! {
            pub fn #keys(&self) -> Box<BaadSharedStrings> { Box::new(BaadSharedStrings(#value.keys().cloned().collect())) }
            pub fn #get<'a>(&'a self, key: &str) -> Option<&'a #wrapped> { #value.get(key).map(#wrapped::borrow) }
        };
    }
    if let Some(wrapped) = names.get(&t) {
        return quote!(pub fn #name<'a>(&'a self) -> &'a #wrapped { #wrapped::borrow(&#value) });
    }
    if t == "ChinaMediaType" {
        return quote!(pub fn #name(&self) -> crate::shared::ffi::BaadSharedChinaMediaType { #value.into() });
    }
    if t == "HashValue" {
        let kind = format_ident!("{name}_kind");
        return quote! {
            pub fn #kind(&self) -> BaadSharedHashKind { (&#value).into() }
            #[allow(clippy::should_implement_trait)]
            pub fn #name(&self, output: &mut DiplomatWrite) { crate::models::write_hash(&#value, output); }
        };
    }
    panic!("unsupported native field {name}: {t}; add a boundary adapter");
}

pub(crate) fn generate_models(root: &Path, items: &mut Vec<Item>, outside: &mut TokenStream) {
    let structs: Vec<_> = read(&root.join("crates/baad-shared/src/types.rs"))
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(item),
            _ => None
        })
        .collect();
    let names: BTreeMap<_, _> =
        structs.iter().map(|s| (s.ident.to_string(), wrapper(&s.ident.to_string()))).collect();
    let mut generated = TokenStream::new();
    for native in structs {
        let ident = &native.ident;
        let name = &names[&ident.to_string()];
        let getters = native
            .fields
            .iter()
            .filter(|field| matches!(field.vis, syn::Visibility::Public(_)))
            .map(|f| getter(f, &names));
        generated.extend(quote! {
            #[diplomat::opaque]
            #[repr(transparent)]
            pub struct #name(pub baad_shared::#ident);
            #[allow(clippy::missing_const_for_fn)]
            impl #name { #(#getters)* }
        });
        outside.extend(quote! {
            impl ffi::#name {
                pub const fn borrow(value: &baad_shared::#ident) -> &Self {
                    unsafe { &*(std::ptr::from_ref(value).cast::<Self>()) }
                }
            }
            const _: () = {
                assert!(std::mem::size_of::<ffi::#name>() == std::mem::size_of::<baad_shared::#ident>());
                assert!(std::mem::align_of::<ffi::#name>() == std::mem::align_of::<baad_shared::#ident>());
            };
        });
    }
    items.extend(syn::parse2::<syn::File>(generated).expect("generated models").items);
}
