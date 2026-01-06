use crate::download::Summary;
use crate::progress::StyleOptions;

use std::env::current_dir;
use std::fmt::{Debug, Formatter, Result};
use std::path::PathBuf;
use std::sync::Arc;

use bon::Builder;
use reqwest_middleware::reqwest::header::HeaderMap;

pub type DownloadCallback = Arc<dyn Fn(&Summary) + Send + Sync>;

#[derive(Builder, Clone)]
pub struct DownloaderConfig {
    #[builder(default = current_dir().unwrap_or_default())]
    pub directory: PathBuf,

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

    #[builder(default)]
    pub style_options: StyleOptions,

    pub headers: Option<HeaderMap>,

    pub proxy: Option<reqwest_middleware::reqwest::Proxy>,

    pub on_complete: Option<DownloadCallback>,
}

impl Debug for DownloaderConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DownloaderConfig")
            .field("directory", &self.directory)
            .field("concurrent_downloads", &self.concurrent_downloads)
            .field("max_chunks_per_file", &self.max_chunks_per_file)
            .field("max_concurrent_chunks", &self.max_concurrent_chunks)
            .field("chunk_threshold", &self.chunk_threshold)
            .field("retries", &self.retries)
            .field("resumable", &self.resumable)
            .field("overwrite", &self.overwrite)
            .field("on_complete", &self.on_complete.is_some())
            .finish()
    }
}
