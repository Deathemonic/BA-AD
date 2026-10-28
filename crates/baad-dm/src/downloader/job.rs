use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use fastcat::fconcat;
use reqwest_middleware::reqwest::Url;
use tokio::fs;
use tokio::time::sleep;

use crate::download::{Download, FetchOutcome};
use crate::downloader::config::DownloaderConfig;
use crate::downloader::pipeline::fetch_source;
use crate::error::Error;
use crate::execution::{Cancellation, ProgressTracker, SharedExecution};
use crate::output::{OutputJob, ensure_parent_dir, lock_file, verify_file};
use crate::transfer::TransferContext;
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
        let _output_lock =
            lock_file(&self.destination.with_added_extension("baad-lock"), &self.cancellation)
                .await?;

        if !self.config.overwrite
            && verify_file(
                &self.shared.blocking,
                &self.destination,
                self.download.size,
                self.download.hash.as_ref()
            )
            .await?
        {
            let size = fs::metadata(&self.destination).await?.len();

            return Ok(FetchOutcome::skipped("File exists with matching metadata", size));
        }

        ensure_parent_dir(&self.destination).await?;

        if self.download.is_extraction() {
            return self.extract_zip().await;
        }

        let progress = Arc::new(ProgressTracker::new(
            filename,
            total,
            self.download.progress_limits(),
            Arc::clone(&self.config.observer)
        ));
        let transfer = TransferContext {
            shared: self.shared,
            config: self.config,
            progress,
            cancellation: Arc::clone(&self.cancellation)
        };

        fetch_source(self.download, &transfer, &self.destination).await
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
        let job =
            OutputJob::new(self.download, self.destination.clone(), Arc::clone(&self.cancellation));
        let size = job.finish_bytes(self.shared, data).await?;

        Ok(FetchOutcome::success(size, false))
    }

    async fn fetch_zip_member(&self, url: &Url, target: &str) -> Result<Bytes, Error> {
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

        Err(last_error.unwrap_or(Error::TransferNotStarted))
    }

    async fn try_zip_member(&self, url: &Url, target: &str) -> Result<Bytes, Error> {
        let source = ZipSource::new(&self.shared.client, url, Some(&self.shared.requests));
        let index = self.shared.zip_cache.get_index(source).await?;
        let info = index.get(target).ok_or_else(|| {
            Error::Archive(fconcat!("File '", target, "' not found in ZIP").into())
        })?;

        source.extract_member(info).await
    }
}
