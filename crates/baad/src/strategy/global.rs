use std::path::Path;

use baad_shared::{
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    Downloads,
    HashValue,
    MEDIA_RESOURCES,
    Resource,
    TABLE_BUNDLES
};

use crate::download::ResourceCategory;

const CATALOG_PREFIX: &str = "Catalog/";

pub struct GlobalStrategy;

impl GlobalStrategy {
    pub fn build_downloads(
        resources: Vec<Resource>,
        base_url: &str,
        category: ResourceCategory
    ) -> Downloads {
        let mut assets = Vec::new();
        let mut tables = Vec::new();
        let mut media = Vec::new();

        for resource in resources {
            if resource.resource_path.starts_with(CATALOG_PREFIX) {
                continue;
            }

            let url = format!("{}{}", base_url, resource.resource_path);
            let hash = HashValue::Md5(resource.resource_hash);
            let size = resource.resource_size;

            if resource.resource_path.contains(TABLE_BUNDLES) {
                if category.contains(ResourceCategory::Tables) {
                    let filename = Path::new(&resource.resource_path)
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or(&resource.resource_path)
                        .into();

                    tables.push(DownloadTable {
                        url,
                        path: filename,
                        hash,
                        size,
                        bundle_files: Vec::new()
                    });
                }
            } else if resource.resource_path.contains(MEDIA_RESOURCES) {
                if category.contains(ResourceCategory::Media) {
                    let filename = Path::new(&resource.resource_path)
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or(&resource.resource_path)
                        .into();

                    media.push(DownloadMedia {
                        url,
                        path: filename,
                        hash,
                        size
                    });
                }
            } else if category.contains(ResourceCategory::Assets) {
                assets.push(DownloadAsset {
                    url,
                    path: resource.resource_path,
                    hash,
                    size,
                    bundle_files: Vec::new()
                });
            }
        }

        Downloads { assets, tables, media }
    }
}
