use baad_core::{
    ASSET_BUNDLES,
    ChinaBundleCatalog,
    ChinaTableCatalog,
    DownloadAsset,
    DownloadTable,
    HashValue,
    Platform,
    TABLE_BUNDLES
};

pub struct ChinaStrategy;

impl ChinaStrategy {
    pub fn build_asset_downloads(
        catalog: ChinaBundleCatalog,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadAsset> {
        let assets = catalog.bundle_files.into_iter().map(|file| DownloadAsset {
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
        });

        assets.collect()
    }

    pub fn build_table_downloads(catalog: ChinaTableCatalog, catalog_url: &str) -> Vec<DownloadTable> {
        let tables = catalog.table.into_values().map(|entry| DownloadTable {
            url: format!("{}/{}/{}", catalog_url, TABLE_BUNDLES, entry.name),
            path: entry.name,
            hash: HashValue::Md5(entry.crc),
            size: entry.size,
            bundle_files: entry.includes.unwrap_or_default()
        });

        tables.collect()
    }
}
