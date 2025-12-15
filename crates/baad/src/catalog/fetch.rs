use crate::apk::ApkFetcher;
use crate::helpers::{
    CatalogError, GAME_CONFIG_PATTERN, GLOBAL_API_URL, GlobalAddressable, GlobalCatalog,
    JapanAddressable, ServerConfig, ServerRegion,
};
use crate::utils::json;

use baad_core::{debug, file, info};
use bacy::table_encryption::{convert_string, create_key, encrypt_string};
use base64::{Engine, engine::general_purpose};
use memchr::memmem::Finder;
use rayon::prelude::*;
use reqwest::Client;
use serde_json::{Value, to_string_pretty};
use std::fs;
use std::path::Path;
use std::rc::Rc;
use walkdir::WalkDir;

struct Paths {
    addressable_path: Box<Path>,
    resources_path: Box<Path>,
}

pub struct CatalogFetcher {
    client: Client,
    apk_fetcher: Rc<ApkFetcher>,
    config: Rc<ServerConfig>,
    paths: Paths,
}

impl CatalogFetcher {
    pub fn new(config: Rc<ServerConfig>, apk_fetcher: ApkFetcher) -> Result<Self, CatalogError> {
        let addressable_path = match config.region {
            ServerRegion::Global => file::get_data_path("catalog/GlobalAddressables.json")?,
            ServerRegion::Japan => file::get_data_path("catalog/JapanAddressables.json")?,
        };
        let resources_path = file::get_data_path("catalog/global/Resources.json")?;

        let client = Client::new();

        Ok(Self {
            client,
            apk_fetcher: Rc::new(apk_fetcher),
            config,
            paths: Paths {
                addressable_path: addressable_path.into_boxed_path(),
                resources_path: resources_path.into_boxed_path(),
            },
        })
    }

    pub fn find_game_config(&self) -> Result<Vec<u8>, CatalogError> {
        info!("Searching for game config...");
        let data_path = file::get_data_path("data")?;

        let mut files: Vec<_> = WalkDir::new(&data_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        files.sort_by_key(|e| e.metadata().map(|m| m.len()).unwrap_or(u64::MAX));

        let files: Vec<_> = files.into_iter().map(|e| e.into_path()).collect();
        let finder = Finder::new(GAME_CONFIG_PATTERN);

        files
            .par_iter()
            .find_map_any(|path| {
                let buffer = fs::read(path).ok()?;
                let pos = finder.find(&buffer)?;

                let data_start = pos + GAME_CONFIG_PATTERN.len();
                let data = &buffer[data_start..];

                if data.len() >= 2 {
                    debug!(?path, "Found game config");
                    Some(data[..data.len() - 2].to_vec())
                } else {
                    None
                }
            })
            .ok_or(CatalogError::DeserializationFailed)
    }

    pub fn decrypt_game_config(&self, data: &[u8]) -> Result<String, CatalogError> {
        info!("Decrypting game config...");

        let encoded_data = general_purpose::STANDARD.encode(data);
        debug!(encoded_data, "Encoded data");

        let game_config = create_key(b"GameMainConfig");
        debug!(?game_config, "Game config");

        let server_data = create_key(b"ServerInfoDataUrl");
        debug!(?server_data, "Server data");

        let decrypted_data = convert_string(&encoded_data, &game_config)
            .map_err(|_| CatalogError::DeserializationFailed)?;
        debug!(decrypted_data, "Decrypted data");

        let loaded_data: Value = serde_json::from_str(&decrypted_data)?;
        debug!(?loaded_data, "Loaded data");

        let decrypted_key = encrypt_string("ServerInfoDataUrl", &server_data)
            .map_err(|_| CatalogError::DeserializationFailed)?;
        debug!(decrypted_key, "Decrypted key");

        let decrypted_value = loaded_data
            .get(&decrypted_key)
            .and_then(|v| v.as_str())
            .ok_or(CatalogError::DeserializationFailed)?;
        debug!(decrypted_value, "Decrypted value");

        let converted = convert_string(decrypted_value, &server_data)
            .map_err(|_| CatalogError::DeserializationFailed)?;
        Ok(converted)
    }

    async fn japan_addressable(&self) -> Result<String, CatalogError> {
        let api_url = self.decrypt_game_config(self.find_game_config()?.as_slice())?;
        debug!(api_url, "API URL");

        let catalog = self
            .client
            .get(&api_url)
            .send()
            .await?
            .json::<JapanAddressable>()
            .await?;

        json::save_json(&self.paths.addressable_path, &catalog).await?;

        json::update_api_data(|data| {
            data.japan.addressable_url = api_url;
            data.japan.platform = self.config.platform.as_str().into();
        })
        .await?;

        info!(success = true, "Saved addressables info");

        Ok(to_string_pretty(&catalog)?)
    }

    async fn japan_catalog(&self) -> Result<String, CatalogError> {
        self.japan_addressable().await?;

        let addressable: JapanAddressable = json::load_json(&self.paths.addressable_path).await?;

        let catalog_url = addressable
            .connection_groups
            .first()
            .and_then(|group| group.override_connection_groups.get(1))
            .map(|override_group| &override_group.addressables_catalog_url_root)
            .ok_or(CatalogError::DeserializationFailed)?;

        json::update_api_data(|data| {
            data.japan.catalog_url = catalog_url.into();
            data.japan.platform = self.config.platform.as_str().into();
        })
        .await?;

        info!(success = true, "Saved catalog info");

        Ok(catalog_url.into())
    }

    async fn global_addressable(&self) -> Result<String, CatalogError> {
        let version = self
            .apk_fetcher
            .check_version()
            .await?
            .ok_or(CatalogError::DeserializationFailed)?;
        debug!(version, "Version");

        let build_number = version
            .split('.')
            .next_back()
            .ok_or(CatalogError::DeserializationFailed)?;
        debug!(build_number, "Build number");

        let market_config = self
            .config
            .get_market_config()
            .ok_or(CatalogError::DeserializationFailed)?;

        let api = self
            .client
            .post(GLOBAL_API_URL)
            .json(&serde_json::json!({
                "market_game_id": market_config.market_game_id,
                "market_code": market_config.market_code,
                "curr_build_version": version,
                "curr_build_number": build_number
            }))
            .send()
            .await?
            .json::<GlobalAddressable>()
            .await?;

        json::save_json(&self.paths.addressable_path, &api).await?;

        info!(success = true, "Saved addressables info");

        Ok(to_string_pretty(&api)?)
    }

    async fn global_resources(&self) -> Result<String, CatalogError> {
        self.global_addressable().await?;

        let addressable: GlobalAddressable = json::load_json(&self.paths.addressable_path).await?;

        let catalog = self
            .client
            .get(&addressable.patch.resource_path)
            .send()
            .await?
            .json::<GlobalCatalog>()
            .await?;

        json::save_json(&self.paths.resources_path, &catalog).await?;

        json::update_api_data(|data| {
            data.global.catalog_url = addressable.patch.resource_path;
            data.global.platform = self.config.platform.as_str().into();
            data.global.build_type = self.config.build_type.as_str().into();
        })
        .await?;

        info!(success = true, "Saved catalogs info");

        Ok(to_string_pretty(&catalog)?)
    }

    pub async fn get_catalogs(&self) -> Result<String, CatalogError> {
        info!("Fetching catalogs...");

        match &self.config.region {
            ServerRegion::Japan => Ok(self.japan_catalog().await?),
            ServerRegion::Global => Ok(self.global_resources().await?),
        }
    }

    pub async fn get_addressable(&self) -> Result<String, CatalogError> {
        info!("Fetching addressables...");

        match &self.config.region {
            ServerRegion::Japan => Ok(self.japan_addressable().await?),
            ServerRegion::Global => Ok(self.global_addressable().await?),
        }
    }
}
