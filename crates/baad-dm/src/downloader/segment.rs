use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest_middleware::reqwest::Url;
use tokio::fs;
use tokio::time::sleep;

use crate::download::ServerSegment;
use crate::downloader::config::DownloaderConfig;
use crate::downloader::helpers::{Cancellation, ServerInfo, SharedExecution};
use crate::downloader::progress::ProgressTracker;
use crate::error::Error;

pub(super) const WRITE_BUFFER_SIZE: usize = 256 * 1024;
const CHUNK_SIZE: u64 = 8 * 1024 * 1024;
const RETRY_BASE_DELAY_MS: u64 = 250;

#[derive(Debug)]
pub struct TransferRequest<'a> {
    pub url: &'a Url,
    pub path: &'a Path,
    pub expected_size: Option<u64>,
    pub expected_hash: Option<&'a String>,
    pub progress_index: usize
}

pub struct TransferContext<'a> {
    pub shared: &'a SharedExecution,
    pub config: &'a DownloaderConfig<'a>,
    pub progress: Arc<ProgressTracker>,
    pub cancellation: Arc<Cancellation>
}

pub(super) struct Transfer<'a, 'shared> {
    pub context: &'a TransferContext<'shared>,
    pub request: &'a TransferRequest<'a>,
    pub server: &'a ServerInfo,
    pub size_on_disk: u64
}

impl<'a> TransferContext<'a> {
    pub const fn new(
        shared: &'a SharedExecution,
        config: &'a DownloaderConfig<'a>,
        progress: Arc<ProgressTracker>,
        cancellation: Arc<Cancellation>
    ) -> Self {
        Self {
            shared,
            config,
            progress,
            cancellation
        }
    }

    pub async fn fetch_segments(
        &self,
        segments: &[ServerSegment],
        cache_dir: &Path
    ) -> Result<Vec<PathBuf>, Error> {
        fs::create_dir_all(cache_dir).await?;

        let paths: Vec<_> =
            (0..segments.len()).map(|index| cache_dir.join(format!("segment-{index}"))).collect();

        let result = stream::iter(segments.iter().zip(&paths).enumerate())
            .map(Ok::<_, Error>)
            .try_for_each_concurrent(
                self.config.max_concurrent_segments.get(),
                |(index, (segment, path))| async move {
                    self.fetch(&TransferRequest {
                        url: &segment.url,
                        path,
                        expected_size: segment.size,
                        expected_hash: segment.hash.as_ref(),
                        progress_index: index
                    })
                    .await
                }
            )
            .await;

        if result.is_err() {
            self.cancellation.cancel();
        }

        result.map(|_| paths)
    }

    pub async fn fetch(&self, request: &TransferRequest<'_>) -> Result<(), Error> {
        if !self.config.overwrite && self.verify_with_size(request, request.expected_size).await? {
            let size = fs::metadata(request.path).await?.len();
            self.progress.complete_position(request.progress_index, size);
            return Ok(());
        }

        let mut last_error = None;

        for attempt in 0..=self.config.retries {
            self.cancellation.check()?;

            match self.fetch_once(request).await {
                Ok(()) => return Ok(()),
                Err(error) => last_error = Some(error)
            }

            self.wait_before_retry(attempt).await;
        }

        Err(last_error.unwrap_or_else(|| Error::DownloadFailed("transfer did not start".into())))
    }

    async fn fetch_once(&self, request: &TransferRequest<'_>) -> Result<(), Error> {
        let server = self.shared.inspect(request.url, &self.cancellation).await?;
        let resumable = self.config.resumable && server.resumable;
        let mut size_on_disk = match fs::metadata(request.path).await {
            Ok(metadata) => metadata.len(),
            Err(error) if error.kind() == ErrorKind::NotFound => 0,
            Err(error) => return Err(error.into())
        };

        if self.config.overwrite
            || !resumable
            || server.content_length.is_some_and(|size| size_on_disk > size)
        {
            remove_if_exists(request.path).await?;
            size_on_disk = 0;
        }

        self.progress.set_position(request.progress_index, size_on_disk);

        let total_size = server.content_length.or(request.expected_size);
        if total_size != Some(size_on_disk) || size_on_disk == 0 {
            self.transfer(request, &server, size_on_disk).await?;
        }

        if self.verify_with_size(request, request.expected_size.or(server.content_length)).await? {
            let size = fs::metadata(request.path).await?.len();
            self.progress.complete_position(request.progress_index, size);
            return Ok(());
        }

        remove_if_exists(request.path).await?;

        self.progress.set_position(request.progress_index, 0);

        Err(Error::HashMismatch {
            expected: request
                .expected_hash
                .cloned()
                .unwrap_or_else(|| "expected transfer metadata".into())
                .into(),
            actual: "downloaded file did not match".into()
        })
    }

    async fn transfer(
        &self,
        request: &TransferRequest<'_>,
        server: &ServerInfo,
        size_on_disk: u64
    ) -> Result<(), Error> {
        let transfer = Transfer {
            context: self,
            request,
            server,
            size_on_disk
        };
        let total_size = server.content_length.or(request.expected_size);
        let resumable = self.config.resumable && server.resumable;
        let chunk_count = self.chunk_count(total_size, size_on_disk, resumable);
        if chunk_count == 1 {
            return transfer.run_stream().await.map(|_| ());
        }

        let result = transfer.run_chunks(total_size.unwrap_or(0), chunk_count).await;

        if result.is_err() {
            remove_if_exists(request.path).await?;

            self.progress.set_position(request.progress_index, 0);
        }

        result
    }

    async fn verify_with_size(
        &self,
        request: &TransferRequest<'_>,
        expected_size: Option<u64>
    ) -> Result<bool, Error> {
        if expected_size.is_none() && request.expected_hash.is_none() {
            return Ok(false);
        }
        self.shared.verify_file(request.path, expected_size, request.expected_hash).await
    }

    fn chunk_count(&self, total_size: Option<u64>, size_on_disk: u64, resumable: bool) -> usize {
        total_size.map_or(1, |size| {
            if resumable && size_on_disk == 0 && size >= self.config.chunk_threshold {
                ((size / CHUNK_SIZE).max(1) as usize)
                    .clamp(1, self.config.max_chunks_per_file.get())
            } else {
                1
            }
        })
    }

    async fn wait_before_retry(&self, attempt: u32) {
        if attempt < self.config.retries {
            let shift = attempt.min(8);
            sleep(Duration::from_millis(RETRY_BASE_DELAY_MS << shift)).await;
        }
    }
}

async fn remove_if_exists(path: &Path) -> Result<(), Error> {
    match fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into())
    }
}
