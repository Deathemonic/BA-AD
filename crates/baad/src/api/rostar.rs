use baad_shared::{ChinaState, ROSTAR_CHANNEL_ID, ROSTAR_PLATFORM_ID, ROSTAR_STATE_URL, client};

use crate::error::CatalogError;

#[derive(Default)]
pub struct RoStarClient;

impl RoStarClient {
    pub fn new() -> Self { Self }

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
