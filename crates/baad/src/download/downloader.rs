use std::path::{Path, PathBuf};

use baad_core::{CatalogError, DownloadStatus, Downloads};
use baad_dm::{Download, Downloader, DownloaderConfig};
use baad_utils::{error, info, warn};
use bon::Builder;
use reqwest::{Proxy, Url};

use crate::download::{ResourceFilter, converter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceCategory {
    Assets,
    Tables,
    Media
}

#[derive(Builder)]
pub struct ResourceDownloader {
    output_dir: PathBuf,
    limit: usize,
    retries: u32,
    proxy: Option<String>
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
            .build();

        let downloader = Downloader::new(config);
        let summaries = downloader.download(&downloads).await;

        let failed_count =
            summaries.iter().filter(|s| matches!(s.status, DownloadStatus::Failed(_))).count();

        if failed_count > 0 {
            error!(category = category, failed = failed_count, "Some downloads failed");
        }

        info!(category = category, "Download complete");
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

    let download =
        Download::builder().url(parsed_url).filename(filename.to_string()).maybe_hash(hash).build();

    let config = DownloaderConfig::builder()
        .directory(output_dir)
        .concurrent_downloads(1)
        .retries(retries)
        .build();

    let summaries = Downloader::new(config).download(&[download]).await;

    if let Some(summary) = summaries.first()
        && matches!(summary.status, DownloadStatus::Failed(_))
    {
        return Err(CatalogError::DeserializationFailed);
    }

    Ok(())
}
