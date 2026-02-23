use std::path::PathBuf;

use baad_core::{
    API_FILENAME,
    ApiData,
    BuildType,
    CatalogError,
    Downloads,
    GlobalCatalog as GlobalCatalogData,
    MarketConfig,
    Platform
};
use baad_utils::file::get_data_path;
use baad_utils::json::{load, update};
use baad_utils::{info, warn};

use crate::catalog::traits::Catalog;
use crate::api::NexonClient;
use crate::cdn::GlobalCdn;
use crate::strategy::GlobalStrategy;

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
        MarketConfig::for_global(platform, build_type)?;

        Ok(Self {
            nexon_client: NexonClient::new(),
            platform,
            build_type,
            paths: GlobalPaths {
                api: get_data_path(API_FILENAME)?
            }
        })
    }

    async fn full_update(&self, version: String) -> Result<String, CatalogError> {
        info!(version = %version, "Performing full update");

        let market_config = MarketConfig::for_global(self.platform, self.build_type)?;

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
            data.global.platform = <&str>::from(platform).into();
            data.global.build_type = <&str>::from(build_type).into();
        })
        .await?;

        Ok(catalog_url)
    }

    pub async fn get_catalog_url(&self, version: &str) -> Result<String, CatalogError> {
        let api_data = load::<ApiData>(&self.paths.api)
            .await
            .ok()
            .filter(|d| d.global.version == version)
            .map(|d| d.global.catalog_url)
            .filter(|url| !url.is_empty());

        if let Some(catalog_url) = api_data {
            info!("Using existing catalog URL");
            return Ok(catalog_url);
        }

        warn!("Catalog URL doesn't exist");
        self.full_update(version.to_string()).await
    }

    pub async fn fetch_catalogs(
        &self,
        catalog_url: &str
    ) -> Result<GlobalCatalogData, CatalogError> {
        let cdn = GlobalCdn::new(catalog_url.to_string());
        let catalog = cdn.fetch().await?;
        Ok(catalog)
    }
}

impl Catalog for GlobalCatalog {
    type Resources = GlobalCatalogData;

    async fn fetch_resources(&self) -> Result<(String, Self::Resources), CatalogError> {
        let version = self.nexon_client.get_version().await?;
        info!(version = %version, "Version");

        let catalog_url = self.get_catalog_url(&version).await?;
        let base_url = GlobalCdn::derive_base_url(&catalog_url)
            .ok_or(CatalogError::DeserializationFailed)?
            .to_string();

        let catalog = self.fetch_catalogs(&catalog_url).await?;
        Ok((base_url, catalog))
    }

    fn build_downloads(&self, resources: &Self::Resources, base_or_url: &str) -> Downloads {
        GlobalStrategy::build_downloads(&resources.resources, base_or_url)
    }
}
