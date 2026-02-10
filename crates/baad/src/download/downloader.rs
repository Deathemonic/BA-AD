use std::path::{Path, PathBuf};
use std::sync::Arc;

use baad_core::{CatalogError, DownloadObserver, NoopObserver};
use baad_dm::{Download, Downloader, DownloaderConfig, Status};
use baad_utils::{error, info, warn};
use reqwest::Url;

use crate::catalog::Downloads;
use crate::download::{ResourceFilter, converter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceCategory {
    Assets,
    Tables,
    Media
}

pub struct ResourceDownloader {
    output_dir: PathBuf,
    limit: usize,
    retries: u32,
    proxy: Option<String>,
    observer: Arc<dyn DownloadObserver>
}

impl ResourceDownloader {
    pub fn new(output_dir: PathBuf, limit: usize, retries: u32) -> Self {
        Self {
            output_dir,
            limit,
            retries,
            proxy: None,
            observer: Arc::new(NoopObserver)
        }
    }

    pub fn with_proxy(mut self, proxy: Option<String>) -> Self {
        self.proxy = proxy;
        self
    }

    pub fn with_observer(mut self, observer: Arc<dyn DownloadObserver>) -> Self {
        self.observer = observer;
        self
    }

    pub async fn download(
        &self,
        downloads: Downloads,
        categories: &[ResourceCategory],
        filter: Option<&ResourceFilter>
    ) -> Result<(), CatalogError> {
        for category in categories {
            match category {
                ResourceCategory::Assets => {
                    let dm_downloads = converter::convert_assets(&downloads.assets, filter);
                    self.execute(dm_downloads, "Assets").await?;
                }
                ResourceCategory::Tables => {
                    let dm_downloads = converter::convert_tables(&downloads.tables, filter);
                    self.execute(dm_downloads, "Tables").await?;
                }
                ResourceCategory::Media => {
                    let dm_downloads = converter::convert_media(&downloads.media, filter);
                    self.execute(dm_downloads, "Media").await?;
                }
            }
        }
        Ok(())
    }

    async fn execute(&self, downloads: Vec<Download>, category: &str) -> Result<(), CatalogError> {
        if downloads.is_empty() {
            warn!(category = category, "No files matched filter");
            return Ok(());
        }

        info!(category = category, count = downloads.len(), "Starting download");

        let mut config = DownloaderConfig::builder()
            .directory(self.output_dir.clone())
            .concurrent_downloads(self.limit)
            .retries(self.retries)
            .observer(Arc::clone(&self.observer))
            .build();

        if let Some(ref proxy_url) = self.proxy
            && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
        {
            config.proxy = Some(proxy);
        }

        let downloader = Downloader::new(config);
        let summaries = downloader.download(&downloads).await;

        let failed_count =
            summaries.iter().filter(|s| matches!(s.status, Status::Failed(_))).count();

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

    let mut download = Download::new(parsed_url, filename);

    if let Some(h) = hash {
        download = download.with_hash(h);
    }

    let config = DownloaderConfig::builder()
        .directory(output_dir.to_path_buf())
        .concurrent_downloads(1)
        .retries(retries)
        .build();

    let downloader = Downloader::new(config);
    let summaries = downloader.download(&[download]).await;

    if let Some(summary) = summaries.first()
        && matches!(summary.status, Status::Failed(_))
    {
        return Err(CatalogError::DeserializationFailed);
    }

    Ok(())
}
