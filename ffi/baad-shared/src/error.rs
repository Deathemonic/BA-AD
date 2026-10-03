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
classify!(std::fmt::Error, InvalidArgument);
classify!(std::io::Error, File);
classify!(baad_shared::ServerConfigError, Configuration);
classify!(reqwest::Error, Network);
classify!(serde_json::Error, Json);
classify!(std::str::Utf8Error, InvalidArgument);
classify!(std::num::ParseIntError, InvalidArgument);
classify!(String, InvalidArgument);
classify!(&str, InvalidArgument);

pub fn error(error: impl Into<ffi::BaadError>) -> Box<ffi::BaadError> { Box::new(error.into()) }

pub fn runtime_error(message: String) -> Box<ffi::BaadError> {
    Box::new(ffi::BaadError(message, ffi::BaadErrorKind::Runtime))
}

pub fn blocking<T, E: Into<ffi::BaadError>>(
    future: impl std::future::Future<Output = Result<T, E>>
) -> Result<T, Box<ffi::BaadError>> {
    crate::runtime::block_on(future).map_err(runtime_error)?.map_err(error)
}
