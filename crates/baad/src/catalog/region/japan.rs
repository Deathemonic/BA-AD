use crate::helpers::{
    BundlePatchPackInfo, CatalogError, JapanAddressable, MediaCatalog, Platform, TableCatalog,
    PATCH_PACK_ANDROID, PATCH_PACK_IOS,
};

use memorypack::MemoryPackSerializer;
use reqwest::Client;

pub struct JapanCatalog {
    client: Client,
    catalog_url: String,
    platform: Platform,
}

pub struct JapanResources {
    pub packing: BundlePatchPackInfo,
    pub table: TableCatalog,
    pub media: MediaCatalog,
}

impl JapanCatalog {
    pub fn new(catalog_url: String, platform: Platform) -> Self {
        Self {
            client: Client::new(),
            catalog_url,
            platform,
        }
    }

    pub fn with_client(client: Client, catalog_url: String, platform: Platform) -> Self {
        Self {
            client,
            catalog_url,
            platform,
        }
    }

    fn patch_pack_path(&self) -> &'static str {
        match self.platform {
            Platform::Android => PATCH_PACK_ANDROID,
            Platform::Ios => PATCH_PACK_IOS,
        }
    }

    pub async fn fetch_addressable(client: &Client, url: &str) -> Result<JapanAddressable, CatalogError> {
        let response = client.get(url).send().await?.json::<JapanAddressable>().await?;
        Ok(response)
    }

    pub fn extract_catalog_url(addressable: &JapanAddressable) -> Option<&str> {
        addressable
            .connection_groups
            .first()
            .and_then(|group| group.override_connection_groups.get(1))
            .map(|og| og.addressables_catalog_url_root.as_str())
    }

    pub async fn fetch(&self) -> Result<JapanResources, CatalogError> {
        let packing = self.fetch_packing().await?;
        let table = self.fetch_table().await?;
        let media = self.fetch_media().await?;

        Ok(JapanResources {
            packing,
            table,
            media,
        })
    }

    pub async fn fetch_packing(&self) -> Result<BundlePatchPackInfo, CatalogError> {
        let url = format!(
            "{}/{}/BundlePackingInfo.bytes",
            self.catalog_url,
            self.patch_pack_path()
        );
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
        let url = format!(
            "{}/MediaResources/Catalog/MediaCatalog.bytes",
            self.catalog_url
        );
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        let catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(&bytes)?;
        Ok(catalog)
    }

    pub fn catalog_url(&self) -> &str {
        &self.catalog_url
    }
}

