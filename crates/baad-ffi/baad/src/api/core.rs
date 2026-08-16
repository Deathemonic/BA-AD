use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use baad::download::{ResourceCategory, ResourceDownloader, ResourceFilter};
use baad_shared::Downloads;

pub const CATEGORY_ASSETS: u8 = 1;
pub const CATEGORY_TABLES: u8 = 2;
pub const CATEGORY_MEDIA: u8 = 4;

pub struct DownloadOptions {
    pub output_dir: PathBuf,
    pub limit: usize,
    pub retries: u32,
    pub proxy: Option<String>,
    pub http1_only: bool,
    pub max_chunks_per_file: usize,
    pub max_concurrent_chunks: usize,
    pub chunk_threshold: u64
}

pub fn category_bits(assets: bool, tables: bool, media: bool) -> u8 {
    [(assets, CATEGORY_ASSETS), (tables, CATEGORY_TABLES), (media, CATEGORY_MEDIA)]
        .into_iter()
        .filter_map(|(selected, bit)| selected.then_some(bit))
        .fold(0, |bits, bit| bits | bit)
}

pub fn resource_category(bits: u8) -> ResourceCategory {
    ResourceCategory::new()
        .include_if(bits & CATEGORY_ASSETS != 0, ResourceCategory::Assets)
        .include_if(bits & CATEGORY_TABLES != 0, ResourceCategory::Tables)
        .include_if(bits & CATEGORY_MEDIA != 0, ResourceCategory::Media)
        .or_all_if_empty()
}

pub async fn run_download(
    options: DownloadOptions,
    downloads: Downloads,
    filter: Option<&Mutex<ResourceFilter>>
) -> Result<(), baad::CatalogError> {
    let downloader = ResourceDownloader::builder()
        .output_dir(options.output_dir)
        .limit(options.limit)
        .retries(options.retries)
        .maybe_proxy(options.proxy)
        .http1_only(options.http1_only)
        .max_chunks_per_file(options.max_chunks_per_file)
        .max_concurrent_chunks(options.max_concurrent_chunks)
        .chunk_threshold(options.chunk_threshold)
        .build();

    let matcher = filter.map(|filter| filter.lock().unwrap_or_else(PoisonError::into_inner));
    downloader.download(downloads, matcher.as_deref()).await
}
