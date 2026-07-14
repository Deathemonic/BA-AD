use baad_shared::{
    ASSET_BUNDLES,
    BundleCatalogCN,
    ChinaMediaType,
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    HashValue,
    MEDIA_RESOURCES,
    MediaCatalogCN,
    Platform,
    TABLE_BUNDLES,
    TableCatalogCN
};

pub struct ChinaStrategy;

impl ChinaStrategy {
    pub fn build_asset_downloads(
        catalog: BundleCatalogCN,
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
        catalog: MediaCatalogCN,
        catalog_url: &str,
        apk_url: &str
    ) -> Vec<DownloadMedia> {
        let media = catalog.table.into_values().map(|entry| DownloadMedia {
            url: Self::hashed_url(catalog_url, &entry.hash, MEDIA_RESOURCES),
            path: Self::media_path(entry.path, entry.media_type),
            hash: HashValue::Md5(entry.hash),
            size: entry.size
        });

        let packs = catalog.table_pack.into_values().map(|pack| DownloadMedia {
            url: apk_url.into(),
            path: Self::media_path(pack.path, pack.media_type),
            hash: HashValue::Md5(pack.hash),
            size: pack.size
        });

        media.chain(packs).collect()
    }

    pub fn build_table_downloads(catalog: TableCatalogCN, catalog_url: &str) -> Vec<DownloadTable> {
        let tables = catalog.table.into_values().map(|entry| DownloadTable {
            url: Self::hashed_url(catalog_url, &entry.crc, TABLE_BUNDLES),
            path: entry.name,
            hash: HashValue::Md5(entry.crc),
            size: entry.size,
            bundle_files: entry.includes.unwrap_or_default()
        });

        tables.collect()
    }

    fn hashed_url(catalog_url: &str, hash: &str, resource: &str) -> String {
        let shard = hash.get(..2).unwrap_or(hash);
        format!("{catalog_url}/pool/{resource}/{shard}/{hash}")
    }

    fn media_path(path: String, media_type: ChinaMediaType) -> String {
        let extension = media_type.as_ref();
        if extension.is_empty() { path } else { format!("{path}.{extension}") }
    }
}
