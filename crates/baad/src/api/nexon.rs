use baad_core::{CatalogError, GLOBAL_API_URL, GlobalAddressable, GlobalCatalog, MarketConfig};
use reqwest::Client;

#[derive(Default)]
pub struct NexonClient {
    client: Client
}

impl NexonClient {
    pub fn new() -> Self { Self::default() }

    pub fn with_client(client: Client) -> Self { Self { client } }

    pub async fn get_addressable(
        &self,
        market_config: &MarketConfig,
        version: &str,
        build_number: &str
    ) -> Result<GlobalAddressable, CatalogError> {
        let response = self
            .client
            .post(GLOBAL_API_URL)
            .json(&serde_json::json!({
                "market_game_id": market_config.market_game_id,
                "market_code": market_config.market_code,
                "curr_build_version": version,
                "curr_build_number": build_number
            }))
            .send()
            .await?;

        let addressable = response.json().await?;
        Ok(addressable)
    }

    pub async fn get_catalog(&self, resource_path: &str) -> Result<GlobalCatalog, CatalogError> {
        let response = self.client.get(resource_path).send().await?;
        let catalog = response.json().await?;
        Ok(catalog)
    }
}
