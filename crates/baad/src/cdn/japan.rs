use baad_core::{
    BundlePatchPackInfo,
    CatalogError,
    JapanAddressable,
    MediaCatalog,
    Platform,
    TableCatalog
};
use memorypack::MemoryPackSerializer;
use reqwest::Client;

pub struct JapanCdn {
    client: Client,
    catalog_url: String,
    platform: Platform
}

pub struct JapanResources {
    pub packing: BundlePatchPackInfo,
    pub table: TableCatalog,
    pub media: MediaCatalog
}

impl JapanCdn {
    pub fn new(catalog_url: String, platform: Platform) -> Result<Self, CatalogError> {
        Self::with_client(Client::new(), catalog_url, platform)
    }

    pub fn with_client(
        client: Client,
        catalog_url: String,
        platform: Platform
    ) -> Result<Self, CatalogError> {
        Ok(Self {
            client,
            catalog_url,
            platform
        })
    }

    pub async fn fetch_addressable(
        client: &Client,
        url: &str
    ) -> Result<JapanAddressable, CatalogError> {
        let response = client.get(url).send().await?.json::<JapanAddressable>().await?;
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
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        let packing = MemoryPackSerializer::deserialize::<BundlePatchPackInfo>(&bytes)?;
        Ok(packing)
    }

    pub async fn fetch_table(&self) -> Result<TableCatalog, CatalogError> {
        let url = format!("{}/TableBundles/TableCatalog.bytes", self.catalog_url);
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        let catalog = MemoryPackSerializer::deserialize::<TableCatalog>(&bytes)?;
        Ok(catalog)
    }

    pub async fn fetch_media(&self) -> Result<MediaCatalog, CatalogError> {
        let url = format!("{}/MediaResources/Catalog/MediaCatalog.bytes", self.catalog_url);
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        let catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(&bytes)?;
        Ok(catalog)
    }
}
