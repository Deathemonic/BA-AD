use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use baad_dm::{Download, Downloader, DownloaderConfig, HashType, Summary};
use baad_shared::{DownloadStatus, Downloads};
use bon::Builder;
use reqwest::{Proxy, Url};
use tracing::{error, info, warn};

use crate::download::ResourceFilter;
use crate::download::manifest::{self, ManifestEntry, Report, ResourceKind};
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
        let report = self.download_with_report(&downloads, filter).await?;
        let failures = report.failures().map(ToString::to_string).collect::<Vec<_>>();
        if !failures.is_empty() {
            return Err(baad_dm::Error::DownloadFailed(failures.join("\n").into()).into());
        }
        Ok(())
    }

    /// Like [`Self::download`], but returns each file's outcome instead of
    /// failing when some files could not be downloaded.
    pub async fn download_with_report(
        &self,
        downloads: &Downloads,
        filter: Option<&ResourceFilter>
    ) -> Result<Report, CatalogError> {
        let mut entries = manifest::plan(downloads, filter);
        entries.retain(|entry| match entry.to_download() {
            Ok(_) => true,
            Err(error) => {
                warn!(path = entry.path, %error, "Skipping resource");
                false
            }
        });
        for (kind, available) in [
            (ResourceKind::Assets, downloads.assets.len()),
            (ResourceKind::Tables, downloads.tables.len()),
            (ResourceKind::Media, downloads.media.len())
        ] {
            if available > 0 && !entries.iter().any(|entry| entry.kind == kind) {
                warn!(category = kind.label(), "No files matched filter");
            }
        }
        self.download_entries(&entries).await
    }

    /// Downloads every entry and reports each file's outcome. Fails before
    /// downloading anything when an entry is invalid; failed files are only
    /// recorded in the report.
    pub async fn download_entries(
        &self,
        entries: &[ManifestEntry]
    ) -> Result<Report, CatalogError> {
        let mut seen = HashSet::new();
        let mut batches: Vec<(ResourceKind, Vec<Download>)> = Vec::new();
        for entry in entries {
            let download = entry.to_download()?;
            if !seen.insert(entry.path.as_str()) {
                continue;
            }
            match batches.iter_mut().find(|(kind, _)| *kind == entry.kind) {
                Some((_, batch)) => batch.push(download),
                None => batches.push((entry.kind, vec![download]))
            }
        }

        let mut report = Report::default();
        for (kind, batch) in batches {
            let mut summaries = self.execute(&batch, kind.label()).await;
            // Downloads finish in any order; report them in manifest order.
            let order: HashMap<&str, usize> = batch
                .iter()
                .enumerate()
                .map(|(index, download)| (download.filename.as_str(), index))
                .collect();
            summaries.sort_by_key(|summary| order.get(summary.download.filename.as_str()).copied());
            report.extend(kind, &summaries);
        }
        Ok(report)
    }

    async fn execute(&self, downloads: &[Download], category: &str) -> Vec<Summary> {
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
        let summaries = downloader.download(downloads).await;

        let failed_count =
            summaries.iter().filter(|s| matches!(s.status, DownloadStatus::Failed(_))).count();

        if failed_count > 0 {
            error!(category = category, failed = failed_count, "Some downloads failed");
        } else {
            info!(category = category, success = true, "Download complete");
        }
        summaries
    }
}

fn failed_downloads(summaries: &[baad_dm::Summary]) -> CatalogError {
    let failures = summaries
        .iter()
        .filter_map(|summary| match &summary.status {
            DownloadStatus::Failed(error) => {
                Some(format!("{}: {error}", summary.download.filename))
            }
            _ => None
        })
        .collect::<Vec<_>>();
    baad_dm::Error::DownloadFailed(failures.join("\n").into()).into()
}

/// Expected properties of a single file, as published by its manifest.
#[derive(Debug, Clone, Default)]
pub struct ExpectedFile {
    /// Content checksum. Opaque version identifiers must not be used here.
    pub hash: Option<String>,
    /// Algorithm for `hash`; when omitted only MD5 and CRC32 are inferred.
    pub hash_type: Option<HashType>,
    pub size: Option<u64>
}

pub async fn download_file(
    url: &str,
    output_path: &Path,
    hash: Option<String>,
    retries: u32
) -> Result<(), CatalogError> {
    let expected = ExpectedFile {
        hash,
        ..ExpectedFile::default()
    };
    download_expected_file(url, output_path, &expected, retries).await
}

/// Downloads one file, reusing an existing copy only when its size or
/// checksum can be checked against `expected`.
pub async fn download_expected_file(
    url: &str,
    output_path: &Path,
    expected: &ExpectedFile,
    retries: u32
) -> Result<(), CatalogError> {
    let parsed_url = Url::parse(url).map_err(|_| CatalogError::DeserializationFailed)?;
    let filename = output_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(CatalogError::DeserializationFailed)?;
    let output_dir = output_path.parent().ok_or(CatalogError::DeserializationFailed)?;

    let overwrite = expected.hash.is_none() && expected.size.is_none();

    let download = Download::builder()
        .url(parsed_url)
        .filename(filename.into())
        .maybe_hash(expected.hash.clone())
        .maybe_hash_type(expected.hash_type)
        .maybe_size(expected.size)
        .build();

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
        return Err(failed_downloads(&summaries));
    }

    Ok(())
}
