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
use baad_utils::file::filename_or;
use fastcat::fconcat;

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

            let url = fconcat!(base_url, resource.resource_path.as_str());
            let hash = HashValue::Md5(resource.resource_hash);
            let size = resource.resource_size;

            if resource.resource_path.contains(TABLE_BUNDLES) {
                if category.contains(ResourceCategory::Tables) {
                    tables.push(DownloadTable {
                        url,
                        path: filename_or(&resource.resource_path).into(),
                        hash,
                        size,
                        bundle_files: Vec::new()
                    });
                }
            } else if resource.resource_path.contains(MEDIA_RESOURCES) {
                if category.contains(ResourceCategory::Media) {
                    media.push(DownloadMedia {
                        url,
                        path: filename_or(&resource.resource_path).into(),
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
