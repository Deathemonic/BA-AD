use baad_core::{
    CatalogError,
    GLOBAL_API_URL,
    GlobalAddressable,
    GlobalCatalog,
    MarketConfig,
    PLAYSTORE_REGEX_VERSION,
    PLAYSTORE_VERSION_URL,
    client
};

#[derive(Default)]
pub struct NexonClient;

impl NexonClient {
    pub fn new() -> Self { Self }

    pub async fn get_version(&self) -> Result<String, CatalogError> {
        let response = client().get(PLAYSTORE_VERSION_URL).send().await?;
        let body = response.text().await?;
        PLAYSTORE_REGEX_VERSION
            .find(&body)
            .map(|m| m.as_str().to_string())
            .ok_or(CatalogError::DeserializationFailed)
    }

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
