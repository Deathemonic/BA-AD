use proc_macro2::TokenStream;
use syn::{ItemEnum, ItemStruct};

use crate::consts::GeneratedConst;

pub(crate) enum RemoteItem {
    Record(ItemStruct),
    Enum(ItemEnum),
    Error(ItemEnum)
}

pub(crate) struct Remote {
    pub(crate) crate_path: String,
    pub(crate) source: String,
    pub(crate) item: RemoteItem
}

pub(crate) struct ErrorMirror {
    pub(crate) crate_path: String,
    pub(crate) item: ItemEnum
}

pub(crate) struct DataMirror {
    pub(crate) crate_path: String,
    pub(crate) item: ItemEnum
}

pub(crate) struct ErrorCode {
    pub(crate) crate_path: String,
    pub(crate) item: ItemEnum
}

#[derive(Default)]
pub(crate) struct Context {
    pub(crate) remotes: Vec<Remote>,
    pub(crate) data_mirrors: Vec<DataMirror>,
    pub(crate) error_mirrors: Vec<ErrorMirror>,
    pub(crate) error_codes: Vec<ErrorCode>,
    pub(crate) sanitized: Vec<TokenStream>,
    pub(crate) consts: Vec<GeneratedConst>,
    pub(crate) mirrored_names: Vec<String>
}

pub(crate) fn remote_name(item: &RemoteItem) -> String {
    match item {
        RemoteItem::Record(item) => item.ident.to_string(),
        RemoteItem::Enum(item) | RemoteItem::Error(item) => item.ident.to_string()
    }
}
