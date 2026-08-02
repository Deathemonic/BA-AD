use baad_ffi_build::{Config, RemoteKind, Sanitized, Source};

fn main() {
    baad_ffi_build::generate(&Config {
        c_prefix: "baad_utils",
        c_types_prefix: "BaadUtils",
        custom_types: &[],
        observer: false,
        c_runtime: true,
        blocking_runtime: false,
        sources: &[Source {
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
        }],
        reexports: &[],
        handles: &[]
    });
}
