use baad_core::{BundlePatchPackInfo, HashValue, Platform};

pub struct AssetDownload {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64,
    pub bundle_files: Vec<String>
}

pub struct AssetStrategy;

impl AssetStrategy {
    pub fn build_downloads(
        packing: &BundlePatchPackInfo,
        catalog_url: &str,
        platform: Platform
    ) -> Vec<AssetDownload> {
        packing
            .full_patch_packs
            .iter()
            .chain(packing.update_packs.iter())
            .map(|pack| AssetDownload {
                url: format!("{}/{}/{}", catalog_url, platform.patch_pack(), pack.pack_name),
                path: format!("AssetBundles/{}", pack.pack_name),
                hash: HashValue::Crc(pack.crc),
                size: pack.pack_size,
                bundle_files: pack.bundle_files.iter().map(|b| b.name.clone()).collect()
            })
            .collect()
    }
}
