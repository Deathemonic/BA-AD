use baad_shared::GlobalCatalog as GlobalCatalogData;
use baad_shared::platform::Platform;
use baad_utils::file::get_data_path;
use fastcat::fconcat;

use crate::cdn::cache;
use crate::cdn::cache::CatalogFile;
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
        let file = CatalogFile {
            url: self.catalog_url.clone(),
            hash_url: None,
            path
        };
        cache::fetch_json(&file).await
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
