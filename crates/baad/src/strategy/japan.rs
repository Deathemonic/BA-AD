use baad_shared::{
    BundlePatchPackInfo,
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    HashValue,
    MediaCatalog,
    Platform,
    TableCatalog
};

pub struct JapanStrategy;

impl JapanStrategy {
    pub fn build_asset_downloads(
        packing: BundlePatchPackInfo,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadAsset> {
        let packs = packing.full_patch_packs.into_iter().chain(packing.update_packs).map(|pack| {
            DownloadAsset {
                url: format!("{}/{}/{}", catalog_url, platform.patch_pack(), pack.pack_name),
                path: pack.pack_name,
                hash: HashValue::Crc(pack.crc),
                size: pack.pack_size,
                bundle_files: pack.bundle_files.into_iter().map(|b| b.name).collect()
            }
        });

        packs.collect()
    }

    pub fn build_media_downloads(
        catalog: MediaCatalog,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadMedia> {
        let media = catalog.table.into_values().map(|entry| {
            let path = entry.path.replace('\\', "/");

            DownloadMedia {
                url: format!("{}/{}/{}", catalog_url, platform.media_path(), path),
                path,
                hash: HashValue::Crc(entry.crc),
                size: entry.bytes
            }
        });

        media.collect()
    }

    pub fn build_table_downloads(catalog: TableCatalog, catalog_url: &str) -> Vec<DownloadTable> {
        let base = catalog.table.into_values().map(|entry| DownloadTable {
            url: format!("{}/TableBundles/{}", catalog_url, entry.name),
            path: entry.name,
            hash: HashValue::Crc(entry.crc),
            size: entry.size,
            bundle_files: Vec::new()
        });

        let packs = catalog.table_pack.into_values().map(|pack| DownloadTable {
            url: format!("{}/TableBundles/{}", catalog_url, pack.name),
            path: pack.name,
            hash: HashValue::Crc(pack.crc),
            size: pack.size,
            bundle_files: pack.bundle_files.into_iter().map(|b| b.name).collect()
        });

        base.chain(packs).collect()
    }
}
