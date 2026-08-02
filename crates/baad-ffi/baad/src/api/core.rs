use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use baad::download::{ResourceCategory, ResourceDownloader, ResourceFilter};
use baad_shared::Downloads;

pub(crate) const CATEGORY_ASSETS: u8 = 1;
pub(crate) const CATEGORY_TABLES: u8 = 2;
pub(crate) const CATEGORY_MEDIA: u8 = 4;

pub(crate) struct DownloadOptions {
    pub output_dir: PathBuf,
    pub limit: usize,
    pub retries: u32,
    pub proxy: Option<String>
}

pub(crate) fn category_bits(assets: bool, tables: bool, media: bool) -> u8 {
    [(assets, CATEGORY_ASSETS), (tables, CATEGORY_TABLES), (media, CATEGORY_MEDIA)]
        .into_iter()
        .filter_map(|(selected, bit)| selected.then_some(bit))
        .fold(0, |bits, bit| bits | bit)
}

pub(crate) fn resource_category(bits: u8) -> ResourceCategory {
    ResourceCategory::new()
        .include_if(bits & CATEGORY_ASSETS != 0, ResourceCategory::Assets)
        .include_if(bits & CATEGORY_TABLES != 0, ResourceCategory::Tables)
        .include_if(bits & CATEGORY_MEDIA != 0, ResourceCategory::Media)
        .or_all_if_empty()
}

pub(crate) async fn run_download(
    options: DownloadOptions,
    downloads: Downloads,
    filter: Option<&Mutex<ResourceFilter>>
) -> Result<(), baad::CatalogError> {
    let downloader = ResourceDownloader::builder()
        .output_dir(options.output_dir)
        .limit(options.limit)
        .retries(options.retries)
        .maybe_proxy(options.proxy)
        .build();

    let matcher = filter.map(|filter| filter.lock().unwrap_or_else(PoisonError::into_inner));
    downloader.download(downloads, matcher.as_deref()).await
}
