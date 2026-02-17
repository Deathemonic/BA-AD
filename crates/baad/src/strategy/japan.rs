use baad_core::{
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
        packing: &BundlePatchPackInfo,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<DownloadAsset> {
        packing
            .full_patch_packs
            .iter()
            .chain(packing.update_packs.iter())
            .map(|pack| DownloadAsset {
                url: format!("{}/{}/{}", catalog_url, platform.patch_pack(), pack.pack_name),
                path: format!("AssetBundles/{}", pack.pack_name),
                hash: HashValue::Crc(pack.crc),
                size: pack.pack_size,
                bundle_files: pack.bundle_files.iter().map(|b| b.name.clone()).collect()
            })
            .collect()
    }

    pub fn build_media_downloads(catalog: &MediaCatalog, catalog_url: &str) -> Vec<DownloadMedia> {
        catalog
            .table
            .values()
            .map(|entry| {
                let path = entry.path.replace('\\', "/");
                DownloadMedia {
                    url: format!("{}/MediaResources/{}", catalog_url, path),
                    path: format!("MediaResources/{}", path),
                    hash: HashValue::Crc(entry.crc),
                    size: entry.bytes
                }
            })
            .collect()
    }

    pub fn build_table_downloads(catalog: &TableCatalog, catalog_url: &str) -> Vec<DownloadTable> {
        catalog
            .table
            .values()
            .map(|entry| DownloadTable {
                url: format!("{}/TableBundles/{}", catalog_url, entry.name),
                path: format!("TableBundles/{}", entry.name),
                hash: HashValue::Crc(entry.crc),
                size: entry.size
            })
            .collect()
    }
}
