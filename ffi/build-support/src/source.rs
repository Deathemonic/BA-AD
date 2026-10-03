use std::fs;
use std::path::Path;

use quote::ToTokens;
use syn::{Item, ItemEnum, ItemStruct, Type};

pub(crate) fn read(path: &Path) -> syn::File {
    if std::env::var_os("OUT_DIR").is_some() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    syn::parse_file(&fs::read_to_string(path).expect("read Rust source"))
        .expect("parse Rust source")
}

pub(crate) fn native_enum(root: &Path, file: &str, name: &str) -> ItemEnum {
    read(&root.join(file))
        .items
        .into_iter()
        .find_map(|item| match item {
            Item::Enum(item) if item.ident == name => Some(item),
            _ => None
        })
        .unwrap_or_else(|| panic!("native enum {name} missing in {file}"))
}

pub(crate) fn native_struct(root: &Path, file: &str, name: &str) -> ItemStruct {
    read(&root.join(file))
        .items
        .into_iter()
        .find_map(|item| match item {
            Item::Struct(item) if item.ident == name => Some(item),
            _ => None
        })
        .unwrap_or_else(|| panic!("native struct {name} missing in {file}"))
}

pub(crate) fn text(ty: &Type) -> String { ty.to_token_stream().to_string().replace(' ', "") }

pub(crate) fn primitive(ty: &Type) -> bool {
    matches!(
        text(ty).as_str(),
        "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "usize"
            | "isize"
            | "f32"
            | "f64"
    )
}

pub(crate) fn enum_spec(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match name {
        "BaadFilterMethod" => Some((
            "crates/baad/src/download/filter.rs",
            "FilterMethod",
            "baad_native::download::FilterMethod"
        )),
        "BaadDmHashType" => {
            Some(("crates/baad-dm/src/download/hash.rs", "HashType", "baad_dm::HashType"))
        }
        "BaadSharedPlatform" => {
            Some(("crates/baad-shared/src/platform.rs", "Platform", "baad_shared::Platform"))
        }
        "BaadSharedBuildType" => {
            Some(("crates/baad-shared/src/platform.rs", "BuildType", "baad_shared::BuildType"))
        }
        "BaadSharedChinaMediaType" => Some((
            "crates/baad-shared/src/types.rs",
            "ChinaMediaType",
            "baad_shared::ChinaMediaType"
        )),
        "BaadSharedProgressUnit" => Some((
            "crates/baad-shared/src/observer.rs",
            "ProgressUnit",
            "baad_shared::ProgressUnit"
        )),
        "BaadSharedProgressStatusKind" => Some((
            "crates/baad-shared/src/observer.rs",
            "ProgressStatus",
            "baad_shared::ProgressStatus"
        )),
        "BaadSharedProgressEventKind" => Some((
            "crates/baad-shared/src/observer.rs",
            "ProgressEvent",
            "baad_shared::ProgressEvent"
        )),
        "BaadSharedHashKind" => {
            Some(("crates/baad-shared/src/types.rs", "HashValue", "baad_shared::HashValue"))
        }
        _ => None
    }
}

pub(crate) fn record_spec(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match name {
        "BaadUtilsAlignedLine" => Some((
            "crates/baad-utils/src/formatter/line.rs",
            "AlignedLine",
            "baad_utils::formatter::AlignedLine"
        )),
        "BaadUtilsLoggingConfig" => Some((
            "crates/baad-utils/src/logging/config.rs",
            "LoggingConfig",
            "baad_utils::config::LoggingConfig"
        )),
        "BaadDmZipFileInfo" => {
            Some(("crates/baad-dm/src/zip/types.rs", "ZipFileInfo", "baad_dm::ZipFileInfo"))
        }
        "BaadDmDownloaderConfig" => Some((
            "crates/baad-dm/src/downloader/config.rs",
            "DownloaderConfig",
            "baad_dm::DownloaderConfig"
        )),
        "BaadDownloaderOptions" => Some((
            "crates/baad/src/download/downloader.rs",
            "ResourceDownloader",
            "baad_native::download::ResourceDownloader"
        )),
        _ => None
    }
}
