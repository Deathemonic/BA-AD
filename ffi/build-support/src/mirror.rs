use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use shadow_ffi::{native_enum, primitive, read, text};
use syn::Fields;

use crate::config::{ENUMS, RECORDS};
use crate::models::generate_models;

fn record(
    item: &mut syn::ItemStruct,
    native: &syn::ItemStruct,
    path: &str,
    additions: &mut TokenStream,
    conversions: &mut TokenStream
) {
    let name = native.ident.to_string();
    if name == "AlignedLine" {
        let ident = &item.ident;
        item.generics = native.generics.clone();
        let fields: Vec<_> = native
            .fields
            .iter()
            .cloned()
            .map(|mut field| {
                field.attrs.clear();
                if text(&field.ty) == "&'aLevel" {
                    field.ty = syn::parse_quote!(BaadUtilsLogLevel);
                } else if text(&field.ty) == "&'astr" {
                    field.ty = syn::parse_quote!(DiplomatUtf8StrSlice<'a>);
                }
                field
            })
            .collect();
        item.fields = Fields::Named(syn::parse_quote!({ #(#fields,)* }));
        let assignments = native.fields.iter().map(|field| {
            let field_name = &field.ident;
            if text(&field.ty) == "&'aLevel" {
                quote!(#field_name: value.#field_name.native_ref(),)
            } else if text(&field.ty) == "&'astr" {
                quote!(#field_name: value.#field_name.into(),)
            } else {
                quote!(#field_name: value.#field_name,)
            }
        });
        let path: syn::Path = syn::parse_str(path).expect("aligned record path");
        conversions.extend(quote! {
            impl<'a> From<ffi::#ident<'a>> for #path<'a> {
                fn from(value: ffi::#ident<'a>) -> Self { Self { #(#assignments)* } }
            }
        });
        return;
    }
    let fields: Vec<_> = native
        .fields
        .iter()
        .filter(|f| primitive(&f.ty))
        .cloned()
        .map(|mut field| {
            field.attrs.clear();
            field.vis = syn::parse_quote!(pub);
            field
        })
        .collect();
    let named = fields.iter().map(|f| &f.ident);
    item.fields = Fields::Named(syn::parse_quote!({ #(#fields,)* }));
    if name == "LoggingConfig" || name == "ZipFileInfo" || name == "DownloaderConfig" {
        let ident = &item.ident;
        let path: syn::Path = syn::parse_str(path).expect("native record path");
        let names: Vec<_> = named.collect();
        let path = if name == "DownloaderConfig" { quote!(#path<'_>) } else { quote!(#path) };
        conversions.extend(quote! {
            impl From<#path> for ffi::#ident {
                fn from(value: #path) -> Self { Self { #(#names: value.#names,)* } }
            }

        });
        if name == "DownloaderConfig" {
            let builder_methods = names.iter().map(|field| quote!(.#field(self.#field)));
            conversions.extend(quote! {
                                    impl ffi::#ident {
                                        pub fn native<'a>(&self, directory: &'a std::path::Path, proxy: Option<reqwest::Proxy>) -> baad_dm::DownloaderConfig<'a> {
                                            baad_dm::DownloaderConfig::builder().directory(directory).maybe_proxy(proxy) #(#builder_methods)* .build()
                                        }
                                    }
                                });
        }
        if name == "LoggingConfig" || name == "ZipFileInfo" {
            conversions.extend(quote! {
                impl From<ffi::#ident> for #path {
                    fn from(value: ffi::#ident) -> Self { Self { #(#names: value.#names,)* } }
                }
            });
        }
    } else if name == "ResourceDownloader" {
        let ident = &item.ident;
        let mut defaults = TokenStream::new();
        for field in native.fields.iter().filter(|f| primitive(&f.ty)) {
            let field_name = &field.ident;
            let attribute =
                field.attrs.iter().find(|a| a.path().is_ident("builder")).expect("builder default");
            let mut default = None;
            attribute
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        default = Some(meta.value()?.parse::<syn::Expr>()?);
                    }
                    Ok(())
                })
                .expect("parse default");
            let default = default.expect("default value");
            defaults.extend(quote!(#field_name: #default,));
        }
        additions.extend(quote! { impl #ident { #[allow(clippy::missing_const_for_fn)] pub fn default_options() -> Self { Self { #defaults } } } });
        let builder_methods = fields.iter().map(|field| {
            let field = &field.ident;
            quote!(.#field(self.#field))
        });
        conversions.extend(quote! {
                                impl ffi::#ident {
                                    pub fn native(&self, output_dir: std::path::PathBuf, proxy: Option<String>) -> baad_native::download::ResourceDownloader {
                                        baad_native::download::ResourceDownloader::builder().output_dir(output_dir).maybe_proxy(proxy) #(#builder_methods)* .build()
                                    }
                                }
                            });
    }
}

pub(crate) fn transform(root: &Path, family: &str, module: &str) -> syn::File {
    let source = read(&root.join(format!("ffi/{family}/src/{module}.rs")));
    let models = if family == "baad-shared" && module == "models" {
        Some(generate_models as shadow_ffi_diplomat::mirror::ModelHook)
    } else {
        None
    };
    let (mut source, mut conversions) = shadow_ffi_diplomat::mirror::transform(
        root,
        source,
        &shadow_ffi_diplomat::mirror::Config {
            enums: ENUMS,
            records: RECORDS,
            record,
            models
        }
    );
    if family == "baad-shared" && module == "models" {
        let native = native_enum(root, "crates/baad-shared/src/types.rs", "HashValue");
        let arms = native.variants.iter().map(|variant| {
            let name = &variant.ident;
            match &variant.fields {
                Fields::Unit => quote!(ffi::BaadSharedHashKind::#name => Ok(baad_shared::HashValue::#name),),
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let ty = &fields.unnamed[0].ty;
                    if text(ty) == "String" {
                        quote!(ffi::BaadSharedHashKind::#name => Ok(baad_shared::HashValue::#name(value.into())),)
                    } else if primitive(ty) {
                        quote!(ffi::BaadSharedHashKind::#name => value.parse::<#ty>().map(baad_shared::HashValue::#name).map_err(|error| crate::error::error(error.to_string())),)
                    } else { panic!("unsupported native hash payload {}: {}", name, text(ty)); }
                }
                _ => panic!("unsupported native hash variant {}", name),
            }
        });
        conversions.extend(quote! {
            pub fn hash(kind: ffi::BaadSharedHashKind, value: &str) -> Result<baad_shared::HashValue, Box<crate::error::ffi::BaadError>> {
                match kind { #(#arms)* }
            }
        });
    }
    if family == "baad-shared" && module == "progress" {
        let native = native_enum(root, "crates/baad-shared/src/observer.rs", "ProgressStatus");
        let arms = native.variants.iter().map(|variant| {
            let name = &variant.ident;
            match &variant.fields {
                Fields::Unit => quote!(baad_shared::ProgressStatus::#name => "",),
                Fields::Unnamed(fields)
                    if fields.unnamed.len() == 1 && {
                        let ty = text(&fields.unnamed[0].ty);
                        ty == "String"
                            || ty.starts_with("Cow<")
                            || ty.starts_with("Arc<")
                            || ty == "Box<str>"
                    } =>
                {
                    quote!(baad_shared::ProgressStatus::#name(reason) => reason.as_ref(),)
                }
                Fields::Unnamed(_) => quote!(baad_shared::ProgressStatus::#name(..) => "",),
                Fields::Named(_) => quote!(baad_shared::ProgressStatus::#name { .. } => "",)
            }
        });
        conversions.extend(quote! { #[allow(clippy::match_same_arms)] pub fn status_reason(status: &baad_shared::ProgressStatus) -> &str { match status { #(#arms)* } } });
    }
    source
        .items
        .extend(syn::parse2::<syn::File>(conversions).expect("generated conversions").items);
    source
}
