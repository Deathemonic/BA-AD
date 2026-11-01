use std::str::from_utf8;

use crate::error::error;
use crate::error::ffi::BaadError;

#[diplomat::bridge]
pub mod ffi {
    use crate::adapter::parse_json;
    use crate::error::error;
    use crate::error::ffi::BaadError;
    use crate::models::{hash as make_hash, strings};

    pub enum BaadSharedHashKind {}

    impl BaadSharedGlobalCatalogData {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }

        pub fn new() -> Box<Self> { Box::new(Self(baad_shared::GlobalCatalog::default())) }

        pub fn push_resource(&mut self, group: &str, path: &str, size: i64, hash: &str) {
            self.0.resources.push(baad_shared::Resource {
                group: group.to_owned().into(),
                resource_path: path.to_owned(),
                resource_size: size,
                resource_hash: hash.to_owned()
            });
        }
    }

    impl BaadSharedTableCatalogCN {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedBundleCatalogCN {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedMediaCatalogCN {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedMediaCatalog {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedTableCatalog {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedBundlePatchPackInfo {
        pub fn parse_json(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            parse_json(json).map(|value| Box::new(Self(value))).map_err(error)
        }
    }

    impl BaadSharedDownloads {
        pub fn new() -> Box<Self> {
            Box::new(Self(baad_shared::Downloads {
                assets: Vec::new(),
                tables: Vec::new(),
                media: Vec::new()
            }))
        }

        pub fn append(&mut self, other: &mut Self) {
            self.0.assets.append(&mut other.0.assets);
            self.0.tables.append(&mut other.0.tables);
            self.0.media.append(&mut other.0.media);
        }

        pub fn push_asset(
            &mut self,
            url: &str,
            path: &str,
            hash_kind: BaadSharedHashKind,
            hash: &str,
            size: i64,
            files: &[DiplomatStrSlice]
        ) -> Result<(), Box<BaadError>> {
            self.0.assets.push(baad_shared::DownloadAsset {
                url: url.into(),
                path: path.into(),
                hash: make_hash(hash_kind, hash)?,
                size,
                bundle_files: strings(files)?
            });
            Ok(())
        }

        pub fn push_table(
            &mut self,
            url: &str,
            path: &str,
            hash_kind: BaadSharedHashKind,
            hash: &str,
            size: i64,
            files: &[DiplomatStrSlice]
        ) -> Result<(), Box<BaadError>> {
            self.0.tables.push(baad_shared::DownloadTable {
                url: url.into(),
                path: path.into(),
                hash: make_hash(hash_kind, hash)?,
                size,
                bundle_files: strings(files)?
            });
            Ok(())
        }

        pub fn push_media(
            &mut self,
            url: &str,
            path: &str,
            hash_kind: BaadSharedHashKind,
            hash: &str,
            size: i64
        ) -> Result<(), Box<BaadError>> {
            self.0.media.push(baad_shared::DownloadMedia {
                url: url.into(),
                path: path.into(),
                hash: make_hash(hash_kind, hash)?,
                size
            });
            Ok(())
        }
    }

    #[diplomat::opaque]
    pub struct BaadSharedStrings(pub Vec<String>);

    impl BaadSharedStrings {
        pub const fn is_empty(&self) -> bool { self.0.is_empty() }

        pub const fn len(&self) -> usize { self.0.len() }

        pub fn get<'a>(&'a self, index: usize) -> Option<&'a str> {
            self.0.get(index).map(String::as_str)
        }
    }
}

fn strings(
    values: &[diplomat_runtime::DiplomatStrSlice<'_>]
) -> Result<Vec<String>, Box<BaadError>> {
    values
        .iter()
        .map(|value| from_utf8(value).map(str::to_owned))
        .collect::<Result<_, _>>()
        .map_err(error)
}

fn write_hash(hash: &baad_shared::HashValue, output: &mut diplomat_runtime::DiplomatWrite) {
    use std::fmt::Write;
    let _ = output.write_str(&hash.as_string());
}
