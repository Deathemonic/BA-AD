use baad_shared::{
    ASSET_BUNDLES,
    ChinaBundleCatalog,
    ChinaMedia,
    ChinaTableCatalog,
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    HashValue,
    MEDIA_RESOURCES,
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

    pub fn build_media_downloads(
        catalog: Vec<ChinaMedia>,
        catalog_url: &str
    ) -> Vec<DownloadMedia> {
        let entries = catalog.into_iter().map(|entry| {
            let url = Self::hashed_url(catalog_url, &entry.crc, MEDIA_RESOURCES);

            DownloadMedia {
                url,
                path: entry.path,
                hash: HashValue::Md5(entry.crc),
                size: entry.bytes
            }
        });

        entries.collect()
    }

    pub fn build_table_downloads(
        catalog: ChinaTableCatalog,
        catalog_url: &str
    ) -> Vec<DownloadTable> {
        let tables = catalog.table.into_values().map(|entry| {
            let url = Self::hashed_url(catalog_url, &entry.crc, TABLE_BUNDLES);

            DownloadTable {
                url,
                path: entry.name,
                hash: HashValue::Md5(entry.crc),
                size: entry.size,
                bundle_files: entry.includes.unwrap_or_default()
            }
        });

        tables.collect()
    }

    fn hashed_url(catalog_url: &str, hash: &str, resource: &str) -> String {
        let shard = hash.get(..2).unwrap_or(hash);
        format!("{}/pool/{}/{}/{}", catalog_url, resource, shard, hash)
    }
}
