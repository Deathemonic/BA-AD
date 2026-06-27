use baad_core::{ASSET_BUNDLES, ChinaBundleCatalog, DownloadAsset, HashValue, Platform};

pub struct ChinaStrategy;

impl ChinaStrategy {
    pub fn build_asset_downloads(
        catalog: ChinaBundleCatalog,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadAsset> {
        let assets = catalog.bundle_files.into_iter().map(|file| {
            DownloadAsset {
                url: format!(
                    "{}/{}/{}/{}",
                    catalog_url,
                    ASSET_BUNDLES,
                    platform.display_name(),
                    file.name
                ),
                path: file.name,
                hash: HashValue::Md5(file.crc),
                size: file.size,
                bundle_files: Vec::new()
            }
        });

        assets.collect()
    }
}
