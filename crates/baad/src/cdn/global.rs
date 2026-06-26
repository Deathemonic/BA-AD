use baad_core::{CatalogError, GlobalCatalog as GlobalCatalogData, Platform};
use baad_utils::file::get_data_path;
use baad_utils::json::load;
use baad_utils::{debug, warn};
use tokio::fs;

use crate::download::download_file;

pub struct GlobalCdn {
    catalog_url: String,
    platform: Platform
}

impl GlobalCdn {
    pub fn new(catalog_url: String, platform: Platform) -> Self {
        Self { catalog_url, platform }
    }

    pub async fn fetch(&self) -> Result<GlobalCatalogData, CatalogError> {
        let filename = Self::catalog_filename(&self.catalog_url)?;
        let path =
            get_data_path(&format!("catalog/global/{}/{}", self.platform.as_ref(), filename))?;
        let marker = path.with_extension("url");

        let cached_url = fs::read_to_string(&marker).await.ok().map(|url| url.trim().to_string());
        let outdated = !path.exists() || cached_url.as_deref() != Some(self.catalog_url.as_str());

        if !outdated {
            match load::<GlobalCatalogData>(&path).await {
                Ok(catalog) => {
                    debug!("Catalog up to date");
                    return Ok(catalog);
                }
                Err(_) => warn!("Cached catalog invalid, refetching")
            }
        }

        debug!("Catalog changed, downloading");
        download_file(&self.catalog_url, &path, None, 3).await?;
        fs::write(&marker, &self.catalog_url).await?;
        let catalog = load::<GlobalCatalogData>(&path).await?;

        Ok(catalog)
    }

    fn catalog_filename(catalog_url: &str) -> Result<&str, CatalogError> {
        catalog_url.rsplit('/').next().filter(|f| !f.is_empty()).ok_or(CatalogError::DeserializationFailed)
    }

    pub fn derive_base_url(resource_path: &str) -> Option<&str> {
        resource_path.rfind('/').map(|pos| &resource_path[..=pos])
    }
}
