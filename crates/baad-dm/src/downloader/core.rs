use std::path::Path;
use std::sync::Arc;

use baad_shared::{DownloadEvent, DownloadStatus};
use futures::stream::{self, StreamExt};
use reqwest_middleware::reqwest::StatusCode;
use tokio::fs;
use tracing::{error, info, warn};

use crate::client::{HttpClientConfig, create_http_client};
use crate::download::summary::FetchOutcome;
use crate::download::{Download, Summary};
use crate::downloader::chunk::download_chunked;
use crate::downloader::config::DownloaderConfig;
use crate::downloader::helpers::{FetchCtx, StreamOpts, check_server, ensure_parent_dir};
use crate::downloader::progress::ProgressTracker;
use crate::downloader::stream::download_stream;
use crate::zip::{ZipCache, ZipExtractor};

#[derive(Clone, Debug)]
pub struct Downloader<'a> {
    config: DownloaderConfig<'a>
}

impl<'a> Downloader<'a> {
    pub fn new(config: DownloaderConfig<'a>) -> Self { Self { config } }

    pub async fn download(&self, downloads: &[Download]) -> Vec<Summary> {
        let client_config = HttpClientConfig::builder()
            .retries(self.config.retries)
            .maybe_proxy(self.config.proxy.clone())
            .maybe_headers(self.config.headers.clone())
            .build();

        let Ok(client) = create_http_client(client_config) else {
            return downloads
                .iter()
                .map(|d| {
                    self.create_summary(
                        d.clone(),
                        FetchOutcome::failed(
                            "Failed to create HTTP client",
                            StatusCode::INTERNAL_SERVER_ERROR
                        )
                    )
                })
                .collect();
        };

        let zip_cache = ZipCache::new();

        stream::iter(downloads)
            .map(|d| {
                let ctx = FetchCtx {
                    client: &client,
                    download: d,
                    file_path: self.config.directory.join(&d.filename),
                    cache: &zip_cache
                };
                async move { self.fetch_with_progress(ctx).await }
            })
            .buffer_unordered(self.config.concurrent_downloads)
            .collect::<Vec<_>>()
            .await
    }

    async fn fetch_with_progress(&self, ctx: FetchCtx<'_>) -> Summary {
        let download = ctx.download.clone();
        let filename: Arc<str> = ctx.download.filename.as_str().into();

        self.config.observer.on_event(DownloadEvent::Started {
            filename: Arc::clone(&filename),
            total_bytes: 0
        });

        let outcome = self.fetch_inner(&ctx, &filename).await.unwrap_or_else(|e| e);
        let summary = self.create_summary(download, outcome);
        self.finalize(summary)
    }

    async fn fetch_inner(
        &self,
        ctx: &FetchCtx<'_>,
        filename: &Arc<str>
    ) -> Result<FetchOutcome, FetchOutcome> {
        if !self.config.overwrite
            && ctx.file_path.exists()
            && let Ok(true) = ctx.download.verify_hash(&ctx.file_path)
        {
            let size = fs::metadata(&ctx.file_path).await.map(|m| m.len()).unwrap_or(0);
            return Ok(FetchOutcome::skipped("File exists with matching hash", size));
        }

        if !self.config.overwrite
            && ctx.file_path.exists()
            && let Ok(false) = ctx.download.verify_hash(&ctx.file_path)
        {
            let _ = fs::remove_file(&ctx.file_path).await;
        }

        if ctx.download.is_extraction() {
            return self.extract_zip(ctx).await;
        }

        let (resumable, content_length, resolved_url) = check_server(ctx.client, ctx.download)
            .await
            .map_err(|e| FetchOutcome::failed(e, StatusCode::BAD_REQUEST))?;

        let size_on_disk = match resumable && ctx.file_path.exists() {
            true => fs::metadata(&ctx.file_path).await.map(|m| m.len()).unwrap_or(0),
            false => 0
        };

        if let Some(len) = content_length
            && len == size_on_disk
            && size_on_disk > 0
        {
            return Ok(FetchOutcome::skipped("Already fully downloaded", len));
        }

        let total_size = content_length.unwrap_or(0);
        let opts = StreamOpts { size_on_disk, resumable };

        let chunk_size = 8 * 1024 * 1024;
        let chunk_count = if resumable && total_size >= self.config.chunk_threshold {
            let calculated = (total_size / chunk_size).max(1) as usize;
            calculated.clamp(1, self.config.max_chunks_per_file)
        } else {
            1
        };

        let progress_tracker = if total_size > 0 {
            Some(Arc::new(ProgressTracker::new(
                Arc::clone(filename),
                total_size,
                Arc::clone(&self.config.observer)
            )))
        } else {
            None
        };

        let result = if chunk_count > 1 {
            download_chunked(
                ctx,
                total_size,
                chunk_count,
                &resolved_url,
                self.config.max_concurrent_chunks,
                progress_tracker.clone()
            )
            .await
            .map(|()| total_size)
        } else {
            download_stream(ctx, opts, progress_tracker.clone()).await
        };

        Ok(FetchOutcome::from_result(result, resumable))
    }

    async fn extract_zip(&self, ctx: &FetchCtx<'_>) -> Result<FetchOutcome, FetchOutcome> {
        let target = ctx.download.target_file.as_ref().ok_or_else(|| {
            FetchOutcome::failed("No target file for ZIP extraction", StatusCode::BAD_REQUEST)
        })?;

        let index = ctx
            .cache
            .get_index(ctx.client, &ctx.download.url)
            .await
            .map_err(|e| FetchOutcome::failed(e, StatusCode::BAD_REQUEST))?;

        let info = index.get(target).ok_or_else(|| {
            FetchOutcome::failed(
                format!("File '{}' not found in ZIP", target),
                StatusCode::NOT_FOUND
            )
        })?;

        let data = ZipExtractor::extract_member(ctx.client, &ctx.download.url, info)
            .await
            .map_err(|e| FetchOutcome::failed(e, StatusCode::NOT_FOUND))?;

        let size = data.len() as u64;

        ensure_parent_dir(&ctx.file_path)
            .await
            .map_err(|e| FetchOutcome::failed(e, StatusCode::INTERNAL_SERVER_ERROR))?;

        fs::write(&ctx.file_path, &data)
            .await
            .map_err(|e| FetchOutcome::failed(e, StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(FetchOutcome::success(size, false))
    }

    fn create_summary(&self, download: Download, outcome: FetchOutcome) -> Summary {
        let download = Arc::new(download);

        match outcome {
            FetchOutcome::Success { size, resumable } => Summary {
                download,
                status_code: StatusCode::OK,
                size,
                status: DownloadStatus::Success,
                resumable
            },
            FetchOutcome::Skipped { reason, size } => Summary {
                download,
                status_code: StatusCode::OK,
                size,
                status: DownloadStatus::Skipped(reason.into()),
                resumable: false
            },
            FetchOutcome::Failed { error, status_code } => Summary {
                download,
                status_code,
                size: 0,
                status: DownloadStatus::Failed(error.into()),
                resumable: false
            }
        }
    }

    fn finalize(&self, summary: Summary) -> Summary {
        let filename: Arc<str> = summary.download.filename.as_str().into();

        self.config.observer.on_event(DownloadEvent::Completed {
            filename,
            size: summary.size,
            status: summary.status.clone()
        });

        let name = Path::new(&summary.download.filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&summary.download.filename);

        match &summary.status {
            DownloadStatus::Success => {
                info!(file = name, success = true, "Downloaded");
            }
            DownloadStatus::Failed(error) => {
                error!(file = name, cause = %error, "Failed");
            }
            DownloadStatus::Skipped(reason) => {
                warn!(file = name, cause = %reason, "Skipped");
            }
            DownloadStatus::HashMismatch(reason) => {
                error!(file = name, cause = %reason, "Hash mismatch");
            }
            DownloadStatus::NotStarted => {}
        }

        summary
    }
}
