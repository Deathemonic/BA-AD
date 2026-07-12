use baad_shared::{GLOBAL_API_URL, GlobalAddressable, GlobalCatalog, MarketConfig, client};

use crate::error::CatalogError;

#[derive(Default)]
pub struct NexonClient;

impl NexonClient {
    pub fn new() -> Self { Self }

    pub async fn get_addressable(
        &self,
        market_config: &MarketConfig,
        version: &str,
        build_number: &str
    ) -> Result<GlobalAddressable, CatalogError> {
        let response = client()
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
        let response = client().get(resource_path).send().await?;
        let catalog = response.json().await?;
        Ok(catalog)
    }
}
