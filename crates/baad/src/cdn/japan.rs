use std::path::Path;

use baad_core::{
    BundlePatchPackInfo,
    CatalogError,
    JapanAddressable,
    MediaCatalog,
    Platform,
    TableCatalog,
    client
};
use baad_utils::file::get_data_path;
use baad_utils::debug;
use memorypack::MemoryPackSerializer;
use tokio::fs;

use crate::download::{ResourceCategory, download_file};

pub struct JapanCdn {
    catalog_url: String,
    platform: Platform
}

pub struct JapanResources {
    pub assets: Option<BundlePatchPackInfo>,
    pub table: Option<TableCatalog>,
    pub media: Option<MediaCatalog>
}

impl JapanCdn {
    pub fn new(catalog_url: String, platform: Platform) -> Self {
        Self { catalog_url, platform }
    }

    pub async fn fetch_addressable(url: &str) -> Result<JapanAddressable, CatalogError> {
        let response = client().get(url).send().await?.json::<JapanAddressable>().await?;
        Ok(response)
    }

    pub fn extract_catalog_url(addressable: &JapanAddressable) -> Result<&str, CatalogError> {
        addressable
            .connection_groups
            .first()
            .and_then(|group| group.override_connection_groups.get(1))
            .map(|og| og.addressables_catalog_url_root.as_str())
            .ok_or(CatalogError::DeserializationFailed)
    }

    pub async fn fetch(
        &self,
        category: &[ResourceCategory]
    ) -> Result<JapanResources, CatalogError> {
        Ok(JapanResources {
            assets: if category.contains(&ResourceCategory::Assets) {
                Some(self.fetch_assets().await?)
            } else {
                None
            },
            table: if category.contains(&ResourceCategory::Tables) {
                Some(self.fetch_table().await?)
            } else {
                None
            },
            media: if category.contains(&ResourceCategory::Media) {
                Some(self.fetch_media().await?)
            } else {
                None
            }
        })
    }

    pub async fn fetch_assets(&self) -> Result<BundlePatchPackInfo, CatalogError> {
        let platform = self.platform.as_ref().to_lowercase();
        let url =
            format!("{}/{}/BundlePackingInfo.bytes", self.catalog_url, self.platform.patch_pack());
        let bytes =
            self.fetch_bytes(&url, &format!("{}/BundlePackingInfo.bytes", platform)).await?;
        let packing = MemoryPackSerializer::deserialize::<BundlePatchPackInfo>(&bytes)?;
        Ok(packing)
    }

    pub async fn fetch_table(&self) -> Result<TableCatalog, CatalogError> {
        let url = format!("{}/TableBundles/TableCatalog.bytes", self.catalog_url);
        let bytes = self.fetch_bytes(&url, "TableCatalog.bytes").await?;
        let catalog = MemoryPackSerializer::deserialize::<TableCatalog>(&bytes)?;
        Ok(catalog)
    }

    pub async fn fetch_media(&self) -> Result<MediaCatalog, CatalogError> {
        let url = format!("{}/MediaResources/Catalog/MediaCatalog.bytes", self.catalog_url);
        let bytes = self.fetch_bytes(&url, "MediaCatalog.bytes").await?;
        let catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(&bytes)?;
        Ok(catalog)
    }

    async fn fetch_bytes(&self, url: &str, filename: &str) -> Result<Vec<u8>, CatalogError> {
        let path = get_data_path(&format!("catalog/japan/{filename}"))?;
        let hash_path = path.with_extension("hash");

        let remote = Self::remote_hash(&Self::hash_url(url)).await;
        let local = Self::local_hash(&hash_path).await;

        let outdated = !path.exists()
            || match (&remote, &local) {
                (Some(remote), Some(local)) => remote != local,
                (Some(_), None) => true,
                (None, _) => false
            };

        if outdated {
            debug!(filename, "Catalog outdated, downloading");
            download_file(url, &path, None, 3).await?;
            if let Some(remote) = &remote {
                fs::write(&hash_path, remote).await?;
            }
        } else {
            debug!(filename, "Catalog up to date, using cache");
        }

        let bytes = fs::read(&path).await?;
        Ok(bytes)
    }

    fn hash_url(url: &str) -> String {
        url.strip_suffix(".bytes").map_or_else(|| url.to_string(), |stem| format!("{stem}.hash"))
    }

    async fn remote_hash(url: &str) -> Option<String> {
        let response = client().get(url).send().await.ok()?.error_for_status().ok()?;
        let text = response.text().await.ok()?;
        Some(text.trim().to_string())
    }

    async fn local_hash(path: &Path) -> Option<String> {
        fs::read_to_string(path).await.ok().map(|hash| hash.trim().to_string())
    }
}
