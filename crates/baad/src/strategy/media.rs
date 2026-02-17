use baad_core::{HashValue, MediaCatalog};

pub struct Media {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64
}

pub struct MediaStrategy;

impl MediaStrategy {
    pub fn build_downloads(catalog: &MediaCatalog, catalog_url: &str) -> Vec<Media> {
        catalog
            .table
            .values()
            .map(|entry| {
                let path = entry.path.replace('\\', "/");
                Media {
                    url: format!("{}/MediaResources/{}", catalog_url, path),
                    path: format!("MediaResources/{}", path),
                    hash: HashValue::Crc(entry.crc),
                    size: entry.bytes
                }
            })
            .collect()
    }
}
