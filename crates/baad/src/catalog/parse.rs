use crate::download::ResourceCategory;
use crate::helpers::{
    ApiData, AssetBundle, CatalogError, GameFiles, GameFilesBundles, GlobalCatalog,
    GlobalGameResources, HashValue, JapanGameResources, MediaResources, Platform, Resource,
    ServerConfig, ServerRegion, TableResources,
};
use crate::utils::{
    catalog::{combine_categories, extract_filenames, load_resources},
    json,
};

use baad_core::{file, info};
use bacy::catalog::{MediaCatalog, Packing, TableCatalog, deserialize_table_catalog, deserialize_media_catalog};
use hashbrown::HashSet;
use reqwest::Client;
use std::path::Path;
use std::rc::Rc;

struct Paths {
    asset_path: Box<Path>,
    table_path: Box<Path>,
    media_path: Box<Path>,
    game_path: Box<Path>,
    resource_path: Box<Path>,
    api_path: Box<Path>,
    patch_pack_path: &'static str,
    platform_path: &'static str,
}

pub struct CatalogParser {
    client: Client,
    config: Rc<ServerConfig>,
    paths: Paths,
}

impl CatalogParser {
    pub fn new(config: Rc<ServerConfig>) -> Result<Self, CatalogError> {
        let client = Client::new();
        let data_dir = file::data_dir()?;

        let api_path = data_dir.join("api_data.json");

        let catalog_dir = match config.region {
            ServerRegion::Global => data_dir.join("catalog/global"),
            ServerRegion::Japan => data_dir.join("catalog/japan"),
        };

        let asset_path = catalog_dir.join("BundlePackingInfo.json");
        let table_path = catalog_dir.join("TableCatalog.json");
        let media_path = catalog_dir.join("MediaCatalog.json");
        let game_path = catalog_dir.join("GameFiles.json");
        let resource_path = catalog_dir.join("Resources.json");

        let patch_pack_path = match config.platform {
            Platform::Android => "Android_PatchPack",
            Platform::Ios => "iOS_PatchPack",
        };

        let platform_path = match config.platform {
            Platform::Android => "/Android/",
            Platform::Ios => "/iOS/",
        };

        Ok(Self {
            client,
            config,
            paths: Paths {
                asset_path: asset_path.into_boxed_path(),
                table_path: table_path.into_boxed_path(),
                media_path: media_path.into_boxed_path(),
                game_path: game_path.into_boxed_path(),
                resource_path: resource_path.into_boxed_path(),
                api_path: api_path.into_boxed_path(),
                patch_pack_path,
                platform_path,
            },
        })
    }

    async fn japan_data(&self, catalog_url: &str) -> Result<(), CatalogError> {
        let asset_data = self
            .client
            .get(format!(
                "{}/{}/BundlePackingInfo.json",
                catalog_url, self.paths.patch_pack_path
            ))
            .send()
            .await?
            .json::<Packing>()
            .await?;

        json::save_json(&self.paths.asset_path, &asset_data).await?;

        info!(success = true, "Saved AssetBundles catalog");

        let table_bytes = self
            .client
            .get(format!("{}/TableBundles/TableCatalog.bytes", catalog_url))
            .send()
            .await?
            .bytes()
            .await?;

        let table_data = deserialize_table_catalog(&table_bytes)?;
        json::save_json(&self.paths.table_path, &table_data).await?;

        info!(success = true, "Saved TableBundles catalog");

        let media_bytes = self
            .client
            .get(format!("{}/MediaResources/Catalog/MediaCatalog.bytes", catalog_url))
            .send()
            .await?
            .bytes()
            .await?;

        let media_data = deserialize_media_catalog(&media_bytes)?;
        json::save_json(&self.paths.media_path, &media_data).await?;

        info!(success = true, "Saved MediaResources catalog");

        Ok(())
    }

    async fn japan_gamefiles(&self, catalog_url: &str) -> Result<(), CatalogError> {
        let bundle_info: Packing = json::load_json(&self.paths.asset_path).await?;
        let table_catalog: TableCatalog = json::load_json(&self.paths.table_path).await?;
        let media_catalog: MediaCatalog = json::load_json(&self.paths.media_path).await?;

        let game_resources = JapanGameResources {
            asset_bundles: bundle_info
                .full_patch_packs
                .into_iter()
                .chain(bundle_info.update_packs)
                .map(|patch| GameFilesBundles {
                    url: format!(
                        "{}/{}/{}",
                        catalog_url, self.paths.patch_pack_path, patch.pack_name
                    ),
                    path: format!("AssetBundles/{}", patch.pack_name),
                    hash: HashValue::Crc(patch.crc),
                    size: patch.pack_size,
                    bundle_files: patch
                        .bundle_files
                        .into_iter()
                        .map(|bundle| bundle.name)
                        .collect(),
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

        json::save_json(&self.paths.game_path, &game_resources).await?;

        info!(success = true, "Saved GameFiles");

        Ok(())
    }

    async fn global_data(&self, resources: &[Resource]) -> Result<(), CatalogError> {
        let mut asset_bundles = Vec::new();
        let mut media_resources = Vec::new();
        let mut table_bundles = Vec::new();

        for resource in resources {
            if resource.resource_path.contains(self.paths.platform_path) {
                asset_bundles.push(resource.clone());
            }

            if resource.resource_path.contains("/MediaResources/") {
                media_resources.push(resource.clone());
            }

            if resource.resource_path.contains("/TableBundles/") {
                table_bundles.push(resource.clone());
            }
        }

        if !asset_bundles.is_empty() {
            let asset_data = AssetBundle { asset_bundles };
            json::save_json(&self.paths.asset_path, &asset_data).await?;

            info!(success = true, "Saved AssetBundles catalog");
        }

        if !table_bundles.is_empty() {
            let table_data = TableResources { table_bundles };
            json::save_json(&self.paths.table_path, &table_data).await?;

            info!(success = true, "Saved TableBundles catalog");
        }

        if !media_resources.is_empty() {
            let media_data = MediaResources { media_resources };
            json::save_json(&self.paths.media_path, &media_data).await?;

            info!(success = true, "Saved MediaResources catalog");
        }

        Ok(())
    }

    async fn global_gamefiles(
        &self,
        catalog_url: &str,
        resources: &[Resource],
    ) -> Result<(), CatalogError> {
        let game_resources = GlobalGameResources {
            asset_bundles: resources
                .iter()
                .filter(|r| r.resource_path.contains(self.paths.platform_path))
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

        json::save_json(&self.paths.game_path, &game_resources).await?;

        info!(success = true, "Saved GameFiles");

        Ok(())
    }

    fn resource_to_gamefiles(
        &self,
        resource: &Resource,
        catalog_url: &str,
        prefix: &str,
    ) -> GameFiles {
        GameFiles {
            url: format!("{}/{}", catalog_url, resource.resource_path),
            path: format!("{}/{}", prefix, resource.resource_path),
            hash: HashValue::Md5(resource.resource_hash.clone()),
            size: resource.resource_size,
        }
    }

    pub async fn process_catalogs(&self) -> Result<(), CatalogError> {
        let api_data: ApiData = json::load_json(&self.paths.api_path).await?;

        info!("Processing catalogs...");

        match self.config.region {
            ServerRegion::Japan => {
                let catalog_url = &api_data.japan.catalog_url;

                if catalog_url.is_empty() {
                    return Err(CatalogError::EmptyCatalogUrl {
                        region: "Japan".into(),
                    });
                }

                self.japan_data(catalog_url).await?;
                self.japan_gamefiles(catalog_url).await?;
            }
            ServerRegion::Global => {
                let resources: GlobalCatalog = json::load_json(&self.paths.resource_path).await?;
                let catalog_url = &api_data
                    .global
                    .catalog_url
                    .trim_end_matches("/resource-data.json");

                if catalog_url.is_empty() {
                    return Err(CatalogError::EmptyCatalogUrl {
                        region: "Global".into(),
                    });
                }

                self.global_data(&resources.resources).await?;
                self.global_gamefiles(catalog_url, &resources.resources)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn list_assets(
        &self,
        category: &ResourceCategory,
    ) -> Result<HashSet<Rc<str>>, CatalogError> {
        let mut file_names = HashSet::new();
        let resources = load_resources(&self.config.region, &self.paths.game_path).await?;

        match category {
            ResourceCategory::Assets => {
                resources.get_asset_bundles(&mut file_names)?;
            }
            ResourceCategory::Tables => {
                extract_filenames(&mut file_names, resources.get_table_bundles());
            }
            ResourceCategory::Media => {
                extract_filenames(&mut file_names, resources.get_media_resources());
            }
            ResourceCategory::All => {
                combine_categories(
                    &mut file_names,
                    |cat| Box::pin(self.list_assets(cat)),
                    &[
                        ResourceCategory::Assets,
                        ResourceCategory::Tables,
                        ResourceCategory::Media,
                    ],
                )
                .await?;
            }
            ResourceCategory::Multiple(categories) => {
                combine_categories(
                    &mut file_names,
                    |cat| Box::pin(self.list_assets(cat)),
                    categories.as_ref(),
                )
                .await?;
            }
        }

        Ok(file_names)
    }
}
