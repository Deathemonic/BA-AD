use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Type};

use crate::config::Config;
use crate::consts::render_consts;
use crate::context::{Context, RemoteItem, remote_name};
use crate::mirror::render_enum_mirror;
use crate::observer::render_observer_uniffi;
use crate::syntax::{crate_path, strip_enum, strip_struct};

pub(crate) fn render_uniffi(config: &Config, context: &Context) -> TokenStream {
    let custom_types = config.custom_types.iter().map(|custom| {
        let alias = format_ident!("{}", custom.alias);
        let target: Type = syn::parse_str(custom.target).expect("custom type target");
        let over = format_ident!("{}", custom.over);
        let lower: Expr = syn::parse_str(custom.lower).expect("custom type lower");
        let try_lift: Expr = syn::parse_str(custom.try_lift).expect("custom type lift");
        quote! {
            type #alias = #target;

            uniffi::custom_type!(#alias, #over, {
                remote,
                lower: #lower,
                try_lift: #try_lift
            });
        }
    });

    let uses_maps = context.remotes.iter().any(|remote| match &remote.item {
        RemoteItem::Record(item) => item.fields.iter().any(|field| {
            let ty = &field.ty;
            quote!(#ty).to_string().contains("HashMap")
        }),
        _ => false
    });
    let map_import = uses_maps.then(|| quote! { use std::collections::HashMap; });

    let aliases = context.remotes.iter().map(|remote| {
        let crate_path = crate_path(&remote.crate_path);
        let source = format_ident!("{}", remote.source);
        let name = format_ident!("{}", remote_name(&remote.item));
        quote! { type #name = #crate_path::#source; }
    });

    let blocks = context.remotes.iter().map(|remote| match &remote.item {
        RemoteItem::Record(item) => {
            let stripped = strip_struct(item);
            quote! { #[uniffi::remote(Record)] #stripped }
        }
        RemoteItem::Enum(item) => {
            let stripped = strip_enum(item);
            quote! { #[uniffi::remote(Enum)] #stripped }
        }
        RemoteItem::Error(item) => {
            let stripped = strip_enum(item);
            quote! { #[uniffi::remote(Error)] #stripped }
        }
    });

    let data_mirrors = context.data_mirrors.iter().map(|mirror| {
        render_enum_mirror(&mirror.crate_path, &mirror.item, &context.mirrored_names, false)
    });
    let error_mirrors = context.error_mirrors.iter().map(|mirror| {
        render_enum_mirror(&mirror.crate_path, &mirror.item, &context.mirrored_names, true)
    });
    let sanitized = context.sanitized.iter();
    let const_record = (!context.consts.is_empty()).then(|| render_consts(&context.consts));
    let observer = config.observer.then(render_observer_uniffi);
    let run_blocking = config.blocking_runtime.then(render_run_blocking);

    quote! {
        #(#custom_types)*
        #map_import
        #(#aliases)*
        #(#blocks)*
        #(#data_mirrors)*
        #(#error_mirrors)*
        #(#sanitized)*
        #const_record
        #observer
        #run_blocking
    }
}

fn render_run_blocking() -> TokenStream {
    quote! {
        #[allow(dead_code)]
        pub(crate) async fn run_blocking<T, F>(task: F) -> T
        where
            F: FnOnce(tokio::runtime::Handle) -> T + Send + 'static,
            T: Send + 'static
        {
            let handle = tokio::runtime::Handle::current();
            tokio::task::spawn_blocking(move || task(handle))
                .await
                .unwrap_or_else(|error| std::panic::resume_unwind(error.into_panic()))
        }
    }
}
