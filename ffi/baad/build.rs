use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use shadow_ffi::{TypeMapping, primitive, read};
use shadow_ffi_diplomat::cargo::CrateConfig;
use shadow_ffi_diplomat::{constants, mirror};
use syn::{Expr, File, ItemStruct, parse2};

const ENUMS: &[TypeMapping<'static>] = &[TypeMapping {
    shadow: "BaadFilterMethod",
    source: "crates/baad/src/download/filter.rs",
    name: "FilterMethod",
    native_path: "baad_native::download::FilterMethod"
}];
const RECORDS: &[TypeMapping<'static>] = &[TypeMapping {
    shadow: "BaadDownloaderOptions",
    source: "crates/baad/src/download/downloader.rs",
    name: "ResourceDownloader",
    native_path: "baad_native::download::ResourceDownloader"
}];

fn main() -> Result<(), baad_ffi_build::Error> {
    baad_ffi_build::build(CrateConfig {
        additional: Some(additional),
        runtime_in_umbrella: true,
        ..CrateConfig::new(transform)
    })
}

fn record(
    item: &mut ItemStruct,
    native: &ItemStruct,
    _: &str,
    additions: &mut TokenStream,
    conversions: &mut TokenStream
) {
    mirror::scalar_record(item, native);
    let ident = &item.ident;
    let mut defaults = TokenStream::new();
    let fields = native.fields.iter().filter(|field| primitive(&field.ty));

    for field in fields {
        let field_name = &field.ident;
        let attribute =
            field.attrs.iter().find(|a| a.path().is_ident("builder")).expect("builder default");
        let mut default = None;
        attribute
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    default = Some(meta.value()?.parse::<Expr>()?);
                }

                Ok(())
            })
            .expect("parse default");
        let default = default.expect("default value");
        defaults.extend(quote!(#field_name: #default,));
    }

    additions.extend(quote! {
        impl #ident {
            pub const fn default_options() -> Self { Self { #defaults } }
        }
    });

    let builder_methods = item.fields.iter().map(|field| {
        let field = &field.ident;
        quote!(.#field(self.#field))
    });
    conversions.extend(quote! {

        impl ffi::#ident {
            pub fn native(
                &self,
                output_dir: std::path::PathBuf,
                proxy: Option<String>
            ) -> baad_native::download::ResourceDownloader {
                baad_native::download::ResourceDownloader::builder()
                    .output_dir(output_dir)
                    .maybe_proxy(proxy)
                    #(#builder_methods)*
                    .build()
            }
        }
    });
}

fn transform(root: &Path, module: &str) -> File {
    let source = read(&root.join(format!("ffi/baad/src/{module}.rs")));
    let (mut source, conversions) = mirror::transform(root, source, &mirror::Config {
        enums: ENUMS,
        records: RECORDS,
        record,
        models: None
    });
    source.items.extend(parse2::<File>(conversions).expect("generated conversions").items);

    source
}

fn additional(root: &Path) -> File {
    constants::flags(
        &root.join("crates/baad/src/download/resource.rs"),
        "baad_native::download",
        "BaadResourceCategory"
    )
}
