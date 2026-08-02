use std::path::Path;
use std::sync::Arc;

use baad_shared::DownloadObserver;
use bon::Builder;
use derive_more::Debug;
use reqwest_middleware::reqwest::Proxy;
use reqwest_middleware::reqwest::header::HeaderMap;

#[derive(Builder, Clone, Debug)]
pub struct DownloaderConfig<'a> {
    pub directory: &'a Path,

    #[builder(default = 32)]
    pub concurrent_downloads: usize,

    #[builder(default = 16)]
    pub max_chunks_per_file: usize,

    #[builder(default = 8)]
    pub max_concurrent_chunks: usize,

    #[builder(default = 10 * 1024 * 1024)]
    pub chunk_threshold: u64,

    #[builder(default = 3)]
    pub retries: u32,

    #[builder(default = true)]
    pub resumable: bool,

    #[builder(default = false)]
    pub overwrite: bool,

    #[debug(skip)]
    pub headers: Option<HeaderMap>,

    #[debug(skip)]
    pub proxy: Option<Proxy>,

    #[debug(skip)]
    #[builder(default = baad_shared::observer())]
    pub observer: Arc<dyn DownloadObserver>
}
