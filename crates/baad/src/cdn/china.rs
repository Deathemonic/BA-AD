use baad_shared::{
    ASSET_BUNDLES,
    BundleCatalogCN,
    ChinaMediaType,
    MEDIA_RESOURCES,
    MediaCN,
    MediaCatalogCN,
    Platform,
    TABLE_BUNDLES,
    TableCatalogCN
};
use baad_utils::file::get_data_path;
use baad_utils::json::load;
use fastcat::fconcat;
use memorypack::{MemoryPackDeserialize, MemoryPackSerialize};
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::cdn::cache;
use crate::cdn::cache::CatalogFile;
use crate::download::ResourceCategory;
use crate::error::CatalogError;

pub struct ChinaCdn {
    pub(crate) catalog_url: String,
    platform: Platform,
    resource_version: String,
    table_version: String,
    media_version: String
}

pub struct ChinaResources {
    pub assets: Option<BundleCatalogCN>,
    pub table: Option<TableCatalogCN>,
    pub media: Option<MediaCatalogCN>
}

impl ChinaCdn {
    pub const fn new(
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

    pub async fn fetch(&self, category: ResourceCategory) -> Result<ChinaResources, CatalogError> {
        Ok(ChinaResources {
            assets: if category.contains(ResourceCategory::Assets) {
                Some(self.fetch_assets().await?)
            } else {
                None
            },
            table: if category.contains(ResourceCategory::Tables) {
                Some(self.fetch_table().await?)
            } else {
                None
            },
            media: if category.contains(ResourceCategory::Media) {
                Some(self.fetch_media().await?)
            } else {
                None
            }
        })
    }

    pub async fn fetch_assets(&self) -> Result<BundleCatalogCN, CatalogError> {
        let platform = self.platform.as_ref();
        let base = fconcat!("/"; self.catalog_url.as_str(), const { ASSET_BUNDLES }, "Catalog", self.resource_version.as_str(), self.platform.display_name());
        let bundle = fconcat!(base.as_str(), "/bundleDownloadInfo.json");
        let hash = fconcat!(base.as_str(), "/bundleDownloadInfo.hash");
        let file =
            self.catalog_file(bundle, hash, &fconcat!(platform, "/bundleDownloadInfo.json"))?;

        self.fetch_json(&file).await
    }

    pub async fn fetch_table(&self) -> Result<TableCatalogCN, CatalogError> {
        let url = fconcat!("/"; self.catalog_url.as_str(), "Manifest", const { TABLE_BUNDLES }, self.table_version.as_str(), "TableManifest");
        let hash_url = fconcat!(url.as_str(), "Hash");
        let file = self.catalog_file(url, hash_url, "TableManifest.json")?;
        self.fetch_json(&file).await
    }

    pub async fn fetch_media(&self) -> Result<MediaCatalogCN, CatalogError> {
        let url = fconcat!("/"; self.catalog_url.as_str(), "Manifest", const { MEDIA_RESOURCES }, self.media_version.as_str(), "MediaManifest");
        let hash_url = fconcat!(url.as_str(), "Hash");
        let file = self.catalog_file(url, hash_url, "MediaManifest.txt")?;
        let downloaded = cache::ensure_cached(&file).await?;

        if !downloaded
            && let Some(value) = cache::read_pack::<MediaCatalogCN>(&file.pack_path()).await
        {
            return Ok(value);
        }

        let bytes = fs::read(&file.path).await?;
        let text = String::from_utf8_lossy(&bytes);
        let catalog = MediaCatalogCN {
            table: Self::parse_media(&text).into_iter().map(Self::media_entry).collect()
        };

        cache::write_pack(&file.pack_path(), &catalog).await?;
        Ok(catalog)
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
        let key = fconcat!("/"; "catalog", "china", filename);
        Ok(CatalogFile {
            url,
            hash_url,
            path: get_data_path(&key)?
        })
    }

    fn media_entry(entry: MediaCN) -> (String, MediaCN) { (entry.path.clone(), entry) }

    fn parse_media(text: &str) -> Vec<MediaCN> {
        text.lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let mut parts = line.split(',');
                let path = parts.next()?.into();
                let hash = parts.next()?.into();
                let media_type = ChinaMediaType::from_repr(parts.next()?.parse().ok()?)?;
                let size = parts.next().and_then(|size| size.parse().ok()).unwrap_or(0);

                Some(MediaCN {
                    path,
                    hash,
                    media_type,
                    size
                })
            })
            .collect()
    }
}
