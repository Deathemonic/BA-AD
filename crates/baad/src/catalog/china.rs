use std::path::PathBuf;

use baad_core::{
    API_FILENAME,
    ApiData,
    CatalogError,
    ChinaState,
    Platform,
    ServerConfigError
};
use baad_utils::file::get_data_path;
use baad_utils::info;
use baad_utils::json::{load, update};

use crate::api::RoStarClient;
use crate::cdn::{ChinaCdn, ChinaResources};
use crate::download::ResourceCategory;

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
        if platform == Platform::Windows {
            return Err(ServerConfigError::WindowsNotSupported.into());
        }

        Ok(Self {
            rostar_client: RoStarClient::new(),
            category,
            platform,
            paths: ChinaPaths {
                api: get_data_path(API_FILENAME)?
            }
        })
    }

    pub async fn fetch_resources(&self) -> Result<(String, ChinaResources), CatalogError> {
        let version = self.rostar_client.get_version().await?;
        info!(version = %version, "Version");

        let state = self.rostar_client.get_state(&version).await?;
        let root = self.resolve(version, &state).await?;

        let cdn = ChinaCdn::new(
            root.clone(),
            self.platform,
            state.resource_version,
            state.table_version,
            state.media_version
        )?;
        let resources = cdn.fetch(&self.category).await?;

        Ok((root, resources))
    }

    async fn resolve(&self, version: String, state: &ChinaState) -> Result<String, CatalogError> {
        let root = state
            .addressables_catalog_url_roots
            .first()
            .ok_or(CatalogError::DeserializationFailed)?
            .trim_end_matches('/')
            .to_string();

        let platform: &'static str = self.platform.into();
        let api_data = load::<ApiData>(&self.paths.api).await.ok();
        let up_to_date = api_data.as_ref().is_some_and(|data| {
            data.china.version == version
                && data.china.catalog_url == root
                && data.china.resource_version == state.resource_version
                && data.china.table_version == state.table_version
                && data.china.media_version == state.media_version
                && data.china.platform == platform
        });

        if up_to_date {
            info!("Catalog up to date");
            return Ok(root);
        }

        info!("Catalog changed, updating...");
        update(&self.paths.api, |data: &mut ApiData| {
            data.china.version = version;
            data.china.catalog_url = root.clone();
            data.china.resource_version = state.resource_version.clone();
            data.china.table_version = state.table_version.clone();
            data.china.media_version = state.media_version.clone();
            data.china.platform = platform.into();
        })
        .await?;

        Ok(root)
    }
}
