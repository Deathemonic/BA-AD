use std::path::PathBuf;

use baad_shared::{API_FILENAME, ApiData, CatalogError, ChinaState, Downloads, Platform};
use baad_utils::file::get_data_path;
use baad_utils::info;
use baad_utils::json::{load, update};

use crate::api::RoStarClient;
use crate::catalog::traits::Catalog;
use crate::cdn::{ChinaCdn, ChinaResources};
use crate::download::ResourceCategory;
use crate::strategy::ChinaStrategy;

struct ChinaPaths {
    api: PathBuf
}

pub struct ChinaCatalog {
    rostar_client: RoStarClient,
    category: Vec<ResourceCategory>,
    platform: Platform,
    paths: ChinaPaths
}

impl ChinaCatalog {
    pub fn new(category: Vec<ResourceCategory>, platform: Platform) -> Result<Self, CatalogError> {
        platform.ensure_supported()?;

        Ok(Self {
            rostar_client: RoStarClient::new(),
            category,
            platform,
            paths: ChinaPaths {
                api: get_data_path(API_FILENAME)?
            }
        })
    }

    async fn resolve(&self, version: String, state: &ChinaState) -> Result<(String, bool), CatalogError> {
        let catalog_url = state
            .addressables_catalog_url_roots
            .first()
            .ok_or(CatalogError::DeserializationFailed)?
            .trim_end_matches('/')
            .to_string();

        let platform: &'static str = self.platform.into();
        let api_data = load::<ApiData>(&self.paths.api).await.ok();
        let up_to_date = api_data.as_ref().is_some_and(|data| {
            data.china.version == version
                && data.china.catalog_url == catalog_url
                && data.china.resource_version == state.resource_version
                && data.china.table_version == state.table_version
                && data.china.media_version == state.media_version
                && data.china.platform == platform
        });

        if up_to_date {
            return Ok((catalog_url, true));
        }

        info!("Catalog changed, updating...");
        update(&self.paths.api, |data: &mut ApiData| {
            data.china.version = version;
            data.china.catalog_url = catalog_url.clone();
            data.china.resource_version = state.resource_version.clone();
            data.china.table_version = state.table_version.clone();
            data.china.media_version = state.media_version.clone();
            data.china.platform = platform.into();
        })
        .await?;

        Ok((catalog_url, false))
    }
}

impl Catalog for ChinaCatalog {
    type Resources = ChinaResources;

    async fn fetch_resources(&self) -> Result<(String, Self::Resources, bool), CatalogError> {
        let version = self.rostar_client.get_version().await?;
        info!(version = %version, "Version");

        let state = self.rostar_client.get_state(&version).await?;
        let (url, up_to_date) = self.resolve(version, &state).await?;

        let cdn = ChinaCdn::new(
            url,
            self.platform,
            state.resource_version,
            state.table_version,
            state.media_version
        );
        let resources = cdn.fetch(&self.category).await?;

        Ok((cdn.catalog_url, resources, up_to_date))
    }

    fn build_downloads(&self, resources: Self::Resources, url: &str) -> Downloads {
        Downloads {
            assets: resources
                .assets
                .map(|m| ChinaStrategy::build_asset_downloads(m, url, self.platform))
                .unwrap_or_default(),
            tables: resources
                .table
                .map(|t| ChinaStrategy::build_table_downloads(t, url))
                .unwrap_or_default(),
            media: resources
                .media
                .map(|m| ChinaStrategy::build_media_downloads(m, url))
                .unwrap_or_default()
        }
    }
}
