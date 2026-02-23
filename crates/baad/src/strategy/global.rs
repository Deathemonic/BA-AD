use std::path::Path;

use baad_core::{
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    Downloads,
    HashValue,
    MEDIA_RESOURCES,
    Resource,
    TABLE_BUNDLES
};

const CATALOG_PREFIX: &str = "Catalog/";

pub struct GlobalStrategy;

impl GlobalStrategy {
    pub fn build_downloads(resources: &[Resource], base_url: &str) -> Downloads {
        let mut assets = Vec::new();
        let mut tables = Vec::new();
        let mut media = Vec::new();

        for resource in resources {
            if resource.resource_path.starts_with(CATALOG_PREFIX) {
                continue;
            }

            let url = format!("{}{}", base_url, resource.resource_path);
            let hash = HashValue::Md5(resource.resource_hash.clone());
            let size = resource.resource_size;

            if resource.resource_path.contains(TABLE_BUNDLES) {
                let filename = Path::new(&resource.resource_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(&resource.resource_path)
                    .to_string();

                tables.push(DownloadTable {
                    url,
                    path: filename,
                    hash,
                    size
                });
            } else if resource.resource_path.contains(MEDIA_RESOURCES) {
                let filename = Path::new(&resource.resource_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(&resource.resource_path)
                    .to_string();

                media.push(DownloadMedia {
                    url,
                    path: filename,
                    hash,
                    size
                });
            } else {
                assets.push(DownloadAsset {
                    url,
                    path: resource.resource_path.clone(),
                    hash,
                    size,
                    bundle_files: Vec::new()
                });
            }
        }

        Downloads { assets, tables, media }
    }
}
