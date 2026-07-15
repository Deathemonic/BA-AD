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
use fastcat::fconcat;

pub struct JapanStrategy;

impl JapanStrategy {
    pub fn build_asset_downloads(
        packing: BundlePatchPackInfo,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadAsset> {
        let packs = packing.full_patch_packs.into_iter().chain(packing.update_packs).map(|pack| {
            DownloadAsset {
                url: fconcat!("/"; catalog_url, platform.patch_pack(), pack.pack_name.as_str()),
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
                url: fconcat!("/"; catalog_url, platform.media_path(), path.as_str()),
                path,
                hash: HashValue::Crc(entry.crc),
                size: entry.bytes,
                target: None
            }
        });

        media.collect()
    }

    pub fn build_table_downloads(catalog: TableCatalog, catalog_url: &str) -> Vec<DownloadTable> {
        let base = catalog.table.into_values().map(|entry| DownloadTable {
            url: fconcat!("/"; catalog_url, "TableBundles", entry.name.as_str()),
            path: entry.name,
            hash: HashValue::Crc(entry.crc),
            size: entry.size,
            bundle_files: Vec::new()
        });

        let packs = catalog.table_pack.into_values().map(|pack| DownloadTable {
            url: fconcat!("/"; catalog_url, "TableBundles", pack.name.as_str()),
            path: pack.name,
            hash: HashValue::Crc(pack.crc),
            size: pack.size,
            bundle_files: pack.bundle_files.into_iter().map(|b| b.name).collect()
        });

        base.chain(packs).collect()
    }
}
