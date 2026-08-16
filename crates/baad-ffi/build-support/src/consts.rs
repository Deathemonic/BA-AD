use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemConst, Type};

use crate::syntax::{crate_path, normalized};

pub struct GeneratedConst {
    pub(crate) crate_path: String,
    pub(crate) field: ConstField
}

pub struct ConstField {
    pub(crate) source: syn::Ident,
    pub(crate) target: syn::Ident,
    pub(crate) ty: ConstType
}

pub enum ConstType {
    String,
    Bytes,
    Strings
}

impl ConstField {
    pub(crate) fn from_item(item: &ItemConst) -> Option<Self> {
        Some(Self {
            source: item.ident.clone(),
            target: format_ident!("{}", item.ident.to_string().to_ascii_lowercase()),
            ty: ConstType::from_type(&item.ty)?
        })
    }
}

impl ConstType {
    fn from_type(ty: &Type) -> Option<Self> {
        match normalized(ty).as_str() {
            "&str" => Some(Self::String),
            "&[u8]" => Some(Self::Bytes),
            "&[&str]" => Some(Self::Strings),
            _ => None
        }
    }

    fn field_type(&self) -> TokenStream {
        match self {
            Self::String => quote! { String },
            Self::Bytes => quote! { Vec<u8> },
            Self::Strings => quote! { Vec<String> }
        }
    }
}

pub fn render_consts(consts: &[GeneratedConst]) -> TokenStream {
    let record_fields = consts.iter().map(|generated| {
        let name = &generated.field.target;
        let ty = generated.field.ty.field_type();
        quote! { pub #name: #ty }
    });
    let values = consts.iter().map(|generated| {
        let crate_path = crate_path(&generated.crate_path);
        let name = &generated.field.target;
        let source = &generated.field.source;
        let value = match generated.field.ty {
            ConstType::String => quote! { String::from(#crate_path::#source) },
            ConstType::Bytes => quote! { #crate_path::#source.to_vec() },
            ConstType::Strings => quote! { string_vec(#crate_path::#source) }
        };
        quote! { #name: #value }
    });

    quote! {
        #[derive(Debug, Clone, uniffi::Record)]
        pub struct Consts {
            #(#record_fields,)*
        }

        #[uniffi::export]
        pub fn consts() -> Consts {
            Consts {
                #(#values,)*
            }
        }

        fn string_vec(values: &[&str]) -> Vec<String> {
            values.iter().copied().map(String::from).collect()
        }
    }
}
