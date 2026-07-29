use baad_ffi_build::{COW_STR, Config, RemoteKind, Source};

fn main() {
    baad_ffi_build::generate(&Config {
        c_prefix: "baad_shared",
        c_types_prefix: "BaadShared",
        custom_types: &[COW_STR],
        observer: true,
        c_runtime: false,
        blocking_runtime: false,
        sources: &[Source {
            crate_path: "baad_shared",
            dir: "../../baad-shared/src",
            type_files: &[
                ("types.rs", RemoteKind::Data),
                ("platform.rs", RemoteKind::Data),
                ("observer.rs", RemoteKind::Data),
                ("error.rs", RemoteKind::Error)
            ],
            const_files: &["consts.rs"],
            renames: &[],
            sanitized: &[],
            skip_types: &["MarketConfig"]
        }],
        reexports: &[]
    });
}
