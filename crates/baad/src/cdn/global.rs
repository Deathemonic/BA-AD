use baad_shared::GlobalCatalog as GlobalCatalogData;
use baad_shared::platform::Platform;
use baad_utils::file::get_data_path;
use baad_utils::json::load;
use fastcat::fconcat;
use tokio::fs;
use tracing::{debug, warn};

use crate::cdn::cache;
use crate::download::download_file;
use crate::error::CatalogError;

pub struct GlobalCdn {
    catalog_url: String,
    platform: Platform
}

impl GlobalCdn {
    pub const fn new(catalog_url: String, platform: Platform) -> Self {
        Self { catalog_url, platform }
    }

    pub async fn fetch(&self) -> Result<GlobalCatalogData, CatalogError> {
        let filename = Self::catalog_filename(&self.catalog_url)?;
        let path =
            get_data_path(&fconcat!("/"; "catalog", "global", self.platform.as_ref(), filename))?;
        let marker = path.with_extension("url");
        let pack = path.with_extension("bytes");

        let cached_url = fs::read_to_string(&marker).await.ok().map(|url| String::from(url.trim()));
        let outdated = !path.exists() || cached_url.as_deref() != Some(self.catalog_url.as_str());

        if !outdated {
            if let Some(catalog) = cache::read_pack::<GlobalCatalogData>(&pack).await {
                debug!("Catalog up to date");
                return Ok(catalog);
            }

            if let Ok(catalog) = load::<GlobalCatalogData>(&path).await {
                debug!("Catalog up to date, rebuilding cache");
                cache::write_pack(&pack, &catalog).await?;
                return Ok(catalog);
            }
            warn!("Catalog invalid, refetching...");
        }

        debug!("Catalog changed, downloading");
        download_file(&self.catalog_url, &path, None, 5).await?;
        fs::write(&marker, &self.catalog_url).await?;
        let catalog = load::<GlobalCatalogData>(&path).await?;
        cache::write_pack(&pack, &catalog).await?;

        Ok(catalog)
    }

    fn catalog_filename(catalog_url: &str) -> Result<&str, CatalogError> {
        catalog_url
            .rsplit('/')
            .next()
            .filter(|f| !f.is_empty())
            .ok_or(CatalogError::DeserializationFailed)
    }

    pub fn derive_base_url(resource_path: &str) -> Option<&str> {
        resource_path.rfind('/').map(|pos| &resource_path[..=pos])
    }
}
