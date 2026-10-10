use baad_shared::{
    CATALOG_PREFIX,
    DownloadAsset,
    DownloadMedia,
    DownloadTable,
    Downloads,
    HashValue,
    MEDIA_RESOURCES,
    Resource,
    TABLE_BUNDLES
};
use fastcat::fconcat;
use tracing::warn;

use crate::download::ResourceCategory;

pub struct GlobalStrategy;

impl GlobalStrategy {
    pub fn build_downloads(
        resources: Vec<Resource>,
        base_url: &str,
        category: ResourceCategory
    ) -> Downloads {
        let mut assets = Vec::new();
        let mut tables = Vec::new();
        let mut media = Vec::new();

        for resource in resources {
            if resource.resource_path.starts_with(CATALOG_PREFIX) {
                continue;
            }

            let url = fconcat!(base_url, resource.resource_path.as_str());
            let hash = HashValue::Md5(resource.resource_hash);
            let size = resource.resource_size;

            if resource.resource_path.contains(TABLE_BUNDLES) {
                if category.contains(ResourceCategory::Tables)
                    && let Some(path) = Self::output_path(&resource.resource_path, TABLE_BUNDLES)
                {
                    tables.push(DownloadTable {
                        url,
                        path,
                        hash,
                        size,
                        bundle_files: Vec::new()
                    });
                }
            } else if resource.resource_path.contains(MEDIA_RESOURCES) {
                if category.contains(ResourceCategory::Media)
                    && let Some(path) = Self::output_path(&resource.resource_path, MEDIA_RESOURCES)
                {
                    media.push(DownloadMedia { url, path, hash, size });
                }
            } else if category.contains(ResourceCategory::Assets) {
                assets.push(DownloadAsset {
                    url,
                    path: resource.resource_path,
                    hash,
                    size,
                    bundle_files: Vec::new()
                });
            }
        }

        Downloads { assets, tables, media }
    }

    fn output_path(resource_path: &str, category: &str) -> Option<String> {
        let path = Self::category_path(resource_path, category).map(String::from);
        if path.is_none() {
            warn!(path = resource_path, "Skipping resource with an unsafe path");
        }
        path
    }

    /// Returns the path below the `category` directory, such as
    /// `Audio/VOC_JP/JP_Momoi/Momoi_Title.ogg` for
    /// `GameData/MediaResources/Audio/VOC_JP/JP_Momoi/Momoi_Title.ogg`.
    /// Keeping the subdirectories prevents files that share a name across
    /// languages from being written to the same output path. Paths that could
    /// leave the output directory are rejected.
    fn category_path<'a>(resource_path: &'a str, category: &str) -> Option<&'a str> {
        let unsafe_segment =
            |segment: &str| matches!(segment, "" | "." | "..") || segment.contains(['\\', ':']);
        if resource_path.split('/').any(unsafe_segment) {
            return None;
        }
        let mut rest = resource_path;
        while let Some((segment, tail)) = rest.split_once('/') {
            if segment == category {
                return Some(tail);
            }
            rest = tail;
        }
        Some(resource_path)
    }
}

#[cfg(test)]
mod tests {
    use baad_shared::{HashValue, Resource};

    use super::GlobalStrategy;
    use crate::download::ResourceCategory;

    fn resource(path: &str, hash: &str, size: i64) -> Resource {
        Resource {
            resource_path: path.into(),
            resource_hash: hash.into(),
            resource_size: size,
            ..Resource::default()
        }
    }

    #[test]
    fn same_named_media_in_different_languages_keep_separate_paths() {
        let resources = vec![
            resource(
                "Preload/MediaResources/Audio/VOC_KR/KR_Momoi/Momoi_Title.ogg",
                "77520526fad0308ebb8f52727400dabd",
                13_349
            ),
            resource(
                "Preload/MediaResources/Audio/VOC_JP/JP_Momoi/Momoi_Title.ogg",
                "ef5d7848cd4dfb566a95b8c6d0d73578",
                16_485
            ),
            resource("GameData/TableBundles/ExcelDB.db", "00000000000000000000000000000000", 1),
            resource("Catalog/MediaResources/MediaCatalog.bytes", "", 1),
        ];
        let downloads = GlobalStrategy::build_downloads(
            resources,
            "https://cdn.example/",
            ResourceCategory::ALL
        );

        let media: Vec<_> = downloads.media.iter().map(|media| media.path.as_str()).collect();
        assert_eq!(media, [
            "Audio/VOC_KR/KR_Momoi/Momoi_Title.ogg",
            "Audio/VOC_JP/JP_Momoi/Momoi_Title.ogg"
        ]);
        assert!(
            matches!(&downloads.media[1].hash, HashValue::Md5(hash) if hash.starts_with("ef5d"))
        );
        assert_eq!(
            downloads.media[1].url,
            "https://cdn.example/Preload/MediaResources/Audio/VOC_JP/JP_Momoi/Momoi_Title.ogg"
        );
        assert_eq!(downloads.tables[0].path, "ExcelDB.db");
    }

    #[test]
    fn category_must_be_a_whole_path_segment() {
        assert_eq!(
            GlobalStrategy::category_path("GameData/NotMediaResources/a/b.ogg", "MediaResources"),
            Some("GameData/NotMediaResources/a/b.ogg")
        );
        assert_eq!(
            GlobalStrategy::category_path("MediaResources/a.ogg", "MediaResources"),
            Some("a.ogg")
        );
    }

    #[test]
    fn paths_that_could_escape_the_output_directory_are_rejected() {
        for path in [
            "GameData/MediaResources/../../outside.ogg",
            "GameData/MediaResources/Audio//a.ogg",
            "GameData/MediaResources/C:/a.ogg",
            "GameData/MediaResources/Audio\\..\\a.ogg",
            "/GameData/MediaResources/a.ogg"
        ] {
            assert_eq!(GlobalStrategy::category_path(path, "MediaResources"), None, "{path}");
        }
        let downloads = GlobalStrategy::build_downloads(
            vec![resource("GameData/MediaResources/../a.ogg", "", 1)],
            "https://cdn.example/",
            ResourceCategory::ALL
        );
        assert!(downloads.media.is_empty());
    }
}
