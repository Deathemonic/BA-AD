use baad_core::{CatalogError, GlobalCatalog as GlobalCatalogData};
use reqwest::Client;

pub struct GlobalCdn {
    client: Client,
    catalog_url: String
}

impl GlobalCdn {
    pub fn new(catalog_url: String) -> Self {
        Self {
            client: Client::new(),
            catalog_url
        }
    }

    pub fn with_client(client: Client, catalog_url: String) -> Self { Self { client, catalog_url } }

    pub async fn fetch(&self) -> Result<GlobalCatalogData, CatalogError> {
        let response = self.client.get(&self.catalog_url).send().await?;
        let catalog = response.json().await?;
        Ok(catalog)
    }

    pub fn catalog_url(&self) -> &str { &self.catalog_url }

    pub fn derive_base_url(resource_path: &str) -> Option<&str> {
        resource_path.rfind('/').map(|pos| &resource_path[..=pos])
    }
}
