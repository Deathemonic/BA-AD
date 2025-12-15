use crate::download::{ResourceCategory, ResourceDownloader, ResourceFilter};
use crate::helpers::{
    CatalogError, GameFiles, GlobalGameResources, JapanGameResources, ServerRegion,
};
use crate::utils::json;

use hashbrown::HashSet;
use std::future::Future;
use std::path::Path;
use std::rc::Rc;
use trauma::Download;

pub(crate) enum GameResources {
    Japan(JapanGameResources),
    Global(GlobalGameResources),
}

impl GameResources {
    pub(crate) fn get_asset_bundles(
        &self,
        file_names: &mut HashSet<Rc<str>>,
    ) -> Result<(), CatalogError> {
        match self {
            GameResources::Japan(res) => {
                for asset_bundle in &res.asset_bundles {
                    add_filename(file_names, &asset_bundle.path);
                    file_names.extend(asset_bundle.bundle_files.iter().map(|s| s.as_str().into()));
                }
            }
            GameResources::Global(res) => {
                extract_filenames(file_names, &res.asset_bundles);
            }
        }
        Ok(())
    }

    pub(crate) fn get_table_bundles(&self) -> &[GameFiles] {
        match self {
            GameResources::Japan(res) => &res.table_bundles,
            GameResources::Global(res) => &res.table_bundles,
        }
    }

    pub(crate) fn get_media_resources(&self) -> &[GameFiles] {
        match self {
            GameResources::Japan(res) => &res.media_resources,
            GameResources::Global(res) => &res.media_resources,
        }
    }

    pub(crate) fn get_downloads(
        &self,
        category: &ResourceCategory,
        filter: Option<&ResourceFilter>,
    ) -> Vec<Download> {
        match category {
            ResourceCategory::Assets => self.process_assets(filter),
            ResourceCategory::Tables => self.process_tables(filter),
            ResourceCategory::Media => self.process_media(filter),
            ResourceCategory::All => self
                .process_assets(filter)
                .into_iter()
                .chain(self.process_tables(filter))
                .chain(self.process_media(filter))
                .collect(),
            ResourceCategory::Multiple(cats) => cats
                .iter()
                .flat_map(|cat| self.get_downloads(cat, filter))
                .collect(),
        }
    }

    fn process_assets(&self, filter: Option<&ResourceFilter>) -> Vec<Download> {
        match self {
            GameResources::Japan(res) => {
                ResourceDownloader::process_japan_assets(&res.asset_bundles, filter)
            }
            GameResources::Global(res) => ResourceDownloader::process_files(
                &res.asset_bundles,
                filter,
                |f| &f.url,
                |f| &f.path,
                |f| &f.hash,
                |_| None,
            ),
        }
    }

    fn process_tables(&self, filter: Option<&ResourceFilter>) -> Vec<Download> {
        ResourceDownloader::process_files(
            self.get_table_bundles(),
            filter,
            |f| &f.url,
            |f| &f.path,
            |f| &f.hash,
            |_| None,
        )
    }

    fn process_media(&self, filter: Option<&ResourceFilter>) -> Vec<Download> {
        ResourceDownloader::process_files(
            self.get_media_resources(),
            filter,
            |f| &f.url,
            |f| &f.path,
            |f| &f.hash,
            |_| None,
        )
    }
}

pub(crate) async fn load_resources(
    region: &ServerRegion,
    path: &Path,
) -> Result<GameResources, CatalogError> {
    match region {
        ServerRegion::Japan => {
            let resources = json::load_json::<JapanGameResources>(path).await?;
            Ok(GameResources::Japan(resources))
        }
        ServerRegion::Global => {
            let resources = json::load_json::<GlobalGameResources>(path).await?;
            Ok(GameResources::Global(resources))
        }
    }
}

pub(crate) fn add_filename(file_names: &mut HashSet<Rc<str>>, path: &str) {
    if let Some(filename) = Path::new(path).file_name()
        && let Some(name_str) = filename.to_str()
    {
        file_names.insert(Rc::from(name_str));
    }
}

pub(crate) fn extract_filenames(file_names: &mut HashSet<Rc<str>>, items: &[GameFiles]) {
    for item in items {
        add_filename(file_names, &item.path);
    }
}

pub(crate) async fn combine_categories<'a, F, Fut>(
    file_names: &mut HashSet<Rc<str>>,
    list_assets_fn: F,
    categories: &'a [ResourceCategory],
) -> Result<(), CatalogError>
where
    F: Fn(&'a ResourceCategory) -> Fut,
    Fut: Future<Output = Result<HashSet<Rc<str>>, CatalogError>>,
{
    for category in categories {
        let cat_files = list_assets_fn(category).await?;
        file_names.extend(cat_files);
    }

    Ok(())
}
