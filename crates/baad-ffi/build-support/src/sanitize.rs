use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemStruct, Lit, Type};

use crate::config::Sanitized;
use crate::syntax::normalized;

pub(crate) fn render_sanitized(item: &ItemStruct, spec: &Sanitized) -> TokenStream {
    let name = &item.ident;
    let struct_defaults = struct_level_defaults(item);

    let fields = item.fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        if spec.skip_fields.contains(&field_name.to_string().as_str()) {
            return None;
        }

        let ty = sanitized_type(&field.ty);
        let default = sanitized_default(field, &struct_defaults);
        Some(quote! { #default pub #field_name: #ty })
    });
    let conversion = spec.native.map(|native| render_sanitized_conversion(item, spec, native));

    quote! {
        #[derive(Debug, Clone, uniffi::Record)]
        pub struct #name {
            #(#fields,)*
        }

        #conversion
    }
}

fn render_sanitized_conversion(item: &ItemStruct, spec: &Sanitized, native: &str) -> TokenStream {
    assert!(
        spec.skip_fields.is_empty(),
        "sanitized struct {} skips fields; a native conversion would be incomplete",
        spec.name
    );

    let name = &item.ident;
    let native: syn::Path = syn::parse_str(native)
        .unwrap_or_else(|error| panic!("sanitized native path {native}: {error}"));
    let fields: Vec<_> = item
        .fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("sanitized fields are named");
            let identity =
                normalized(&field.ty) == sanitized_type(&field.ty).to_string().replace(' ', "");
            assert!(
                identity,
                "sanitized struct {} field {field_name} changes type; conversion unsupported",
                spec.name
            );
            quote! { #field_name: value.#field_name }
        })
        .collect();

    quote! {
        impl From<#name> for #native {
            fn from(value: #name) -> Self {
                Self { #(#fields,)* }
            }
        }
    }
}

fn struct_level_defaults(item: &ItemStruct) -> Vec<(String, Expr)> {
    let Some(attr) = item.attrs.iter().find(|attr| attr.path().is_ident("default")) else {
        return Vec::new();
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Vec::new();
    };

    list.tokens
        .to_string()
        .split(',')
        .filter_map(|entry| {
            let (field, value) = entry.split_once(':')?;
            let expression = syn::parse_str::<Expr>(value.trim()).ok()?;
            Some((field.trim().to_string(), expression))
        })
        .collect()
}

fn sanitized_type(ty: &Type) -> TokenStream {
    let text = normalized(ty);
    if text.contains("Path") {
        return quote! { String };
    }
    if text == "Option<Proxy>" {
        return quote! { Option<String> };
    }
    if text == "usize" {
        return quote! { u64 };
    }
    quote! { #ty }
}

fn sanitized_default(field: &syn::Field, struct_defaults: &[(String, Expr)]) -> TokenStream {
    let is_option = normalized(&field.ty).starts_with("Option<");
    let field_name = field.ident.as_ref().map(ToString::to_string).unwrap_or_default();

    let struct_default = struct_defaults
        .iter()
        .find(|(name, _)| *name == field_name)
        .map(|(_, expression)| expression.clone());

    let builder_default = field.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("builder") {
            return None;
        }
        let syn::Meta::List(list) = &attr.meta else {
            return None;
        };
        let text = list.tokens.to_string();
        let expression = text.strip_prefix("default")?.trim().strip_prefix('=')?.trim();
        syn::parse_str::<Expr>(expression).ok()
    });

    match struct_default.or(builder_default).as_ref().and_then(evaluate_literal) {
        Some(value) => quote! { #[uniffi(default = #value)] },
        None if is_option => quote! { #[uniffi(default = None)] },
        None => quote! {}
    }
}

fn evaluate_literal(expression: &Expr) -> Option<TokenStream> {
    match expression {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Int(_) | Lit::Bool(_) | Lit::Str(_) => Some(quote! { #literal }),
            _ => None
        },
        Expr::Binary(binary) if matches!(binary.op, syn::BinOp::Mul(_)) => {
            let left = evaluate_integer(&binary.left)?;
            let right = evaluate_integer(&binary.right)?;
            let product = left.checked_mul(right)?;
            let literal = proc_macro2::Literal::u64_unsuffixed(product);
            Some(quote! { #literal })
        }
        _ => None
    }
}

fn evaluate_integer(expression: &Expr) -> Option<u64> {
    match expression {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Int(value) => value.base10_parse().ok(),
            _ => None
        },
        Expr::Binary(binary) if matches!(binary.op, syn::BinOp::Mul(_)) => {
            let left = evaluate_integer(&binary.left)?;
            let right = evaluate_integer(&binary.right)?;
            left.checked_mul(right)
        }
        _ => None
    }
}
