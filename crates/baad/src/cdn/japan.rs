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
use memorypack::MemoryPackSerializer;
use tokio::fs;

use crate::download::download_file;

pub struct JapanCdn {
    catalog_url: String,
    platform: Platform
}

pub struct JapanResources {
    pub packing: BundlePatchPackInfo,
    pub table: TableCatalog,
    pub media: MediaCatalog
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

    pub async fn fetch(&self) -> Result<JapanResources, CatalogError> {
        let packing = self.fetch_packing().await?;
        let table = self.fetch_table().await?;
        let media = self.fetch_media().await?;

        Ok(JapanResources { packing, table, media })
    }

    pub async fn fetch_packing(&self) -> Result<BundlePatchPackInfo, CatalogError> {
        let url =
            format!("{}/{}/BundlePackingInfo.bytes", self.catalog_url, self.platform.patch_pack());
        let bytes = self.fetch_bytes(&url, "BundlePackingInfo.bytes").await?;
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
        download_file(url, &path, None, 3).await?;
        let bytes = fs::read(&path).await?;
        Ok(bytes)
    }
}
