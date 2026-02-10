use std::path::PathBuf;

use baad_core::{
    API_FILENAME,
    ApiData,
    BuildType,
    CatalogError,
    GlobalCatalog as GlobalCatalogData,
    Platform,
    ServerConfig,
    ServerRegion
};
use baad_utils::file::get_data_path;
use baad_utils::info;
use baad_utils::json::{load, update};
use reqwest::Client;

use crate::api::NexonClient;
use crate::cdn::GlobalCdn;

struct GlobalPaths {
    api: PathBuf
}

pub struct GlobalCatalog {
    nexon_client: NexonClient,
    platform: Platform,
    build_type: BuildType,
    paths: GlobalPaths
}

impl GlobalCatalog {
    pub fn new(platform: Platform, build_type: BuildType) -> Result<Self, CatalogError> {
        Self::with_client(Client::new(), platform, build_type)
    }

    pub fn with_client(
        client: Client,
        platform: Platform,
        build_type: BuildType
    ) -> Result<Self, CatalogError> {
        Ok(Self {
            nexon_client: NexonClient::with_client(client),
            platform,
            build_type,
            paths: GlobalPaths {
                api: get_data_path(API_FILENAME)?
            }
        })
    }

    async fn full_update(&self, version: String) -> Result<String, CatalogError> {
        info!(version = %version, "Performing full update");

        let market_config =
            ServerConfig::new(ServerRegion::Global, Some(self.platform), Some(self.build_type))?
                .get_market_config()
                .ok_or(CatalogError::DeserializationFailed)?;

        let build_number =
            version.split('.').next_back().ok_or(CatalogError::DeserializationFailed)?;

        let addressable =
            self.nexon_client.get_addressable(&market_config, &version, build_number).await?;

        let catalog_url = addressable.patch.resource_path.clone();
        let platform = self.platform;
        let build_type = self.build_type;

        update(&self.paths.api, |data: &mut ApiData| {
            data.global.version = version;
            data.global.catalog_url = catalog_url.clone();
            data.global.platform = platform.as_str().into();
            data.global.build_type = build_type.as_str().into();
        })
        .await?;

        Ok(catalog_url)
    }

    pub async fn get_catalog_url(&self, version: &str) -> Result<String, CatalogError> {
        let api_data = load::<ApiData>(&self.paths.api).await.ok();

        if api_data.as_ref().map(|d| d.global.version.as_str()) != Some(version) {
            info!("Version changed, performing full update");
            return self.full_update(version.to_string()).await;
        }

        let api_data = api_data.ok_or(CatalogError::DeserializationFailed)?;

        if api_data.global.catalog_url.is_empty() {
            info!("Catalog URL empty, performing full update");
            return self.full_update(version.to_string()).await;
        }

        info!("Using cached catalog URL");
        Ok(api_data.global.catalog_url)
    }

    pub async fn fetch_catalogs(
        &self,
        catalog_url: &str
    ) -> Result<GlobalCatalogData, CatalogError> {
        let cdn_client = GlobalCdn::new(catalog_url.to_string(), self.platform);
        let catalog = cdn_client.fetch().await?;

        Ok(catalog)
    }
}
