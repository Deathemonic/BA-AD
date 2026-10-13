use std::fmt::Write;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use bacy::hash::sha;
use fastcat::fconcat;
use reqwest_middleware::reqwest::Url;
use tokio::fs;
use tokio::task::spawn_blocking;
use tokio::time::sleep;
use tracing::warn;

use crate::download::summary::FetchOutcome;
use crate::download::{Download, DownloadSource, ServerSegment, detect_hash_type};
use crate::downloader::assembly::OutputJob;
use crate::downloader::config::DownloaderConfig;
use crate::downloader::helpers::{Cancellation, SharedExecution, ensure_parent_dir};
use crate::downloader::progress::ProgressTracker;
use crate::downloader::segment::{TransferContext, TransferRequest};
use crate::error::Error;
use crate::zip::ZipSource;

const RETRY_BASE_DELAY_MS: u64 = 250;

pub struct DownloadJob<'job, 'config> {
    pub shared: &'job SharedExecution,
    pub config: &'job DownloaderConfig<'config>,
    pub download: &'job Download,
    pub destination: PathBuf,
    pub cancellation: Arc<Cancellation>
}

impl DownloadJob<'_, '_> {
    pub async fn run(&self, filename: Arc<str>, total: u64) -> Result<FetchOutcome, Error> {
        let identity = source_identity(self.download);
        let resource_lock = self.shared.resource_lock(&identity);
        let _resource_guard = resource_lock.lock().await;

        if !self.config.overwrite && self.output_matches().await? {
            let size = fs::metadata(&self.destination).await?.len();
            return Ok(FetchOutcome::skipped("File exists with matching metadata", size));
        }

        if self.download.is_extraction() {
            return self.extract_zip().await;
        }

        let progress = Arc::new(ProgressTracker::new(
            filename,
            total,
            progress_limits(self.download),
            Arc::clone(&self.config.observer)
        ));

        let cache_dir = self.config.directory.join(".baad-dm-cache").join(identity);
        let transfer = TransferContext::new(
            self.shared,
            self.config,
            progress,
            Arc::clone(&self.cancellation)
        );

        let paths = self.fetch_source(&transfer, &cache_dir).await?;
        let size = self.process(paths).await?;

        clean_cache(&cache_dir).await;

        Ok(FetchOutcome::success(size, self.config.resumable))
    }

    async fn fetch_source(
        &self,
        transfer: &TransferContext<'_>,
        cache_dir: &Path
    ) -> Result<Vec<PathBuf>, Error> {
        match &self.download.source {
            DownloadSource::Url(url) => {
                fs::create_dir_all(cache_dir).await?;
                let passthrough = self.download.transform.is_none();
                let path = cache_dir.join("payload");
                let request = TransferRequest {
                    url,
                    path: &path,
                    expected_size: if passthrough { self.download.size } else { None },
                    expected_hash: if passthrough { self.download.hash.as_ref() } else { None },
                    progress_index: 0
                };

                transfer.fetch(&request).await?;
                Ok(vec![path])
            }
            DownloadSource::Segments(segments) => transfer.fetch_segments(segments, cache_dir).await
        }
    }

    async fn process(&self, paths: Vec<PathBuf>) -> Result<u64, Error> {
        ensure_parent_dir(&self.destination).await?;
        let passthrough = matches!(self.download.source, DownloadSource::Url(_))
            && self.download.transform.is_none()
            && !self.download.is_extraction();

        let permit = self
            .shared
            .transforms
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| Error::LimiterClosed)?;

        let job = OutputJob {
            paths,
            destination: self.destination.clone(),
            transform: self.download.transform.clone(),
            expected_size: self.download.size,
            expected_hash: self.download.hash.clone(),
            cancellation: Arc::clone(&self.cancellation)
        };

        spawn_blocking(move || {
            let _permit = permit;
            match (passthrough, job.paths.as_slice()) {
                (true, [path]) => match job.publish(path) {
                    Err(Error::Io(error)) if error.kind() == ErrorKind::CrossesDevices => job.run(),
                    result => result
                },
                _ => job.run()
            }
        })
        .await?
    }

    async fn output_matches(&self) -> Result<bool, Error> {
        self.shared
            .verify_file(&self.destination, self.download.size, self.download.hash.as_ref())
            .await
    }

    async fn extract_zip(&self) -> Result<FetchOutcome, Error> {
        let target = self.download.target_file.as_ref().ok_or_else(|| {
            Error::InvalidDownload("ZIP extraction requires a target member".into())
        })?;

        let url = self
            .download
            .url()
            .ok_or_else(|| Error::InvalidDownload("ZIP extraction requires a URL source".into()))?;

        let data = self.fetch_zip_member(url, target).await?;
        let cache_dir =
            self.config.directory.join(".baad-dm-cache").join(source_identity(self.download));
        let payload = cache_dir.join("payload");
        ensure_parent_dir(&payload).await?;
        fs::write(&payload, data).await?;
        let size = self.process(vec![payload]).await?;

        clean_cache(&cache_dir).await;

        Ok(FetchOutcome::success(size, false))
    }

    async fn fetch_zip_member(&self, url: &Url, target: &str) -> Result<bytes::Bytes, Error> {
        let mut last_error = None;

        for attempt in 0..=self.config.retries {
            self.cancellation.check()?;

            match self.try_zip_member(url, target).await {
                Ok(data) => return Ok(data),
                Err(error) => last_error = Some(error)
            }

            if attempt < self.config.retries {
                let shift = attempt.min(8);
                sleep(Duration::from_millis(RETRY_BASE_DELAY_MS << shift)).await;
            }
        }

        Err(last_error
            .unwrap_or_else(|| Error::DownloadFailed("ZIP transfer did not start".into())))
    }

    async fn try_zip_member(&self, url: &Url, target: &str) -> Result<bytes::Bytes, Error> {
        let source = ZipSource::new(&self.shared.client, url, Some(&self.shared.requests));
        let index = self.shared.zip_cache.get_index(source).await?;
        let info = index.get(target).ok_or_else(|| {
            Error::Archive(fconcat!("File '", target, "' not found in ZIP").into())
        })?;

        source.extract_member(info).await
    }
}

pub fn network_total(download: &Download) -> Option<u64> {
    match &download.source {
        DownloadSource::Url(_) if download.transform.is_none() => download.size,
        DownloadSource::Url(_) => None,
        DownloadSource::Segments(segments) => {
            segments.iter().try_fold(0_u64, |total, segment| total.checked_add(segment.size?))
        }
    }
}

pub fn validate_download(download: &Download) -> Result<(), Error> {
    validate_hash(download.hash.as_ref())?;
    if download.target_file.is_some() && download.transform.is_some() {
        return Err(Error::InvalidDownload(
            "ZIP member extraction does not support output transforms".into()
        ));
    }
    match &download.source {
        DownloadSource::Url(_) => Ok(()),
        DownloadSource::Segments(segments) => validate_segments(download, segments)
    }
}

fn validate_segments(download: &Download, segments: &[ServerSegment]) -> Result<(), Error> {
    if segments.is_empty() {
        return Err(Error::InvalidDownload("segment list must not be empty".into()));
    }
    if download.target_file.is_some() {
        return Err(Error::InvalidDownload(
            "ZIP member extraction does not support segment sources".into()
        ));
    }
    segments.iter().try_fold(0_u64, |total, segment| {
        validate_hash(segment.hash.as_ref())?;
        segment.size.map_or(Ok(total), |size| {
            total
                .checked_add(size)
                .ok_or_else(|| Error::InvalidDownload("segment transfer size overflow".into()))
        })
    })?;
    Ok(())
}

fn validate_hash(hash: Option<&String>) -> Result<(), Error> {
    if let Some(hash) = hash
        && detect_hash_type(hash).is_none()
    {
        return Err(Error::UnsupportedHash(hash.clone().into()));
    }
    Ok(())
}

fn progress_limits(download: &Download) -> Vec<Option<u64>> {
    match &download.source {
        DownloadSource::Url(_) => {
            vec![if download.transform.is_none() { download.size } else { None }]
        }
        DownloadSource::Segments(segments) => segments.iter().map(|segment| segment.size).collect()
    }
}

fn source_identity(download: &Download) -> String {
    let mut descriptor = String::new();
    match &download.source {
        DownloadSource::Url(url) => {
            append_identity(&mut descriptor, url, download.size, download.hash.as_ref());
        }
        DownloadSource::Segments(segments) => {
            for segment in segments {
                append_identity(&mut descriptor, &segment.url, segment.size, segment.hash.as_ref());
            }
        }
    }
    let hash = sha::compute(descriptor.as_bytes());
    hash.iter().fold(String::with_capacity(hash.len() * 2), |mut identity, byte| {
        let _ = write!(identity, "{byte:02x}");
        identity
    })
}

fn append_identity(descriptor: &mut String, url: &Url, size: Option<u64>, hash: Option<&String>) {
    let mut sanitized = url.clone();
    sanitized.set_query(None);
    sanitized.set_fragment(None);
    descriptor.push_str(sanitized.as_str());
    descriptor.push('|');
    descriptor.push_str(&size.map_or_else(|| "?".into(), |value| value.to_string()));
    descriptor.push('|');
    descriptor.push_str(hash.map_or("?", String::as_str));
    descriptor.push('\n');
}

async fn clean_cache(path: &Path) {
    if let Err(error) = fs::remove_dir_all(path).await {
        warn!(path = %path.display(), cause = %error, "Failed to clean download cache");
    }
}
