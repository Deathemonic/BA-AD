use std::time::{SystemTime, UNIX_EPOCH};

use baad_shared::{
    Domain,
    GameBaseConfig,
    GameFile,
    GameJsonConfig,
    GameJsonData,
    YOSTAR_BASE_URL,
    YOSTAR_DOMAIN_PATH,
    YOSTAR_GAME_BASE_CONFIG_PATH,
    YOSTAR_GAME_JSON_CONFIG_PATH,
    YOSTAR_GAME_TAG,
    YOSTAR_SIGNATURE_DATA,
    YOSTAR_VERSION,
    client
};
use bacy::crypto::md5::compute_hash_str;
use serde::{Deserialize, Serialize};

use crate::error::CatalogError;

#[derive(Serialize, Deserialize)]
struct YoStarHead {
    game_tag: String,
    time: u64,
    version: String
}

#[derive(Serialize, Deserialize)]
struct YoStarSignature {
    head: YoStarHead,
    sign: String
}

#[derive(Deserialize)]
struct YoStarResponse<T> {
    code: i32,
    data: T
}

#[derive(Default)]
pub struct YoStarClient;

impl YoStarClient {
    pub fn new() -> Self { Self }

    fn get_timestamp() -> Result<u64, CatalogError> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| CatalogError::DeserializationFailed)?;
        Ok(duration.as_millis() as u64)
    }

    fn create_signature() -> Result<String, CatalogError> {
        let head = YoStarHead {
            game_tag: YOSTAR_GAME_TAG.to_string(),
            time: Self::get_timestamp()?,
            version: YOSTAR_VERSION.to_string()
        };

        let head_json = serde_json::to_string(&head)?;
        let sign_data = format!("{}{}", head_json, YOSTAR_SIGNATURE_DATA);
        let sign = compute_hash_str(&sign_data);

        let signature = YoStarSignature { head, sign };
        serde_json::to_string(&signature).map_err(Into::into)
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str
    ) -> Result<T, CatalogError> {
        let url = format!("{}{}", YOSTAR_BASE_URL, endpoint);
        let signature = Self::create_signature()?;

        let response = client().get(&url).header("Authorization", signature).send().await?;

        let yostar_response: YoStarResponse<T> = response.json().await?;

        if yostar_response.code != 200 {
            return Err(CatalogError::DeserializationFailed);
        }

        Ok(yostar_response.data)
    }

    pub async fn get_base_config(&self) -> Result<GameBaseConfig, CatalogError> {
        self.request(YOSTAR_GAME_BASE_CONFIG_PATH).await
    }

    pub async fn get_domain(&self) -> Result<Domain, CatalogError> {
        self.request(YOSTAR_DOMAIN_PATH).await
    }

    pub async fn get_json_config(
        &self,
        version: &str,
        file_path: &str
    ) -> Result<GameJsonConfig, CatalogError> {
        let encoded_path = urlencoding::encode(file_path);
        let endpoint = format!(
            "{}?version={}&file_path={}",
            YOSTAR_GAME_JSON_CONFIG_PATH, version, encoded_path
        );
        self.request(&endpoint).await
    }

    pub async fn get_json_data(&self, url: &str) -> Result<GameJsonData, CatalogError> {
        let response = client().get(url).send().await?;
        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_resources_asset(&self) -> Result<(String, GameFile), CatalogError> {
        let base_config = self.get_base_config().await?;
        let domain = self.get_domain().await?;
        let json_config = self
            .get_json_config(&base_config.game_latest_version, &base_config.game_latest_file_path)
            .await?;
        let json_data = self.get_json_data(&json_config.url).await?;

        let resources_file = json_data
            .file
            .into_iter()
            .find(|f| f.path.ends_with("/resources.assets"))
            .ok_or(CatalogError::DeserializationFailed)?;

        let download_url =
            format!("{}{}{}", domain.primary_cdn, json_data.source, resources_file.path);

        Ok((download_url, resources_file))
    }
}
