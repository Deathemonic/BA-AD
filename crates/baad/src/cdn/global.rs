use baad_core::{CatalogError, GlobalCatalog as GlobalCatalogData, Platform};
use reqwest::Client;

pub struct GlobalCdn {
    client: Client,
    catalog_url: String,
    platform: Platform
}

impl GlobalCdn {
    pub fn new(catalog_url: String, platform: Platform) -> Self {
        Self {
            client: Client::new(),
            catalog_url,
            platform
        }
    }

    pub fn with_client(client: Client, catalog_url: String, platform: Platform) -> Self {
        Self {
            client,
            catalog_url,
            platform
        }
    }

    pub async fn fetch(&self) -> Result<GlobalCatalogData, CatalogError> {
        let response = self.client.get(&self.catalog_url).send().await?;
        let catalog = response.json().await?;
        Ok(catalog)
    }

    pub fn catalog_url(&self) -> &str { &self.catalog_url }
}
