use std::path::{Path, PathBuf};

use baad_dm::{Download, Downloader, DownloaderConfig};
use baad_shared::{DownloadStatus, Downloads};
use bon::Builder;
use reqwest::{Proxy, Url};
use tracing::{error, info, warn};

use crate::download::{ResourceFilter, converter};
use crate::error::CatalogError;

#[derive(Builder)]
pub struct ResourceDownloader {
    #[builder(default = PathBuf::from("./output"))]
    output_dir: PathBuf,

    #[builder(default = 10)]
    limit: usize,

    #[builder(default = 10)]
    retries: u32,

    proxy: Option<String>,

    #[builder(default = false)]
    http1_only: bool,

    #[builder(default = 16)]
    max_chunks_per_file: usize,

    #[builder(default = 8)]
    max_concurrent_chunks: usize,

    #[builder(default = 10 * 1024 * 1024)]
    chunk_threshold: u64
}

impl ResourceDownloader {
    pub async fn download(
        &self,
        downloads: Downloads,
        filter: Option<&ResourceFilter>
    ) -> Result<(), CatalogError> {
        if !downloads.assets.is_empty() {
            let convert = converter::convert_assets(&downloads.assets, filter);
            self.execute(convert, "Assets").await?;
        }
        if !downloads.tables.is_empty() {
            let convert = converter::convert_tables(&downloads.tables, filter);
            self.execute(convert, "Tables").await?;
        }
        if !downloads.media.is_empty() {
            let convert = converter::convert_media(&downloads.media, filter);
            self.execute(convert, "Media").await?;
        }
        Ok(())
    }

    async fn execute(&self, downloads: Vec<Download>, category: &str) -> Result<(), CatalogError> {
        if downloads.is_empty() {
            warn!(category = category, "No files matched filter");
            return Ok(());
        }

        info!(category = category, "Starting download");

        let proxy = self.proxy.as_deref().and_then(|url| Proxy::all(url).ok());

        let config = DownloaderConfig::builder()
            .directory(self.output_dir.as_path())
            .concurrent_downloads(self.limit)
            .retries(self.retries)
            .maybe_proxy(proxy)
            .http1_only(self.http1_only)
            .max_chunks_per_file(self.max_chunks_per_file)
            .max_concurrent_chunks(self.max_concurrent_chunks)
            .chunk_threshold(self.chunk_threshold)
            .build();

        let downloader = Downloader::new(config);
        let summaries = downloader.download(&downloads).await;

        let failed_count =
            summaries.iter().filter(|s| matches!(s.status, DownloadStatus::Failed(_))).count();

        if failed_count > 0 {
            error!(category = category, failed = failed_count, "Some downloads failed");
        }

        info!(category = category, success = true, "Download complete");
        Ok(())
    }
}

pub async fn download_file(
    url: &str,
    output_path: &Path,
    hash: Option<String>,
    retries: u32
) -> Result<(), CatalogError> {
    let parsed_url = Url::parse(url).map_err(|_| CatalogError::DeserializationFailed)?;
    let filename = output_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(CatalogError::DeserializationFailed)?;
    let output_dir = output_path.parent().ok_or(CatalogError::DeserializationFailed)?;

    let overwrite = hash.is_none();

    let download =
        Download::builder().url(parsed_url).filename(filename.into()).maybe_hash(hash).build();

    let config = DownloaderConfig::builder()
        .directory(output_dir)
        .concurrent_downloads(1)
        .retries(retries)
        .overwrite(overwrite)
        .build();

    let summaries = Downloader::new(config).download(&[download]).await;

    if let Some(summary) = summaries.first()
        && matches!(summary.status, DownloadStatus::Failed(_))
    {
        return Err(CatalogError::DeserializationFailed);
    }

    Ok(())
}
