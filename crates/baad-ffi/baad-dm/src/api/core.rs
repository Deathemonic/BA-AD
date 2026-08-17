use std::path::PathBuf;

use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;

#[derive(Clone)]
pub struct DownloadOptions {
    pub directory: PathBuf,
    pub concurrent_downloads: usize,
    pub max_chunks_per_file: usize,
    pub max_concurrent_chunks: usize,
    pub chunk_threshold: u64,
    pub retries: u32,
    pub resumable: bool,
    pub overwrite: bool,
    pub http1_only: bool,
    pub proxy: Option<reqwest::Proxy>
}

pub fn parse_url(url: &str) -> Result<Url, baad_dm::Error> {
    Url::parse(url).map_err(|error| baad_dm::Error::InvalidUrl {
        url: url.into(),
        reason: error.to_string().into()
    })
}

pub fn create_proxy(proxy_url: Option<&str>) -> Result<Option<reqwest::Proxy>, baad_dm::Error> {
    proxy_url
        .map(|url| {
            reqwest::Proxy::all(url).map_err(|error| baad_dm::Error::InvalidUrl {
                url: url.into(),
                reason: error.to_string().into()
            })
        })
        .transpose()
}

pub fn default_client() -> Result<ClientWithMiddleware, baad_dm::Error> {
    baad_dm::create_http_client(baad_dm::HttpClientConfig::builder().build()).map_err(Into::into)
}

pub async fn run_download(
    options: DownloadOptions,
    items: Vec<baad_dm::Download>
) -> Vec<baad_dm::Summary> {
    let config = baad_dm::DownloaderConfig::builder()
        .directory(options.directory.as_path())
        .concurrent_downloads(options.concurrent_downloads)
        .max_chunks_per_file(options.max_chunks_per_file)
        .max_concurrent_chunks(options.max_concurrent_chunks)
        .chunk_threshold(options.chunk_threshold)
        .retries(options.retries)
        .resumable(options.resumable)
        .overwrite(options.overwrite)
        .http1_only(options.http1_only)
        .maybe_proxy(options.proxy)
        .build();

    baad_dm::Downloader::new(config).download(&items).await
}
