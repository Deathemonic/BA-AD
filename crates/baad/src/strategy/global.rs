use baad_core::{HashValue, Resource};

use crate::catalog::Downloads;
use crate::strategy::{AssetDownload, MediaDownload, TableDownload};

const TABLE_BUNDLES_PREFIX: &str = "TableBundles/";
const MEDIA_RESOURCES_PREFIX: &str = "MediaResources/";
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

            if resource.resource_path.contains(TABLE_BUNDLES_PREFIX) {
                tables.push(TableDownload {
                    url,
                    path: resource.resource_path.clone(),
                    hash,
                    size
                });
            } else if resource.resource_path.contains(MEDIA_RESOURCES_PREFIX) {
                media.push(MediaDownload {
                    url,
                    path: resource.resource_path.clone(),
                    hash,
                    size
                });
            } else {
                assets.push(AssetDownload {
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
