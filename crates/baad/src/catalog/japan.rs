use std::path::PathBuf;

use baad_core::{
    API_FILENAME,
    ApiData,
    CatalogError,
    GAME_CONFIG_PATTERN,
    JapanAddressable,
    Platform
};
use baad_utils::file::get_data_path;
use baad_utils::json::{load, update};
use baad_utils::{debug, info};
use bacy::crypto::table::{create_key, decrypt_string, encrypt_string};
use base64::Engine;
use base64::engine::general_purpose;
use memchr::memmem::Finder;
use serde_json::Value;
use tokio::fs;

use crate::api::YoStarClient;
use crate::catalog::Downloads;
use crate::cdn::{JapanCdn, JapanResources};
use crate::download::download_file;
use crate::strategy::{AssetStrategy, MediaStrategy, TableStrategy};

struct JapanPaths {
    api: PathBuf,
    resources: PathBuf
}

pub struct JapanCatalog {
    yostar_client: YoStarClient,
    platform: Platform,
    paths: JapanPaths
}

impl JapanCatalog {
    pub fn new(platform: Platform) -> Result<Self, CatalogError> {
        Ok(Self {
            yostar_client: YoStarClient::new(),
            platform,
            paths: JapanPaths {
                api: get_data_path(API_FILENAME)?,
                resources: get_data_path("data/resources.assets")?
            }
        })
    }

    pub fn platform(&self) -> Platform { self.platform }

    async fn full_update(&self, version: String) -> Result<String, CatalogError> {
        info!(version = %version, "Performing full update");

        let resources_path = self.download_resources_asset().await?;
        let game_main_data = self.extract_game_main(&resources_path).await?;
        let server_info_url = self.decrypt_game_main(&game_main_data)?;
        let addressable = self.fetch_addressable(&server_info_url).await?;
        let catalog_url = JapanCdn::extract_catalog_url(&addressable)?;

        let platform = self.platform;
        update(&self.paths.api, |data: &mut ApiData| {
            data.japan.version = version;
            data.japan.server_info_url = server_info_url.clone();
            data.japan.catalog_url = catalog_url.to_string();
            data.japan.platform = <&str>::from(platform).into();
        })
        .await?;

        Ok(catalog_url.to_string())
    }

    pub async fn get_catalog_url(&self) -> Result<String, CatalogError> {
        let api_data = load::<ApiData>(&self.paths.api).await.ok();

        let base_config = self.yostar_client.get_base_config().await?;
        let latest_version = base_config.game_latest_version;

        if api_data.as_ref().map(|d| &d.japan.version) != Some(&latest_version) {
            info!("Version changed, performing full update");
            return self.full_update(latest_version).await;
        }

        let api_data = api_data.ok_or(CatalogError::DeserializationFailed)?;
        let addressable = self.fetch_addressable(&api_data.japan.server_info_url).await?;
        let catalog_url = JapanCdn::extract_catalog_url(&addressable)?;

        if api_data.japan.catalog_url == catalog_url {
            info!("Catalog URL unchanged, using cache");
            return Ok(api_data.japan.catalog_url);
        }

        info!("Catalog URL changed, updating cache");
        update(&self.paths.api, |data: &mut ApiData| {
            data.japan.catalog_url = catalog_url.to_string();
        })
        .await?;

        Ok(catalog_url.to_string())
    }

    pub async fn fetch_resources(&self) -> Result<(String, JapanResources), CatalogError> {
        let url = self.get_catalog_url().await?;
        let cdn = JapanCdn::new(url.clone(), self.platform);
        let resources = cdn.fetch().await?;
        Ok((url, resources))
    }

    pub fn build_downloads(&self, resources: &JapanResources, url: &str) -> Downloads {
        Downloads {
            assets: AssetStrategy::build_downloads(&resources.packing, url, self.platform),
            tables: TableStrategy::build_downloads(&resources.table, url),
            media: MediaStrategy::build_downloads(&resources.media, url)
        }
    }

    async fn download_resources_asset(&self) -> Result<PathBuf, CatalogError> {
        let (download_url, file_info) = self.yostar_client.get_resources_asset().await?;

        if self.paths.resources.exists() {
            debug!("resources.assets already exists, skipping download");
            return Ok(self.paths.resources.clone());
        }

        debug!(url = %download_url, "Downloading resources.assets");
        debug!(hash = %file_info.hash, size = %file_info.size, "Expected file info");

        download_file(&download_url, &self.paths.resources, Some(file_info.hash), 3).await?;

        Ok(self.paths.resources.clone())
    }

    async fn extract_game_main(&self, resources_path: &PathBuf) -> Result<Vec<u8>, CatalogError> {
        debug!("Extracting GameMain from resources.assets");

        let buffer = fs::read(resources_path).await?;
        let finder = Finder::new(GAME_CONFIG_PATTERN);

        let pos = finder.find(&buffer).ok_or(CatalogError::DeserializationFailed)?;

        let data_start = pos + GAME_CONFIG_PATTERN.len();
        let size_bytes = &GAME_CONFIG_PATTERN[GAME_CONFIG_PATTERN.len() - 4..];
        let size = i32::from_le_bytes([size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]])
            as usize;

        let content_end = data_start + size;

        if content_end > buffer.len() {
            return Err(CatalogError::DeserializationFailed);
        }

        let extracted = &buffer[data_start..content_end];

        Ok(extracted.to_vec())
    }

    fn decrypt_game_main(&self, data: &[u8]) -> Result<String, CatalogError> {
        debug!("Decrypting GameMain config");

        let encoded_data = general_purpose::STANDARD.encode(data);
        let game_config = create_key("GameMainConfig");
        let server_data = create_key("ServerInfoDataUrl");

        let decrypted_data = decrypt_string(&encoded_data, &game_config)?;
        let loaded_data: Value = serde_json::from_str(&decrypted_data)?;
        let decrypted_key = encrypt_string("ServerInfoDataUrl", &server_data);

        let decrypted_value = loaded_data
            .get(&decrypted_key)
            .and_then(|v| v.as_str())
            .ok_or(CatalogError::DeserializationFailed)?;

        let converted = decrypt_string(decrypted_value, &server_data)?;

        Ok(converted)
    }

    async fn fetch_addressable(&self, url: &str) -> Result<JapanAddressable, CatalogError> {
        debug!(url = %url, "Fetching addressable");
        JapanCdn::fetch_addressable(url).await
    }
}
