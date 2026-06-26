use baad_core::{
    ASSET_BUNDLES,
    CatalogError,
    ChinaBundleManifest,
    ChinaMediaEntry,
    ChinaTableManifest,
    MEDIA_RESOURCES,
    Platform,
    ROSTAR_BUNDLE_INFO_FILE,
    ROSTAR_CATALOG_DIR,
    ROSTAR_MANIFEST_DIR,
    ROSTAR_MEDIA_MANIFEST_FILE,
    ROSTAR_TABLE_MANIFEST_FILE,
    ServerConfigError,
    TABLE_BUNDLES,
    client
};
use baad_utils::debug;
use baad_utils::file::get_data_path;
use baad_utils::json::load;
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::download::{ResourceCategory, download_file};

pub struct ChinaCdn {
    root: String,
    platform: Platform,
    resource_version: String,
    table_version: String,
    media_version: String
}

pub struct ChinaResources {
    pub assets: Option<ChinaBundleManifest>,
    pub table: Option<ChinaTableManifest>,
    pub media: Option<Vec<ChinaMediaEntry>>
}

impl ChinaCdn {
    pub fn new(
        root: String,
        platform: Platform,
        resource_version: String,
        table_version: String,
        media_version: String
    ) -> Result<Self, ServerConfigError> {
        if platform == Platform::Windows {
            return Err(ServerConfigError::WindowsNotSupported);
        }

        Ok(Self {
            root: root.trim_end_matches('/').to_string(),
            platform,
            resource_version,
            table_version,
            media_version
        })
    }

    pub async fn fetch(
        &self,
        category: &[ResourceCategory]
    ) -> Result<ChinaResources, CatalogError> {
        Ok(ChinaResources {
            assets: if category.contains(&ResourceCategory::Assets) {
                Some(self.fetch_assets().await?)
            } else {
                None
            },
            table: if category.contains(&ResourceCategory::Tables) {
                Some(self.fetch_table().await?)
            } else {
                None
            },
            media: if category.contains(&ResourceCategory::Media) {
                Some(self.fetch_media().await?)
            } else {
                None
            }
        })
    }

    pub async fn fetch_assets(&self) -> Result<ChinaBundleManifest, CatalogError> {
        let url = format!(
            "{}/{}/{}/{}/{}/{}",
            self.root,
            ASSET_BUNDLES,
            ROSTAR_CATALOG_DIR,
            self.resource_version,
            self.platform.display_name(),
            ROSTAR_BUNDLE_INFO_FILE
        );
        let cache = format!("bundleDownloadInfo_{}.json", self.resource_version);
        self.fetch_json(&url, &cache).await
    }

    pub async fn fetch_table(&self) -> Result<ChinaTableManifest, CatalogError> {
        let url = format!(
            "{}/{}/{}/{}/{}",
            self.root,
            ROSTAR_MANIFEST_DIR,
            TABLE_BUNDLES,
            self.table_version,
            ROSTAR_TABLE_MANIFEST_FILE
        );
        let cache = format!("TableManifest_{}.json", self.table_version);
        self.fetch_json(&url, &cache).await
    }

    pub async fn fetch_media(&self) -> Result<Vec<ChinaMediaEntry>, CatalogError> {
        let url = format!(
            "{}/{}/{}/{}/{}",
            self.root,
            ROSTAR_MANIFEST_DIR,
            MEDIA_RESOURCES,
            self.media_version,
            ROSTAR_MEDIA_MANIFEST_FILE
        );
        let path =
            get_data_path(&self.cache_key(&format!("MediaManifest_{}.txt", self.media_version)))?;

        let text = if path.exists() {
            debug!("Media manifest up to date, using cache");
            fs::read_to_string(&path).await?
        } else {
            debug!("Media manifest outdated, downloading");
            let body = client().get(&url).send().await?.text().await?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&path, &body).await?;
            body
        };

        Ok(Self::parse_media(&text))
    }

    async fn fetch_json<T: DeserializeOwned>(
        &self,
        url: &str,
        filename: &str
    ) -> Result<T, CatalogError> {
        let path = get_data_path(&self.cache_key(filename))?;

        if !path.exists() {
            debug!(filename, "Manifest outdated, downloading");
            download_file(url, &path, None, 3).await?;
        } else {
            debug!(filename, "Manifest up to date, using cache");
        }

        Ok(load::<T>(&path).await?)
    }

    fn cache_key(&self, filename: &str) -> String {
        format!("catalog/china/{}/{}", self.platform.as_ref(), filename)
    }

    fn parse_media(text: &str) -> Vec<ChinaMediaEntry> {
        text.lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let mut parts = line.split(',');
                Some(ChinaMediaEntry {
                    path: parts.next()?.to_string(),
                    hash: parts.next()?.to_string(),
                    media_type: parts.next()?.parse().ok()?,
                    bytes: parts.next()?.parse().ok()?
                })
            })
            .collect()
    }
}
