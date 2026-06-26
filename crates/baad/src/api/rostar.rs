use baad_core::{
    CatalogError,
    ChinaState,
    REGEX_VERSION,
    ROSTAR_CHANNEL_ID,
    ROSTAR_PLATFORM_ID,
    ROSTAR_STATE_URL,
    ROSTAR_VERSION_URL,
    client
};

#[derive(Default)]
pub struct RoStarClient;

impl RoStarClient {
    pub fn new() -> Self { Self }

    pub async fn get_version(&self) -> Result<String, CatalogError> {
        let response = client().get(ROSTAR_VERSION_URL).send().await?;
        let body = response.text().await?;
        REGEX_VERSION
            .find(&body)
            .map(|m| m.as_str().to_string())
            .ok_or(CatalogError::DeserializationFailed)
    }

    pub async fn get_state(&self, version: &str) -> Result<ChinaState, CatalogError> {
        let response = client()
            .get(ROSTAR_STATE_URL)
            .header("APP-VER", version)
            .header("PLATFORM-ID", ROSTAR_PLATFORM_ID)
            .header("CHANNEL-ID", ROSTAR_CHANNEL_ID)
            .send()
            .await?;

        let state = response.json().await?;
        Ok(state)
    }
}
