use crate::helpers::{
    apk_headers, ApiData, ApkError, NetworkError, ServerConfig, ServerRegion, JAPAN_REGEX_URL,
    REGEX_VERSION,
};
use crate::utils::{json, network};

use baad_core::{debug, file, info, warn};
use reqwest::{Client, Url};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tokio::fs;
use trauma::download::Download;
use trauma::downloader::{Downloader, DownloaderBuilder};

#[derive(Clone)]
pub struct ApkFetcher {
    client: Client,
    config: Rc<ServerConfig>,
    downloader: Downloader,
    proxy: Option<String>,
}

impl ApkFetcher {
    pub fn new(config: Rc<ServerConfig>) -> Result<Self, ApkError> {
        Self::with_proxy(config, None)
    }

    pub fn with_proxy(config: Rc<ServerConfig>, proxy: Option<String>) -> Result<Self, ApkError> {
        let mut client_builder = Client::builder().default_headers(apk_headers());

        if let Some(ref proxy_url) = proxy {
            let proxy = network::create_proxy(Some(proxy_url))
                .map_err(ApkError::Network)?
                .ok_or(NetworkError::Proxy)?;
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build()?;

        let downloader = DownloaderBuilder::new()
            .directory(file::data_dir()?.into())
            .headers(apk_headers())
            .use_range_for_content_length(true)
            .single_file_progress(true)
            .overwrite(true)
            .build();

        Ok(Self {
            client,
            config,
            downloader,
            proxy,
        })
    }

    fn extract_url(&self, body: &str) -> Result<String, ApkError> {
        match JAPAN_REGEX_URL.captures(body) {
            Some(caps) if caps.len() >= 3 => caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .ok_or(ApkError::DownloadUrlExtractionFailed),
            _ => Err(ApkError::DownloadUrlExtractionFailed),
        }
    }

    pub async fn get_version(&self) -> Result<String, ApkError> {
        let response = self.client.get(&*self.config.version_url).send().await?;
        let body = response.text().await?;
        REGEX_VERSION
            .find(&body)
            .map(|m| m.as_str().to_string())
            .ok_or(ApkError::VersionExtractionFailed)
    }

    pub async fn check_version(&self) -> Result<Option<String>, ApkError> {
        let version = self.get_version().await?;
        let platform = self.config.platform.as_str();
        let build_type = self.config.build_type.as_str();

        json::update_api_data(|data| match &self.config.region {
            ServerRegion::Global => {
                data.global.version = version.clone();
                data.global.platform = platform.into();
                data.global.build_type = build_type.into();
            }
            ServerRegion::Japan => {
                data.japan.version = version.clone();
                data.japan.platform = platform.into();
            }
        })
        .await?;

        Ok(Some(version))
    }

    async fn check_apk(&self, download_url: &str, apk_path: &Path) -> Result<bool, ApkError> {
        if !apk_path.exists() {
            return Ok(true);
        }

        let local_size = match fs::metadata(apk_path).await {
            Ok(metadata) => metadata.len(),
            Err(_) => return Ok(true),
        };

        let response = self
            .client
            .get(download_url)
            .header("Range", "bytes=0-0")
            .send()
            .await?;

        if !response.status().is_success()
            && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
        {
            return Err(ApkError::ApkInfoFailed);
        }

        let remote_size = network::get_content_length(&response);
        if remote_size == 0 || local_size != remote_size {
            debug!("APK is outdated or incomplete");
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn needs_catalog_update(&self) -> Result<bool, ApkError> {
        let api_data_path = file::get_data_path("api_data.json")?;
        if !api_data_path.exists() {
            return Ok(true);
        }

        let current_version = self.get_version().await?;
        let api_data: ApiData = json::load_json(&api_data_path).await?;

        let (cached_version, cached_platform, cached_build_type) = match &self.config.region {
            ServerRegion::Global => (
                &api_data.global.version,
                &api_data.global.platform,
                &api_data.global.build_type,
            ),
            ServerRegion::Japan => (
                &api_data.japan.version,
                &api_data.japan.platform,
                &"Standard".into(),
            ),
        };

        let current_platform = self.config.platform.as_str();
        let current_build_type = self.config.build_type.as_str();

        if current_version != *cached_version {
            info!("Version has changed, updating catalogs...");
            Ok(true)
        } else if current_platform != *cached_platform {
            info!("Platform has changed, updating catalogs...");
            debug!(current_platform, "Using platform");
            Ok(true)
        } else if current_build_type != *cached_build_type {
            info!("Build type has changed, updating catalogs...");
            debug!(current_build_type, "Using build");
            Ok(true)
        } else {
            info!("Version type are up to date");
            Ok(false)
        }
    }

    pub async fn needs_update(
        &self,
        download_url: &str,
        apk_path: &Path,
    ) -> Result<bool, ApkError> {
        let needs_download = self.check_apk(download_url, apk_path).await?;

        if !needs_download {
            info!("APK is up to date, skipping download");
            return Ok(false);
        }

        if !apk_path.exists() {
            warn!("APK doesn't exist, downloading...");
        } else {
            warn!("APK is outdated, downloading...");
        }

        Ok(true)
    }

    pub async fn download_apk(&self, force: bool) -> Result<(String, PathBuf, bool), ApkError> {
        let new_version = self.get_version().await?;
        debug!(new_version, "Using version");

        let apk_path = file::get_data_path(&self.config.apk_path)?;

        let response = self.client.get(&*self.config.version_url).send().await?;
        let body = response.text().await?;
        let download_url = self.extract_url(&body)?;

        debug!(download_url, "Download URL");

        if !force && !self.needs_update(&download_url, &apk_path).await? {
            return Ok((new_version, apk_path, false));
        }

        if apk_path.exists() {
            fs::remove_file(&apk_path).await?;
            info!(success = true, "Removed old APK");
        }

        info!("Downloading APK...");
        let apk = vec![Download {
            url: Url::parse(download_url.as_str())?,
            filename: self.config.apk_path.to_string(),
            target_file: None,
            hash: None,
        }];

        let proxy = network::create_proxy(self.proxy.as_deref()).map_err(ApkError::Network)?;
        self.downloader.download(&apk, proxy).await;

        info!(success = true, "APK downloaded");

        Ok((new_version, apk_path, true))
    }
}
