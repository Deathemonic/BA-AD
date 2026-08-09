mod c_api;
mod collect;
mod config;
mod consts;
mod context;
mod mirror;
mod observer;
mod reexport;
mod sanitize;
mod syntax;
mod uniffi;

use std::env;
use std::path::PathBuf;

pub use config::{
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

use crate::c_api::render_c;
use crate::collect::collect_source;
use crate::context::Context;
use crate::observer::render_observer_dispatch;
use crate::reexport::render_reexport;
use crate::syntax::write_rust;
use crate::uniffi::render_uniffi;

pub fn generate(config: &Config) {
    let mut context = Context::default();

    for source in config.sources {
        collect_source(config, source, &mut context);
    }

    let mut uniffi_stream = render_uniffi(config, &context);
    let mut c_stream = render_c(config, &context);

    for reexport in config.reexports {
        uniffi_stream.extend(render_reexport(config, reexport, "uniffi_api.rs", true));
        c_stream.extend(render_reexport(config, reexport, "c_api.rs", false));
    }

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|error| panic!("OUT_DIR: {error}")));
    write_rust(&out_dir.join("shadow.rs"), uniffi_stream);
    write_rust(&out_dir.join("c_shadow.rs"), c_stream);

    if config.observer {
        write_rust(&out_dir.join("observer_shadow.rs"), render_observer_dispatch());
    }
}
