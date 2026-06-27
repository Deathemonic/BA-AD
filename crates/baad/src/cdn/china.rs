use std::path::Path;

use baad_core::{
    ASSET_BUNDLES,
    CatalogError,
    ChinaBundleCatalog,
    ChinaMediaEntry,
    ChinaTableCatalog,
    MEDIA_RESOURCES,
    Platform,
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
    catalog_url: String,
    platform: Platform,
    resource_version: String,
    table_version: String,
    media_version: String
}

pub struct ChinaResources {
    pub assets: Option<ChinaBundleCatalog>,
    pub table: Option<ChinaTableCatalog>,
    pub media: Option<Vec<ChinaMediaEntry>>
}

impl ChinaCdn {
    pub fn new(
        catalog_url: String,
        platform: Platform,
        resource_version: String,
        table_version: String,
        media_version: String
    ) -> Result<Self, ServerConfigError> {
        if platform == Platform::Windows {
            return Err(ServerConfigError::WindowsNotSupported);
        }

        Ok(Self {
            catalog_url: catalog_url.trim_end_matches('/').to_string(),
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

    pub async fn fetch_assets(&self) -> Result<ChinaBundleCatalog, CatalogError> {
        let base = format!(
            "{}/{}/Catalog/{}/{}",
            self.catalog_url,
            ASSET_BUNDLES,
            self.resource_version,
            self.platform.display_name()
        );
        let url = format!("{base}/bundleDownloadInfo.json");
        let hash_url = format!("{base}/bundleDownloadInfo.hash");
        self.fetch_json(&url, &hash_url, "bundleDownloadInfo.json").await
    }

    pub async fn fetch_table(&self) -> Result<ChinaTableCatalog, CatalogError> {
        let url = format!(
            "{}/Manifest/{}/{}/TableManifest",
            self.catalog_url,
            TABLE_BUNDLES,
            self.table_version
        );
        let hash_url = format!("{url}Hash");
        self.fetch_json(&url, &hash_url, "TableManifest.json").await
    }

    pub async fn fetch_media(&self) -> Result<Vec<ChinaMediaEntry>, CatalogError> {
        let url = format!(
            "{}/Manifest/{}/{}/MediaManifest",
            self.catalog_url,
            MEDIA_RESOURCES,
            self.media_version
        );
        let hash_url = format!("{url}Hash");
        let bytes = self.fetch_bytes(&url, &hash_url, "MediaManifest.txt").await?;
        let text = String::from_utf8_lossy(&bytes);
        Ok(Self::parse_media(&text))
    }

    async fn fetch_json<T: DeserializeOwned>(
        &self,
        url: &str,
        hash_url: &str,
        filename: &str
    ) -> Result<T, CatalogError> {
        let path = get_data_path(&self.cache_key(filename))?;
        self.ensure_cached(url, hash_url, &path, filename).await?;
        Ok(load::<T>(&path).await?)
    }

    async fn fetch_bytes(
        &self,
        url: &str,
        hash_url: &str,
        filename: &str
    ) -> Result<Vec<u8>, CatalogError> {
        let path = get_data_path(&self.cache_key(filename))?;
        self.ensure_cached(url, hash_url, &path, filename).await?;
        Ok(fs::read(&path).await?)
    }

    async fn ensure_cached(
        &self,
        url: &str,
        hash_url: &str,
        path: &Path,
        filename: &str
    ) -> Result<(), CatalogError> {
        let hash_path = path.with_extension("hash");

        let remote = Self::remote_hash(hash_url).await;
        let local = Self::local_hash(&hash_path).await;

        let outdated = !path.exists()
            || match (&remote, &local) {
                (Some(remote), Some(local)) => remote != local,
                (Some(_), None) => true,
                (None, _) => false
            };

        if outdated {
            debug!(filename, "Catalog outdated, fetching...");
            download_file(url, path, None, 3).await?;
            if let Some(remote) = &remote {
                fs::write(&hash_path, remote).await?;
            }
        } else {
            debug!(filename, "Catalog up to date, using cache");
        }

        Ok(())
    }

    fn cache_key(&self, filename: &str) -> String {
        format!("catalog/china/{}/{}", self.platform.as_ref(), filename)
    }

    async fn remote_hash(url: &str) -> Option<String> {
        let response = client().get(url).send().await.ok()?.error_for_status().ok()?;
        let text = response.text().await.ok()?;
        Some(text.trim().to_string())
    }

    async fn local_hash(path: &Path) -> Option<String> {
        fs::read_to_string(path).await.ok().map(|hash| hash.trim().to_string())
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
