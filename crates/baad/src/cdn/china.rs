use baad_shared::{
    ASSET_BUNDLES,
    CatalogError,
    ChinaBundleCatalog,
    ChinaMediaEntry,
    ChinaTableCatalog,
    MEDIA_RESOURCES,
    Platform,
    TABLE_BUNDLES
};
use baad_utils::file::get_data_path;
use baad_utils::json::load;
use memorypack::{MemoryPackDeserialize, MemoryPackSerialize};
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::cdn::cache;
use crate::cdn::cache::CatalogFile;
use crate::download::ResourceCategory;

pub struct ChinaCdn {
    pub(crate) catalog_url: String,
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
    ) -> Self {
        Self {
            catalog_url,
            platform,
            resource_version,
            table_version,
            media_version
        }
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
        let file = self.catalog_file(
            format!("{base}/bundleDownloadInfo.json"),
            format!("{base}/bundleDownloadInfo.hash"),
            "bundleDownloadInfo.json"
        )?;
        self.fetch_json(&file).await
    }

    pub async fn fetch_table(&self) -> Result<ChinaTableCatalog, CatalogError> {
        let url = format!(
            "{}/Manifest/{}/{}/TableManifest",
            self.catalog_url, TABLE_BUNDLES, self.table_version
        );
        let hash_url = format!("{url}Hash");
        let file = self.catalog_file(url, hash_url, "TableManifest.json")?;
        self.fetch_json(&file).await
    }

    pub async fn fetch_media(&self) -> Result<Vec<ChinaMediaEntry>, CatalogError> {
        let url = format!(
            "{}/Manifest/{}/{}/MediaManifest",
            self.catalog_url, MEDIA_RESOURCES, self.media_version
        );
        let hash_url = format!("{url}Hash");
        let file = self.catalog_file(url, hash_url, "MediaManifest.txt")?;
        let downloaded = cache::ensure_cached(&file).await?;

        if !downloaded
            && let Some(value) = cache::read_pack::<Vec<ChinaMediaEntry>>(&file.pack_path()).await
        {
            return Ok(value);
        }

        let bytes = fs::read(&file.path).await?;
        let text = String::from_utf8_lossy(&bytes);
        let entries = Self::parse_media(&text);
        cache::write_pack(&file.pack_path(), &entries).await?;
        Ok(entries)
    }

    async fn fetch_json<T>(&self, file: &CatalogFile) -> Result<T, CatalogError>
    where
        T: DeserializeOwned + MemoryPackSerialize + MemoryPackDeserialize
    {
        let downloaded = cache::ensure_cached(file).await?;

        if !downloaded && let Some(value) = cache::read_pack::<T>(&file.pack_path()).await {
            return Ok(value);
        }

        let value = load::<T>(&file.path).await?;
        cache::write_pack(&file.pack_path(), &value).await?;
        Ok(value)
    }

    fn catalog_file(
        &self,
        url: String,
        hash_url: String,
        filename: &str
    ) -> Result<CatalogFile, CatalogError> {
        let key = format!("catalog/china/{}/{}", self.platform.as_ref(), filename);
        Ok(CatalogFile {
            url,
            hash_url,
            path: get_data_path(&key)?
        })
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
