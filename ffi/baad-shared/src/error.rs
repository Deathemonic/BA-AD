use std::fmt::Error as FmtError;
use std::future::Future;
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::str::Utf8Error;

use reqwest::Error as HttpError;
use serde_json::Error as JsonError;

use crate::runtime::block_on;

#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;

    pub enum BaadErrorKind {
        InvalidArgument,
        Filter,
        Catalog,
        Download,
        File,
        Json,
        Network,
        Configuration,
        Runtime
    }

    #[diplomat::opaque]
    pub struct BaadError(pub String, pub BaadErrorKind);

    impl BaadError {
        pub fn message(&self, output: &mut DiplomatWrite) { let _ = output.write_str(&self.0); }

        pub const fn kind(&self) -> BaadErrorKind { self.1 }
    }
}
macro_rules! classify {
    ($native:ty, $kind:ident) => {
        impl From<$native> for ffi::BaadError {
            fn from(error: $native) -> Self { Self(error.to_string(), ffi::BaadErrorKind::$kind) }
        }
    };
}
classify!(baad_native::FilterError, Filter);
classify!(baad_native::CatalogError, Catalog);
classify!(baad_dm::Error, Download);
classify!(baad_utils::FileError, File);
classify!(baad_utils::JsonError, Json);
classify!(baad_utils::NetworkError, Network);
classify!(baad_utils::ConfigError, Configuration);
classify!(baad_utils::ProgressError, Configuration);
classify!(FmtError, InvalidArgument);
classify!(IoError, File);
classify!(baad_shared::ServerConfigError, Configuration);
classify!(HttpError, Network);
classify!(JsonError, Json);
classify!(Utf8Error, InvalidArgument);
classify!(ParseIntError, InvalidArgument);
classify!(String, InvalidArgument);
classify!(&str, InvalidArgument);

pub fn error(error: impl Into<ffi::BaadError>) -> Box<ffi::BaadError> { Box::new(error.into()) }

pub fn runtime_error(message: String) -> Box<ffi::BaadError> {
    Box::new(ffi::BaadError(message, ffi::BaadErrorKind::Runtime))
}

pub fn blocking<T, E: Into<ffi::BaadError>>(
    future: impl Future<Output = Result<T, E>>
) -> Result<T, Box<ffi::BaadError>> {
    block_on(future).map_err(runtime_error)?.map_err(error)
}
