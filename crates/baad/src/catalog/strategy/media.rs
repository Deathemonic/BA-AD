use crate::helpers::{HashValue, MediaCatalog};

pub struct MediaDownload {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64,
}

pub struct MediaStrategy;

impl MediaStrategy {
    pub fn build_downloads(catalog: &MediaCatalog, catalog_url: &str) -> Vec<MediaDownload> {
        catalog
            .table
            .values()
            .map(|entry| {
                let path = entry.path.replace('\\', "/");
                MediaDownload {
                    url: format!("{}/MediaResources/{}", catalog_url, path),
                    path: format!("MediaResources/{}", path),
                    hash: HashValue::Crc(entry.crc),
                    size: entry.bytes,
                }
            })
            .collect()
    }
}

