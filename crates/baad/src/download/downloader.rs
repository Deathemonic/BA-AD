use crate::download::ResourceFilter;
use crate::helpers::{
    DownloadError, GameFilesBundles, HashValue,
    ServerConfig, ServerRegion,
};
use crate::utils::{catalog::load_resources, network};

use baad_core::{debug, error, file, info, warn};
use reqwest::Url;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use trauma::download::{Download, Status};
use trauma::downloader::{Downloader, DownloaderBuilder};
use trauma::progress::{ProgressBarOpts, StyleOptions};

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Assets,
    Tables,
    Media,
    All,
    Multiple(Vec<ResourceCategory>),
}

impl ResourceCategory {
    pub fn multiple(categories: Vec<ResourceCategory>) -> Self {
        ResourceCategory::Multiple(categories)
    }
}

pub struct ResourceDownloader {
    downloader: Downloader,
    config: Rc<ServerConfig>,
    proxy: Option<String>,
}

pub struct ResourceDownloadBuilder {
    output: Option<PathBuf>,
    retries: u32,
    limit: u64,
    config: Rc<ServerConfig>,
    proxy: Option<String>,
}

impl ResourceDownloader {
    pub async fn new(
        output: Option<PathBuf>,
        config: Rc<ServerConfig>,
    ) -> Result<Self, DownloadError> {
        ResourceDownloadBuilder::new(config)?
            .output(output)
            .build()
            .await
    }

    fn matches_filter(
        filter: &ResourceFilter,
        path: &str,
        bundle_files: Option<&[String]>,
    ) -> bool {
        let filename = Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(path);

        if filter.matches(filename) {
            return true;
        }

        if let Some(bundles) = bundle_files {
            return bundles
                .iter()
                .any(|bundle_name| filter.matches(bundle_name));
        }

        false
    }

    pub async fn download(
        &self,
        category: ResourceCategory,
        filter: Option<ResourceFilter>,
    ) -> Result<(), DownloadError> {
        let path = match self.config.region {
            ServerRegion::Global => file::get_data_path("catalog/global/GameFiles.json")?,
            ServerRegion::Japan => file::get_data_path("catalog/japan/GameFiles.json")?,
        };

        let resources = load_resources(&self.config.region, &path).await?;
        let downloads = resources.get_downloads(&category, filter.as_ref());
        self.execute_downloads(downloads, category).await
    }

    pub(crate) fn process_files<T>(
        files: &[T],
        filter: Option<&ResourceFilter>,
        get_url: impl Fn(&T) -> &str,
        get_path: impl Fn(&T) -> &str,
        get_hash: impl Fn(&T) -> &HashValue,
        get_bundles: impl Fn(&T) -> Option<&[String]>,
    ) -> Vec<Download> {
        files
            .iter()
            .filter(|file| {
                filter.is_none_or(|f| Self::matches_filter(f, get_path(file), get_bundles(file)))
            })
            .filter_map(|file| {
                Self::create_download(get_url(file), get_path(file), get_hash(file), None)
            })
            .collect()
    }

    fn convert_path_to_bundle(zip_path: &str, bundle_filename: &str) -> String {
        if let Some(last_slash) = zip_path.rfind('/') {
            format!("{}/{}", &zip_path[..last_slash], bundle_filename)
        } else {
            bundle_filename.to_string()
        }
    }

    pub(crate) fn process_japan_assets(
        files: &[GameFilesBundles],
        filter: Option<&ResourceFilter>,
    ) -> Vec<Download> {
        let Some(f) = filter else {
            return files
                .iter()
                .filter_map(|file| Self::create_download(&file.url, &file.path, &file.hash, None))
                .collect();
        };

        let mut downloads = Vec::new();

        for file in files {
            let filename = Path::new(&file.path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&file.path);

            if f.matches(filename) {
                Self::add_download(
                    &mut downloads,
                    Self::create_download(&file.url, &file.path, &file.hash, None),
                );
            }

            for bundle_name in &file.bundle_files {
                if f.matches(bundle_name) {
                    Self::add_download(
                        &mut downloads,
                        Self::create_download(&file.url, &file.path, &file.hash, Some(bundle_name)),
                    );
                }
            }
        }

        downloads
    }

    fn add_download(downloads: &mut Vec<Download>, download: Option<Download>) {
        if let Some(download) = download {
            downloads.push(download);
        }
    }

    fn create_download(
        url: &str,
        path: &str,
        hash: &HashValue,
        target: Option<&str>,
    ) -> Option<Download> {
        let parsed_url = Url::parse(url).ok()?;
        debug!(?parsed_url, "URL");

        let final_path = if let Some(bundle_filename) = target {
            Self::convert_path_to_bundle(path, bundle_filename)
        } else {
            path.to_string()
        };
        debug!(?final_path, "Path");

        let target_filename = Path::new(&final_path).file_name()?.to_str()?.to_string();
        debug!(?target_filename, "Target");

        Some(Download {
            url: parsed_url,
            filename: final_path,
            target_file: target.map(|_| target_filename),
            hash: Some(match hash {
                HashValue::Crc(crc) => crc.to_string(),
                HashValue::Md5(md5) => md5.clone(),
            }),
        })
    }

    async fn execute_downloads(
        &self,
        downloads: Vec<Download>,
        category: ResourceCategory,
    ) -> Result<(), DownloadError> {
        if downloads.is_empty() {
            warn!(
                ?category,
                "No files matched the filter criteria for catalog"
            );
            return Err(DownloadError::NoFilesMatched);
        }

        info!(?category, "Found {} files for download", downloads.len());
        info!("Downloading...");

        let proxy = network::create_proxy(self.proxy.as_deref()).map_err(DownloadError::Network)?;
        self.downloader.download(&downloads, proxy).await;

        Ok(())
    }
}

impl ResourceDownloadBuilder {
    pub fn new(config: Rc<ServerConfig>) -> Result<Self, DownloadError> {
        Ok(Self {
            output: None,
            retries: 10,
            limit: 10,
            config,
            proxy: None,
        })
    }

    pub fn output(mut self, output: Option<PathBuf>) -> Self {
        self.output = output;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = limit;
        self
    }

    pub fn proxy(mut self, proxy: Option<String>) -> Self {
        self.proxy = proxy;
        self
    }

    pub async fn build(self) -> Result<ResourceDownloader, DownloadError> {
        if self.retries == 0 {
            return Err(DownloadError::RetryCountZero);
        }

        if self.limit == 0 {
            return Err(DownloadError::DownloadLimitZero);
        }

        let style = StyleOptions::new(ProgressBarOpts::hidden(), ProgressBarOpts::hidden());

        let downloader = DownloaderBuilder::new()
            .directory(file::get_output_dir(self.output).await?)
            .concurrent_downloads(self.limit as usize)
            .retries(self.retries)
            .style_options(style)
            .on_complete({
                move |summary| {
                    let filename = Path::new(&summary.download().filename)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or(&summary.download().filename);
                    match summary.status() {
                        Status::Success => {
                            info!(success = true, filename = filename, "Downloaded");
                        }
                        Status::Fail(error) => {
                            error!(cause = error, filename = filename, "Failed to download");
                        }
                        Status::Skipped(reason) => {
                            warn!(cause = reason, filename = filename, "Skipped");
                        }
                        Status::HashMismatch(reason) => {
                            warn!(cause = reason, filename = filename, "Outdated");
                        }
                        Status::NotStarted => {
                            // Ignore
                        }
                    }
                }
            })
            .build();

        Ok(ResourceDownloader {
            downloader,
            config: self.config,
            proxy: self.proxy,
        })
    }
}
