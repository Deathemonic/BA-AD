use shadow_ffi::TypeMapping;

pub(crate) const ENUMS: &[TypeMapping<'static>] = &[
    TypeMapping {
        shadow: "BaadFilterMethod",
        source: "crates/baad/src/download/filter.rs",
        name: "FilterMethod",
        native_path: "baad_native::download::FilterMethod"
    },
    TypeMapping {
        shadow: "BaadDmHashType",
        source: "crates/baad-dm/src/download/hash.rs",
        name: "HashType",
        native_path: "baad_dm::HashType"
    },
    TypeMapping {
        shadow: "BaadSharedPlatform",
        source: "crates/baad-shared/src/platform.rs",
        name: "Platform",
        native_path: "baad_shared::Platform"
    },
    TypeMapping {
        shadow: "BaadSharedBuildType",
        source: "crates/baad-shared/src/platform.rs",
        name: "BuildType",
        native_path: "baad_shared::BuildType"
    },
    TypeMapping {
        shadow: "BaadSharedChinaMediaType",
        source: "crates/baad-shared/src/types.rs",
        name: "ChinaMediaType",
        native_path: "baad_shared::ChinaMediaType"
    },
    TypeMapping {
        shadow: "BaadSharedProgressUnit",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressUnit",
        native_path: "baad_shared::ProgressUnit"
    },
    TypeMapping {
        shadow: "BaadSharedProgressStatusKind",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressStatus",
        native_path: "baad_shared::ProgressStatus"
    },
    TypeMapping {
        shadow: "BaadSharedProgressEventKind",
        source: "crates/baad-shared/src/observer.rs",
        name: "ProgressEvent",
        native_path: "baad_shared::ProgressEvent"
    },
    TypeMapping {
        shadow: "BaadSharedHashKind",
        source: "crates/baad-shared/src/types.rs",
        name: "HashValue",
        native_path: "baad_shared::HashValue"
    }
];

pub(crate) const RECORDS: &[TypeMapping<'static>] = &[
    TypeMapping {
        shadow: "BaadUtilsAlignedLine",
        source: "crates/baad-utils/src/formatter/line.rs",
        name: "AlignedLine",
        native_path: "baad_utils::formatter::AlignedLine"
    },
    TypeMapping {
        shadow: "BaadUtilsLoggingConfig",
        source: "crates/baad-utils/src/logging/config.rs",
        name: "LoggingConfig",
        native_path: "baad_utils::config::LoggingConfig"
    },
    TypeMapping {
        shadow: "BaadDmZipFileInfo",
        source: "crates/baad-dm/src/zip/types.rs",
        name: "ZipFileInfo",
        native_path: "baad_dm::ZipFileInfo"
    },
    TypeMapping {
        shadow: "BaadDmDownloaderConfig",
        source: "crates/baad-dm/src/downloader/config.rs",
        name: "DownloaderConfig",
        native_path: "baad_dm::DownloaderConfig"
    },
    TypeMapping {
        shadow: "BaadDownloaderOptions",
        source: "crates/baad/src/download/downloader.rs",
        name: "ResourceDownloader",
        native_path: "baad_native::download::ResourceDownloader"
    }
];

pub(crate) const FAMILIES: &[(&str, &[&str])] = &[
    ("baad-shared", &["adapter", "error", "models", "observer", "progress", "runtime", "shared"]),
    ("baad-utils", &["adapter", "utils"]),
    ("baad-dm", &["adapter", "dm"]),
    ("baad", &["catalog", "cdn", "clients", "download", "download_adapter", "filter", "strategy"])
];
