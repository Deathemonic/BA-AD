use baad_shared::{
    ChinaState,
    ROSTAR_BILIBILI_GAME_INFO_URL,
    ROSTAR_CHANNEL_ID,
    ROSTAR_PLATFORM_ID,
    ROSTAR_STATE_URL,
    client
};
use serde::Deserialize;

use crate::error::CatalogError;

#[derive(Deserialize)]
struct BilibiliGameInfo {
    code: i32,
    data: BilibiliGameData
}

#[derive(Deserialize)]
struct BilibiliGameData {
    android_download_link: String,
    android_download_link2: String
}

impl BilibiliGameData {
    fn apk_url(self) -> Option<String> {
        [self.android_download_link, self.android_download_link2]
            .into_iter()
            .find(|url| !url.is_empty())
    }
}

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

    pub async fn get_apk_url(&self) -> Result<String, CatalogError> {
        let response = client().get(ROSTAR_BILIBILI_GAME_INFO_URL).send().await?;
        let game_info = response.json::<BilibiliGameInfo>().await?;

        if game_info.code != 0 {
            return Err(CatalogError::DeserializationFailed);
        }

        game_info.data.apk_url().ok_or(CatalogError::DeserializationFailed)
    }
}
