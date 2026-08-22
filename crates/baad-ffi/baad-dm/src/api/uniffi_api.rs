//! `UniFFI` bindings for `baad-dm`. Borrowed lifetimes become owning objects,
//! `bon` builders become records, `reqwest` types become strings; generated
//! mirrors live in `shadow.rs`.

//! `UniFFI` arguments cross the boundary by value, so exported functions cannot
//! take records or optional strings by reference.
#![allow(clippy::needless_pass_by_value)]

use std::path::PathBuf;

use baad_dm::client::{create_range_header as create_range_header_native, resolve_url as resolve};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;

use super::core;

fn parse_url(url: &str) -> Result<Url, Error> { core::parse_url(url).map_err(Into::into) }

#[uniffi::export]
pub fn detect_hash_type(hash: &str) -> Option<baad_dm::HashType> { baad_dm::detect_hash_type(hash) }

#[uniffi::export]
pub fn verify_hash(file_path: &str, expected: Option<String>) -> Result<bool, Error> {
    baad_dm::verify_hash(PathBuf::from(file_path).as_path(), expected.as_ref()).map_err(Into::into)
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct Download {
    pub url: String,
    pub filename: String,
    #[uniffi(default = None)]
    pub hash: Option<String>,
    #[uniffi(default = None)]
    pub target_file: Option<String>,
    #[uniffi(default = None)]
    pub size: Option<u64>
}

impl TryFrom<Download> for baad_dm::Download {
    type Error = Error;

    fn try_from(download: Download) -> Result<Self, Error> {
        Ok(Self::builder()
            .url(parse_url(&download.url)?)
            .filename(download.filename)
            .maybe_hash(download.hash)
            .maybe_target_file(download.target_file)
            .maybe_size(download.size)
            .build())
    }
}

impl From<&baad_dm::Download> for Download {
    fn from(download: &baad_dm::Download) -> Self {
        Self {
            url: String::from(download.url.as_str()),
            filename: download.filename.clone(),
            hash: download.hash.clone(),
            target_file: download.target_file.clone(),
            size: download.size
        }
    }
}

#[uniffi::export]
pub fn download_from_url(url: &str) -> Result<Download, Error> {
    let download = baad_dm::Download::try_from(url).map_err(Error::from)?;
    Ok(Download::from(&download))
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct Summary {
    pub download: Download,
    pub status_code: u16,
    pub size: u64,
    pub status: DownloadStatus,
    pub resumable: bool
}

impl From<&baad_dm::Summary> for Summary {
    fn from(summary: &baad_dm::Summary) -> Self {
        Self {
            download: Download::from(summary.download.as_ref()),
            status_code: summary.status_code.as_u16(),
            size: summary.size,
            status: DownloadStatus::from(summary.status.clone()),
            resumable: summary.resumable
        }
    }
}

#[uniffi::export]
pub const fn summary_is_success(summary: &Summary) -> bool {
    matches!(summary.status, DownloadStatus::Success)
}

#[derive(uniffi::Object)]
pub struct Downloader {
    options: core::DownloadOptions
}

#[uniffi::export]
impl Downloader {
    #[uniffi::constructor]
    pub fn new(config: DownloaderConfig) -> Result<Self, Error> {
        Ok(Self {
            options: core::DownloadOptions {
                directory: PathBuf::from(&config.directory),
                concurrent_downloads: config.concurrent_downloads as usize,
                max_chunks_per_file: config.max_chunks_per_file as usize,
                max_concurrent_chunks: config.max_concurrent_chunks as usize,
                chunk_threshold: config.chunk_threshold,
                retries: config.retries,
                resumable: config.resumable,
                overwrite: config.overwrite,
                http1_only: config.http1_only,
                proxy: core::create_proxy(config.proxy.as_deref())?
            }
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn download(&self, downloads: Vec<Download>) -> Result<Vec<Summary>, Error> {
        let items = downloads
            .into_iter()
            .map(baad_dm::Download::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let options = self.options.clone();

        let summaries =
            run_blocking(move |handle| handle.block_on(core::run_download(options, items))).await;
        Ok(summaries.iter().map(Summary::from).collect())
    }
}

#[derive(uniffi::Object)]
pub struct ZipExtractor {
    client: ClientWithMiddleware,
    url: Url
}

#[uniffi::export]
impl ZipExtractor {
    #[uniffi::constructor]
    pub fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: core::default_client().map_err(Error::from)?,
            url: parse_url(url)?
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn extract_file(&self, target: &str) -> Result<Vec<u8>, Error> {
        let extractor = baad_dm::ZipExtractor::new(&self.client, &self.url).await?;
        extractor.extract_file(target).await.map(|b| b.to_vec()).map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn list_files(&self) -> Result<Vec<String>, Error> {
        let extractor = baad_dm::ZipExtractor::new(&self.client, &self.url).await?;
        let index = extractor.build_index().await?;
        Ok(index.names().map(String::from).collect())
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn file_info(&self, target: &str) -> Result<Option<baad_dm::ZipFileInfo>, Error> {
        let extractor = baad_dm::ZipExtractor::new(&self.client, &self.url).await?;
        let index = extractor.build_index().await?;
        Ok(index.get(target).cloned())
    }
}

#[derive(uniffi::Object)]
pub struct ZipCache {
    client: ClientWithMiddleware,
    inner: baad_dm::ZipCache
}

#[uniffi::export]
impl ZipCache {
    #[uniffi::constructor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            client: core::default_client().map_err(Error::from)?,
            inner: baad_dm::ZipCache::new()
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn file_info(
        &self,
        url: &str,
        target: &str
    ) -> Result<Option<baad_dm::ZipFileInfo>, Error> {
        let parsed = parse_url(url)?;
        let index = self.inner.get_index(&self.client, &parsed).await?;
        Ok(index.get(target).cloned())
    }
}

#[uniffi::export]
pub fn create_range_header(start: u64, end: Option<u64>) -> String {
    create_range_header_native(start, end)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn resolve_url(url: &str) -> Result<String, Error> {
    let client = core::default_client().map_err(Error::from)?;
    resolve(&client, url).await.map_err(Into::into)
}

include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
