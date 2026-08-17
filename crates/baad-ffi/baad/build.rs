use baad_ffi_build::{
    COW_STR,
    Config,
    CustomType,
    Handle,
    Reexport,
    RemoteKind,
    Rename,
    Sanitized,
    Source
};

const CUSTOM_TYPES: &[CustomType] = &[COW_STR];

const HANDLES: &[Handle] = &[
    Handle {
        name: "ResourceFilter",
        native: "std::sync::Mutex<baad::download::ResourceFilter>"
    },
    Handle {
        name: "Downloads",
        native: "baad_shared::Downloads"
    },
    Handle {
        name: "JapanCatalog",
        native: "baad::catalog::JapanCatalog"
    },
    Handle {
        name: "GlobalCatalog",
        native: "baad::catalog::GlobalCatalog"
    },
    Handle {
        name: "ChinaCatalog",
        native: "baad::catalog::ChinaCatalog"
    },
    Handle {
        name: "JapanCdn",
        native: "baad::cdn::JapanCdn"
    },
    Handle {
        name: "GlobalCdn",
        native: "baad::cdn::GlobalCdn"
    },
    Handle {
        name: "ChinaCdn",
        native: "baad::cdn::ChinaCdn"
    }
];

const SOURCES: &[Source] = &[
    Source {
        crate_path: "baad",
        dir: "../../baad/src",
        type_files: &[("error.rs", RemoteKind::Error)],
        const_files: &[],
        renames: &[],
        sanitized: &[],
        skip_types: &[]
    },
    Source {
        crate_path: "baad::download",
        dir: "../../baad/src",
        type_files: &[("download/filter.rs", RemoteKind::Data)],
        const_files: &[],
        renames: &[],
        sanitized: &[],
        skip_types: &["ResourceFilter"]
    },
    Source {
        crate_path: "baad_shared",
        dir: "../../baad-shared/src",
        type_files: &[
            ("types.rs", RemoteKind::Data),
            ("platform.rs", RemoteKind::Data),
            ("observer.rs", RemoteKind::Data),
            ("error.rs", RemoteKind::Error)
        ],
        const_files: &["consts.rs"],
        renames: &[Rename {
            source: "GlobalCatalog",
            target: "GlobalCatalogData"
        }],
        sanitized: &[],
        skip_types: &["MarketConfig"]
    },
    Source {
        crate_path: "baad_utils",
        dir: "../../baad-utils/src",
        type_files: &[("error.rs", RemoteKind::Error)],
        const_files: &[],
        renames: &[],
        sanitized: &[Sanitized {
            file: "logging/config.rs",
            name: "LoggingConfig",
            skip_fields: &[],
            native: Some("baad_utils::config::LoggingConfig")
        }],
        skip_types: &[]
    }
];

const REEXPORTS: &[Reexport] = &[
    Reexport {
        name: "baad_utils",
        dir: "../baad-utils/src/api",
        c_prefix: "baad_utils",
        c_types_prefix: "BaadUtils",
        skip: &[
            "json_load",
            "json_save",
            "format_bytes",
            "terminal_is_terminal",
            "terminal_size",
            "TerminalSize"
        ]
    },
    Reexport {
        name: "baad_shared",
        dir: "../baad-shared/src/api",
        c_prefix: "baad_shared",
        c_types_prefix: "BaadShared",
        skip: &[]
    }
];

fn main() {
    baad_ffi_build::generate(&Config {
        c_prefix: "baad",
        c_types_prefix: "Baad",
        custom_types: CUSTOM_TYPES,
        observer: true,
        c_runtime: true,
        blocking_runtime: true,
        handles: HANDLES,
        sources: SOURCES,
        reexports: REEXPORTS
    });
}
