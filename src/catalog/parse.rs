use crate::helpers::{
    ApiData, AssetBundle, BundleDownloadInfo,
    GameFiles, GameResources, GlobalCatalog,
    HashValue, MediaResources, Resource,
    ServerConfig, ServerRegion, TableResources
};
use crate::utils::json::{load_json, save_json};
use crate::utils::FileManager;

use anyhow::Result;
use baad_core::{errors::{ErrorContext, ErrorExt}, info, success};
use bacy::{MediaCatalog, TableCatalog};
use reqwest::Client;
use std::rc::Rc;

pub struct CatalogParser {
    client: Client,
    file_manager: Rc<FileManager>,
    config: Rc<ServerConfig>,
}

impl CatalogParser {
    pub fn new(file_manager: Rc<FileManager>, config: Rc<ServerConfig>) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            config,
            file_manager,
        })
    }

    async fn japan_data(&self, catalog_url: &str) -> Result<()> {
        let asset_data = self
            .client
            .get(format!("{}/Android/bundleDownloadInfo.json", catalog_url))
            .send()
            .await
            .handle_errors()?
            .json::<BundleDownloadInfo>()
            .await
            .handle_errors()?;


        save_json(
            &self.file_manager,
            "catalog/japan/bundleDownloadInfo.json",
            &asset_data,
        )
        .await?;

        success!("Saved AssetBundles catalog");

        let media_bytes = self
            .client
            .get(format!(
                "{}/MediaResources/Catalog/MediaCatalog.bytes",
                catalog_url
            ))
            .send()
            .await
            .handle_errors()?
            .bytes()
            .await
            .handle_errors()?;
        let media_data = MediaCatalog::deserialize(&media_bytes, catalog_url).handle_errors()?;

        save_json(
            &self.file_manager,
            "catalog/japan/MediaCatalog.json",
            &media_data,
        )
        .await?;

        success!("Saved MediaResources catalog");

        let table_bytes = self
            .client
            .get(format!("{}/TableBundles/TableCatalog.bytes", catalog_url))
            .send()
            .await
            .handle_errors()?
            .bytes()
            .await
            .handle_errors()?;
        let table_data = TableCatalog::deserialize(&table_bytes, catalog_url).handle_errors()?;

        save_json(
            &self.file_manager,
            "catalog/japan/TableCatalog.json",
            &table_data,
        )
        .await?;

        success!("Saved TableBundles catalog");

        Ok(())
    }

    async fn japan_gamefiles(&self, catalog_url: &str) -> Result<()> {
        let bundle_info: BundleDownloadInfo =
            load_json(&self.file_manager, "catalog/japan/bundleDownloadInfo.json").await?;
        let table_catalog: TableCatalog =
            load_json(&self.file_manager, "catalog/japan/TableCatalog.json").await?;
        let media_catalog: MediaCatalog =
            load_json(&self.file_manager, "catalog/japan/MediaCatalog.json").await?;

        let game_resources = GameResources {
            asset_bundles: bundle_info
                .bundle_files
                .into_iter()
                .map(|bundle| GameFiles {
                    url: format!("{}/Android/{}", catalog_url, bundle.name),
                    path: format!("AssetBundles/{}", bundle.name),
                    hash: HashValue::Crc(bundle.crc),
                    size: bundle.size,
                })
                .collect(),

            table_bundles: table_catalog
                .table
                .into_values()
                .map(|entry| GameFiles {
                    url: format!("{}/TableBundles/{}", catalog_url, entry.name),
                    path: format!("TableBundles/{}", entry.name),
                    hash: HashValue::Crc(entry.crc),
                    size: entry.size,
                })
                .collect(),

            media_resources: media_catalog
                .table
                .into_values()
                .map(|entry| {
                    let path = entry.path.replace('\\', "/");
                    GameFiles {
                        url: format!("{}/MediaResources/{}", catalog_url, path),
                        path: format!("MediaResources/{}", path),
                        hash: HashValue::Crc(entry.crc),
                        size: entry.bytes,
                    }
                })
                .collect(),
        };

        save_json(
            &self.file_manager,
            "catalog/japan/GameFiles.json",
            &game_resources,
        )
        .await?;

        success!("Saved GameFiles");
        
        Ok(())
    }

    async fn global_data(&self, resources: &[Resource]) -> Result<()> {
        let asset_bundles: Vec<Resource> = resources
            .iter()
            .filter(|r| r.resource_path.contains("/Android/"))
            .cloned()
            .collect();

        let media_resources: Vec<Resource> = resources
            .iter()
            .filter(|r| r.resource_path.contains("/MediaResources/"))
            .cloned()
            .collect();


        let table_bundles: Vec<Resource> = resources
            .iter()
            .filter(|r| r.resource_path.contains("/TableBundles/"))
            .cloned()
            .collect();
        
        if !asset_bundles.is_empty() {
            let asset_data = AssetBundle { asset_bundles };
            save_json(
                &self.file_manager,
                "catalog/global/bundleDownloadInfo.json",
                &asset_data,
            )
            .await?;

            success!("Saved AssetBundles catalog");
        }

        if !media_resources.is_empty() {
            let media_data = MediaResources { media_resources };
            save_json(
                &self.file_manager,
                "catalog/global/MediaCatalog.json",
                &media_data,
            )
            .await?;
            
            success!("Saved MediaResources catalog");
        }

        if !table_bundles.is_empty() {
            let table_data = TableResources { table_bundles };
            save_json(
                &self.file_manager,
                "catalog/global/TableCatalog.json",
                &table_data,
            )
            .await?;

            success!("Saved TableBundles catalog");
        }

        Ok(())
    }

    async fn global_gamefiles(&self, catalog_url: &str, resources: &[Resource]) -> Result<()> {
        let game_resources = GameResources {
            asset_bundles: resources
                .iter()
                .filter(|r| r.resource_path.contains("/Android/"))
                .map(|r| self.resource_to_gamefiles(r, catalog_url, "AssetBundles"))
                .collect(),

            table_bundles: resources
                .iter()
                .filter(|r| r.resource_path.contains("/TableBundles/"))
                .map(|r| self.resource_to_gamefiles(r, catalog_url, "TableBundles"))
                .collect(),

            media_resources: resources
                .iter()
                .filter(|r| r.resource_path.contains("/MediaResources/"))
                .map(|r| self.resource_to_gamefiles(r, catalog_url, "MediaResources"))
                .collect(),
        };

        save_json(
            &self.file_manager,
            "catalog/global/GameFiles.json",
            &game_resources,
        ).await?;

        success!("Saved GameFiles");
        
        Ok(())
    }

    fn resource_to_gamefiles(&self, resource: &Resource, catalog_url: &str, prefix: &str) -> GameFiles {
        GameFiles {
            url: format!("{}/{}", catalog_url, resource.resource_path),
            path: format!("{}/{}", prefix, resource.resource_path),
            hash: HashValue::Md5(resource.resource_hash.clone()),
            size: resource.resource_size,
        }
    }

    pub async fn process_catalogs(&self) -> Result<()> {
        let api_data: ApiData = load_json(&self.file_manager, "api_data.json").await?;
        
        info!("Processing catalogs...");

        match self.config.region {
            ServerRegion::Japan => {
                let catalog_url = &api_data.japan.catalog_url;

                if catalog_url.is_empty() {
                    return None.error_context("Japan catalog URL is empty - run CatalogFetcher first");
                }

                self.japan_data(catalog_url).await?;
                self.japan_gamefiles(catalog_url).await?;
            }
            ServerRegion::Global => {
                let resources: GlobalCatalog =
                    load_json(&self.file_manager, "catalog/global/Resources.json").await?;
                let catalog_url = &api_data
                    .global
                    .catalog_url
                    .trim_end_matches("/resource-data.json");

                if catalog_url.is_empty() {
                    return None.error_context("Global catalog URL is empty - run CatalogFetcher first");
                }

                self.global_data(&resources.resources).await?;
                self.global_gamefiles(catalog_url, &resources.resources)
                    .await?;
            }
        }

        Ok(())
    }
}