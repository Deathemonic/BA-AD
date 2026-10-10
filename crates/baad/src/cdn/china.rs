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
use crate::strategy::ChinaStrategy;

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
            Self::catalog_file(bundle, hash, &fconcat!(platform, "/bundleDownloadInfo.json"))?;

        self.fetch_json(&file).await
    }

    pub async fn fetch_table(&self) -> Result<TableCatalogCN, CatalogError> {
        let url = fconcat!("/"; self.catalog_url.as_str(), "Manifest", const { TABLE_BUNDLES }, self.table_version.as_str(), "TableManifest");
        let hash_url = fconcat!(url.as_str(), "Hash");
        let file = Self::catalog_file(url, hash_url, "TableManifest.json")?;
        self.fetch_json(&file).await
    }

    pub async fn fetch_media(&self) -> Result<MediaCatalogCN, CatalogError> {
        let url = fconcat!("/"; self.catalog_url.as_str(), "Manifest", const { MEDIA_RESOURCES }, self.media_version.as_str(), "MediaManifest");
        let hash_url = fconcat!(url.as_str(), "Hash");
        let file = Self::catalog_file(url, hash_url, "MediaManifest.txt")?;
        let downloaded = cache::ensure_cached(&file).await?;
        // Older packs used extensionless keys and dropped paired ACB/AWB
        // entries.
        let pack_path = file.path.with_extension("v2.bytes");

        if !downloaded && let Some(value) = cache::read_pack::<MediaCatalogCN>(&pack_path).await {
            return Ok(value);
        }

        let bytes = fs::read(&file.path).await?;
        let text = String::from_utf8_lossy(&bytes);
        let catalog = MediaCatalogCN {
            table: Self::parse_media(&text).into_iter().map(Self::media_entry).collect()
        };

        cache::write_pack(&pack_path, &catalog).await?;
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

    fn media_entry(entry: MediaCN) -> (String, MediaCN) {
        (ChinaStrategy::media_path(&entry.path, entry.media_type), entry)
    }

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

#[cfg(test)]
mod tests {
    use baad_shared::MediaCatalogCN;

    use super::ChinaCdn;
    use crate::strategy::ChinaStrategy;

    #[test]
    fn paired_story_audio_preserves_both_downloads() {
        let manifest = "audio/voc_cn/scenario/21040,37c7d06d0621c59a703f7a328ab94e1a,5,18688,\n\
                        audio/voc_cn/scenario/21040,02c5554f35578799d8aeb9dc18ee237f,6,5851136,\n";
        let catalog = MediaCatalogCN {
            table: ChinaCdn::parse_media(manifest).into_iter().map(ChinaCdn::media_entry).collect()
        };
        let mut downloads = ChinaStrategy::build_media_downloads(catalog, "https://example.com");
        downloads.sort_by(|left, right| left.path.cmp(&right.path));

        assert_eq!(downloads.len(), 2);
        assert_eq!(downloads[0].path, "audio/voc_cn/scenario/21040.acb");
        assert_eq!(downloads[0].size, 18_688);
        assert!(downloads[0].url.ends_with("/37/37c7d06d0621c59a703f7a328ab94e1a"));
        assert_eq!(downloads[1].path, "audio/voc_cn/scenario/21040.awb");
        assert_eq!(downloads[1].size, 5_851_136);
        assert!(downloads[1].url.ends_with("/02/02c5554f35578799d8aeb9dc18ee237f"));
    }
}
