use baad_ffi_build::{Config, RemoteKind, Sanitized, Source};

fn main() {
    baad_ffi_build::generate(&Config {
        c_prefix: "baad_dm",
        c_types_prefix: "BaadDm",
        custom_types: &[],
        observer: true,
        c_runtime: true,
        blocking_runtime: true,
        sources: &[
            Source {
                crate_path: "baad_dm",
                dir: "../../baad-dm/src",
                type_files: &[
                    ("download/hash.rs", RemoteKind::Data),
                    ("zip/types.rs", RemoteKind::Data),
                    ("error.rs", RemoteKind::Error)
                ],
                const_files: &[],
                renames: &[],
                sanitized: &[Sanitized {
                    file: "downloader/config.rs",
                    name: "DownloaderConfig",
                    skip_fields: &["headers", "observer"],
                    native: None
                }],
                skip_types: &[]
            },
            Source {
                crate_path: "baad_shared",
                dir: "../../baad-shared/src",
                type_files: &[("observer.rs", RemoteKind::Data)],
                const_files: &[],
                renames: &[],
                sanitized: &[],
                skip_types: &[]
            }
        ],
        reexports: &[]
    });
}
