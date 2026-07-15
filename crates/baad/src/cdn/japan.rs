use baad_shared::{
    BundlePatchPackInfo,
    JapanAddressable,
    MEDIA_RESOURCES,
    MediaCatalog,
    Platform,
    TABLE_BUNDLES,
    TableCatalog,
    client
};
use baad_utils::file::get_data_path;
use fastcat::fconcat;
use memorypack::MemoryPackSerializer;
use tokio::fs;
use tracing::trace;
use crate::cdn::cache;
use crate::cdn::cache::CatalogFile;
use crate::download::ResourceCategory;
use crate::error::CatalogError;

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
    pub fn new(catalog_url: String, platform: Platform) -> Self { Self { catalog_url, platform } }

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

    pub async fn fetch(&self, category: ResourceCategory) -> Result<JapanResources, CatalogError> {
        Ok(JapanResources {
            assets: if category.contains(ResourceCategory::Assets) {
                Some(self.fetch_assets().await?)
            } else {
                None
            },
            table: if category.contains(ResourceCategory::Tables) {
                Some(self.fetch_table().await?)
            } else {
                None
            },
            media: if category.contains(ResourceCategory::Media) {
                Some(self.fetch_media().await?)
            } else {
                None
            }
        })
    }

    pub async fn fetch_assets(&self) -> Result<BundlePatchPackInfo, CatalogError> {
        let platform = self.platform.as_ref().to_lowercase();
        let url = fconcat!("/"; self.catalog_url.as_str(), self.platform.patch_pack(), "BundlePackingInfo.bytes");
        let bytes = self
            .fetch_bytes(&url, &fconcat!(platform.as_str(), "/BundlePackingInfo.bytes"))
            .await?;
        let catalog = MemoryPackSerializer::deserialize::<BundlePatchPackInfo>(&bytes)?;
        Ok(catalog)
    }

    pub async fn fetch_table(&self) -> Result<TableCatalog, CatalogError> {
        let url =
            fconcat!("/"; self.catalog_url.as_str(), const { TABLE_BUNDLES }, "TableCatalog.bytes");
        let bytes = self.fetch_bytes(&url, "TableCatalog.bytes").await?;
        let catalog = MemoryPackSerializer::deserialize::<TableCatalog>(&bytes)?;
        Ok(catalog)
    }

    pub async fn fetch_media(&self) -> Result<MediaCatalog, CatalogError> {
        let url = fconcat!("/"; self.catalog_url.as_str(), const { MEDIA_RESOURCES }, "Catalog", "MediaCatalog.bytes");
        let bytes = self.fetch_bytes(&url, "MediaCatalog.bytes").await?;
        let catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(&bytes)?;
        Ok(catalog)
    }

    async fn fetch_bytes(&self, url: &str, filename: &str) -> Result<Vec<u8>, CatalogError> {
        let file = CatalogFile {
            url: url.into(),
            hash_url: Self::hash_url(url),
            path: get_data_path(&fconcat!("/"; "catalog", "japan", filename))?
        };
        cache::ensure_cached(&file).await?;
        Ok(fs::read(&file.path).await?)
    }

    fn hash_url(url: &str) -> String {
        url.strip_suffix(".bytes").map_or_else(|| url.into(), |stem| fconcat!(stem, ".hash"))
    }
}
