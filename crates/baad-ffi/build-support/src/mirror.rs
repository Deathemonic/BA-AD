use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum, Type};

use crate::syntax::{crate_path, normalized, type_is_ffi_safe};

enum FieldConversion {
    Keep,
    BoxStr,
    ArcStr,
    Mirrored,
    Message
}

impl FieldConversion {
    fn classify(ty: &Type, mirrored_names: &[String]) -> Self {
        let text = normalized(ty);
        if mirrored_names.contains(&text) {
            return FieldConversion::Mirrored;
        }
        match text.as_str() {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64"
            | "bool" | "String" => FieldConversion::Keep,
            "Box<str>" => FieldConversion::BoxStr,
            "Arc<str>" => FieldConversion::ArcStr,
            _ => FieldConversion::Message
        }
    }

    fn target_type(&self, ty: &Type) -> TokenStream {
        match self {
            FieldConversion::Keep | FieldConversion::Mirrored => quote! { #ty },
            FieldConversion::BoxStr | FieldConversion::ArcStr | FieldConversion::Message => {
                quote! { String }
            }
        }
    }

    fn convert(&self, binding: &syn::Ident) -> TokenStream {
        match self {
            FieldConversion::Keep => quote! { #binding },
            FieldConversion::Mirrored | FieldConversion::BoxStr => quote! { #binding.into() },
            FieldConversion::ArcStr => quote! { String::from(#binding.as_ref()) },
            FieldConversion::Message => quote! { #binding.to_string() }
        }
    }
}

pub(crate) fn enum_is_mirrorable(item: &ItemEnum) -> bool {
    item.variants.iter().all(|variant| {
        variant.fields.iter().all(|field| {
            let text = normalized(&field.ty);
            type_is_ffi_safe(&field.ty)
                || text == "Arc<str>"
                || text == "Box<str>"
                || text == "DownloadStatus"
        })
    })
}

fn mirror_field_name(name: &syn::Ident) -> syn::Ident {
    match name.to_string().as_str() {
        "source" => format_ident!("message"),
        _ => name.clone()
    }
}

pub(crate) fn render_enum_mirror(
    crate_path_text: &str,
    item: &ItemEnum,
    mirrored_names: &[String],
    error: bool
) -> TokenStream {
    let crate_path = crate_path(crate_path_text);
    let source = &item.ident;
    let name = source;

    let variants = item.variants.iter().map(|variant| {
        let mut variant = variant.clone();
        for field in &mut variant.fields {
            field.attrs.clear();
        }
        let display = error.then(|| display_attribute(&variant));
        let variant = &variant;
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => quote! { #display #ident },
            Fields::Unnamed(fields) => {
                let types = fields.unnamed.iter().map(|field| {
                    FieldConversion::classify(&field.ty, mirrored_names).target_type(&field.ty)
                });
                quote! { #display #ident(#(#types),*) }
            }
            Fields::Named(fields) => {
                let entries = fields.named.iter().map(|field| {
                    let field_name = mirror_field_name(field.ident.as_ref().unwrap());
                    let ty =
                        FieldConversion::classify(&field.ty, mirrored_names).target_type(&field.ty);
                    quote! { #field_name: #ty }
                });
                quote! { #display #ident { #(#entries),* } }
            }
        }
    });

    let from_arms = item.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => quote! { #crate_path::#source::#ident => Self::#ident },
            Fields::Unnamed(fields) => {
                let bindings: Vec<_> = (0..fields.unnamed.len())
                    .map(|index| format_ident!("field_{index}"))
                    .collect();
                let conversions = fields.unnamed.iter().zip(&bindings).map(|(field, binding)| {
                    FieldConversion::classify(&field.ty, mirrored_names).convert(binding)
                });
                quote! {
                    #crate_path::#source::#ident(#(#bindings),*) => Self::#ident(#(#conversions),*)
                }
            }
            Fields::Named(fields) => {
                let bindings: Vec<_> =
                    fields.named.iter().map(|field| field.ident.clone().unwrap()).collect();
                let conversions = fields.named.iter().zip(&bindings).map(|(field, binding)| {
                    let target = mirror_field_name(binding);
                    let conversion_kind = FieldConversion::classify(&field.ty, mirrored_names);
                    let conversion = conversion_kind.convert(binding);
                    if matches!(conversion_kind, FieldConversion::Keep) && target == *binding { quote! { #binding } } else { quote! { #target: #conversion } }
                });
                quote! {
                    #crate_path::#source::#ident { #(#bindings),* } => Self::#ident { #(#conversions),* }
                }
            }
        }
    });

    let derive = if error {
        quote! { #[derive(Debug, thiserror::Error, uniffi::Error)] }
    } else {
        quote! { #[derive(Debug, Clone, uniffi::Enum)] }
    };

    quote! {
        #derive
        pub enum #name {
            #(#variants,)*
        }

        impl From<#crate_path::#source> for #name {
            fn from(value: #crate_path::#source) -> Self {
                match value {
                    #(#from_arms,)*
                }
            }
        }
    }
}

fn display_attribute(variant: &syn::Variant) -> TokenStream {
    let error_attr = variant.attrs.iter().find(|attr| attr.path().is_ident("error"));

    match error_attr {
        Some(attr) => {
            let is_transparent = matches!(
                &attr.meta,
                syn::Meta::List(list) if list.tokens.to_string() == "transparent"
            );
            if !is_transparent {
                return quote! { #attr };
            }
            match &variant.fields {
                Fields::Unnamed(_) => quote! { #[error("{0}")] },
                Fields::Named(fields) => {
                    let field_name = fields.named[0].ident.as_ref().unwrap().to_string();
                    let format = format!("{{{field_name}}}");
                    quote! { #[error(#format)] }
                }
                Fields::Unit => quote! { #[error("error")] }
            }
        }
        None => {
            let format = variant.ident.to_string();
            quote! { #[error(#format)] }
        }
    }
}
