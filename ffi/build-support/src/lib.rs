mod constants;
mod mirror;
mod models;
mod source;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::{env, fs};

use constants::constants;
use mirror::transform;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

const FAMILIES: &[(&str, &[&str])] = &[
    ("baad-shared", &["adapter", "error", "models", "observer", "progress", "runtime", "shared"]),
    ("baad-utils", &["adapter", "utils"]),
    ("baad-dm", &["adapter", "dm"]),
    ("baad", &["catalog", "cdn", "clients", "download", "download_adapter", "filter", "strategy"])
];

pub fn build(family: &str) {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let root = manifest.parent().expect("ffi directory").parent().expect("workspace root");
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("build output"));
    generate(family, &manifest, root, &out);
}

pub fn generate(family: &str, manifest: &Path, root: &Path, out: &Path) {
    let mut entry = TokenStream::new();
    let mut own = TokenStream::new();
    for (name, modules) in FAMILIES {
        let namespace = format_ident!("{}_ffi", name.replace('-', "_"));
        let mut contents = TokenStream::new();
        for module in *modules {
            let source = transform(root, name, module);
            let module = format_ident!("{module}");
            let visibility = if *name == "baad-shared" && module == "observer" {
                TokenStream::new()
            } else {
                quote!(pub)
            };
            contents.extend(quote!(#visibility mod #module { #source }));
        }
        if let Some(source) = constants(root, name) {
            contents.extend(quote!(pub mod constants { #source }));
        }
        if *name == family {
            own = contents.clone();
        }
        let namespace_source =
            contents.to_string().replace("crate ::", &format!("crate :: {namespace} ::"));
        let contents: TokenStream = namespace_source.parse().expect("namespaced source");
        entry.extend(quote!(pub mod #namespace { #contents }));
    }
    let own: syn::File = syn::parse2(own).expect("crate API");
    fs::write(out.join("api.rs"), prettyplease::unparse(&own)).expect("write crate API");
    let entry: syn::File = syn::parse2(entry).expect("binding entry");
    let mut entry = prettyplease::unparse(&entry);
    for name in ["baad_shared_ffi", "baad_utils_ffi", "baad_dm_ffi"] {
        entry = entry.replace(&format!("use {name}::"), &format!("use crate::{name}::"));
    }
    let entry_path = out.join("bindings.rs");
    fs::write(&entry_path, entry).expect("write binding entry");
    let generated = out.join("headers");
    if generated.exists() {
        fs::remove_dir_all(&generated).expect("clear previous generated headers");
    }
    let mut config = diplomat_tool::config::Config::default();
    config.shared_config.unsafe_references_in_callbacks = Some(true);
    diplomat_tool::r#gen(
        &entry_path,
        "c",
        &generated,
        &diplomat_tool::DocsUrlGenerator::default(),
        config,
        true
    )
    .expect("generate C headers");
    let destination = manifest.join("bindings");
    fs::create_dir_all(&destination).expect("create bindings directory");
    let mut headers = Vec::new();
    let mut expected = BTreeSet::new();
    for entry in fs::read_dir(&generated).expect("generated headers") {
        let path = entry.expect("header entry").path();
        let name = path.file_name().expect("header filename").to_string_lossy();
        let family_owner = owner(&name);
        if family_owner != family {
            continue;
        }
        let mut content = fs::read_to_string(&path).expect("read header");
        for line in content.clone().lines() {
            if let Some(include) =
                line.strip_prefix("#include \"").and_then(|s| s.strip_suffix('"'))
            {
                let other = owner(include);
                if other != family && include != "diplomat_runtime.h" {
                    content = content.replace(
                        &format!("\"{include}\""),
                        &format!("\"../../{other}/bindings/{include}\"")
                    );
                }
            }
        }
        expected.insert(name.to_string());
        write_changed(&destination.join(name.as_ref()), &content);
        if !name.ends_with(".d.h") {
            headers.push(name.into_owned());
        }
    }
    write_changed(
        &destination.join("diplomat_runtime.h"),
        &fs::read_to_string(generated.join("diplomat_runtime.h")).expect("runtime header")
    );
    headers.sort();
    let guard = format!("{}_H", family.replace('-', "_").to_uppercase());
    let umbrella = format!(
        "#ifndef {guard}\n#define {guard}\n{}#endif\n",
        headers.iter().map(|h| format!("#include \"{h}\"\n")).collect::<String>()
    );
    let umbrella_name = format!("{}.h", family.replace('-', "_"));
    write_changed(&destination.join(&umbrella_name), &umbrella);
    expected.insert(umbrella_name);
    expected.insert("diplomat_runtime.h".into());
    for entry in fs::read_dir(&destination).expect("previous bindings") {
        let path = entry.expect("previous binding").path();
        if path.extension().is_some_and(|extension| extension == "h")
            && !expected
                .contains(path.file_name().expect("header filename").to_string_lossy().as_ref())
        {
            fs::remove_file(path).expect("remove obsolete generated binding");
        }
    }
}

fn owner(name: &str) -> &'static str {
    if name.starts_with("BaadShared") || name.starts_with("BaadError") {
        "baad-shared"
    } else if name.starts_with("BaadUtils") {
        "baad-utils"
    } else if name.starts_with("BaadDm") {
        "baad-dm"
    } else {
        "baad"
    }
}

fn write_changed(path: &Path, content: &str) {
    if fs::read_to_string(path).ok().as_deref() != Some(content) {
        fs::write(path, content).expect("write generated file");
    }
}
