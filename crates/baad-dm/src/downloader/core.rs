use std::path::{Path, PathBuf};
use std::sync::Arc;

use baad_shared::{DownloadEvent, DownloadStatus};
use bacy::crypto::md5;
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
use crate::error::Error;
use crate::zip::{ZipCache, ZipExtractor};

#[derive(Clone, Debug)]
pub struct Downloader<'a> {
    config: DownloaderConfig<'a>
}

impl<'a> Downloader<'a> {
    pub const fn new(config: DownloaderConfig<'a>) -> Self { Self { config } }

    pub async fn download(&self, downloads: &[Download]) -> Vec<Summary> {
        let client_config = HttpClientConfig::builder()
            .retries(self.config.retries)
            .maybe_proxy(self.config.proxy.clone())
            .maybe_headers(self.config.headers.clone())
            .http1_only(self.config.http1_only)
            .build();

        let Ok(client) = create_http_client(client_config).map(Arc::new) else {
            return downloads
                .iter()
                .map(|d| {
                    Self::create_summary(
                        d.clone(),
                        FetchOutcome::failed(
                            &"Failed to create HTTP client",
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
                    client: Arc::clone(&client),
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
            total_bytes: ctx.download.size.unwrap_or(0)
        });

        let outcome = self.fetch_inner(&ctx, &filename).await.unwrap_or_else(|e| e);
        let summary = Self::create_summary(download, outcome);
        self.finalize(summary)
    }

    async fn fetch_inner(
        &self,
        ctx: &FetchCtx<'_>,
        filename: &Arc<str>
    ) -> Result<FetchOutcome, FetchOutcome> {
        // ZIP metadata describes the archive, not the extracted member.
        if ctx.download.is_extraction() {
            if !self.config.overwrite
                && ctx.download.hash.is_none()
                && let Ok(metadata) = fs::metadata(&ctx.file_path).await
            {
                return Ok(FetchOutcome::skipped("File exists", metadata.len()));
            }
            return self.extract_zip(ctx).await;
        }

        if !self.config.overwrite
            && let Ok(size) = ctx.download.verify_file(&ctx.file_path, None)
        {
            return Ok(FetchOutcome::skipped("File exists with matching size and hash", size));
        }

        let (supports_ranges, content_length, resolved_url) =
            check_server(&ctx.client, ctx.download)
                .await
                .map_err(|e| FetchOutcome::failed(&e, StatusCode::BAD_REQUEST))?;

        let resumable = supports_ranges && self.config.resumable;
        let total_size = content_length.unwrap_or(0);
        let chunk_size = 8 * 1024 * 1024;
        let chunk_count = if resumable && total_size >= self.config.chunk_threshold {
            let calculated = (total_size / chunk_size).max(1) as usize;
            calculated.clamp(1, self.config.max_chunks_per_file)
        } else {
            1
        };

        let staged_ctx = FetchCtx {
            client: Arc::clone(&ctx.client),
            download: ctx.download,
            file_path: staging_path(ctx),
            cache: ctx.cache
        };
        let partial_size =
            fs::metadata(&staged_ctx.file_path).await.map_or(0, |metadata| metadata.len());
        // Only sequential prefixes are resumable. A complete staging file might
        // be a preallocated chunk download interrupted before validation.
        let can_resume =
            !self.config.overwrite && resumable && chunk_count == 1 && ctx.download.hash.is_some();
        let size_on_disk = if can_resume && content_length.is_some_and(|size| partial_size < size) {
            partial_size
        } else {
            0
        };
        let opts = StreamOpts {
            size_on_disk,
            resumable,
            total_size: content_length
        };

        let tracker_total = content_length.or(ctx.download.size).unwrap_or(0);
        let progress_tracker = if tracker_total > 0 {
            Some(Arc::new(ProgressTracker::new(
                Arc::clone(filename),
                tracker_total,
                Arc::clone(&self.config.observer)
            )))
        } else {
            None
        };

        let result = if chunk_count > 1 {
            download_chunked(
                &staged_ctx,
                total_size,
                chunk_count,
                &resolved_url,
                self.config.max_concurrent_chunks,
                progress_tracker.clone()
            )
            .await
            .map(|()| total_size)
        } else {
            download_stream(&staged_ctx, opts, progress_tracker.clone()).await
        };

        let result = match result {
            Ok(_) => {
                let result = match ctx.download.verify_file(&staged_ctx.file_path, content_length) {
                    Ok(size) => fs::rename(&staged_ctx.file_path, &ctx.file_path)
                        .await
                        .map(|()| size)
                        .map_err(Error::Io),
                    Err(error) => Err(error)
                };
                if result.is_err() {
                    let _ = fs::remove_file(&staged_ctx.file_path).await;
                }
                result
            }
            Err(error) => {
                // A failed chunked download contains holes; only retain a
                // sequential prefix after a network or I/O interruption.
                let partial_size =
                    fs::metadata(&staged_ctx.file_path).await.map_or(0, |metadata| metadata.len());
                let keep_partial = can_resume
                    && matches!(error, Error::Http(_) | Error::Io(_))
                    && content_length.is_some_and(|size| partial_size > 0 && partial_size < size);
                if !keep_partial {
                    let _ = fs::remove_file(&staged_ctx.file_path).await;
                }
                Err(error)
            }
        };
        Ok(FetchOutcome::from_result(result, resumable))
    }

    async fn extract_zip(&self, ctx: &FetchCtx<'_>) -> Result<FetchOutcome, FetchOutcome> {
        let target = ctx.download.target_file.as_ref().ok_or_else(|| {
            FetchOutcome::failed(&"No target file for ZIP extraction", StatusCode::BAD_REQUEST)
        })?;

        let index = ctx
            .cache
            .get_index(&ctx.client, &ctx.download.url)
            .await
            .map_err(|e| FetchOutcome::failed(&e, StatusCode::BAD_REQUEST))?;

        let info = index.get(target).ok_or_else(|| {
            FetchOutcome::failed(
                &format!("File '{target}' not found in ZIP"),
                StatusCode::NOT_FOUND
            )
        })?;

        let data = ZipExtractor::extract_member(&ctx.client, &ctx.download.url, info)
            .await
            .map_err(|e| FetchOutcome::failed(&e, StatusCode::NOT_FOUND))?;

        let size = data.len() as u64;

        ensure_parent_dir(&ctx.file_path)
            .await
            .map_err(|e| FetchOutcome::failed(&e, StatusCode::INTERNAL_SERVER_ERROR))?;

        fs::write(&ctx.file_path, &data)
            .await
            .map_err(|e| FetchOutcome::failed(&e, StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(FetchOutcome::success(size, false))
    }

    fn create_summary(download: Download, outcome: FetchOutcome) -> Summary {
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

fn staging_path(ctx: &FetchCtx<'_>) -> PathBuf {
    let identity =
        format!("{}\n{:?}\n{:?}", ctx.download.url, ctx.download.hash, ctx.download.size);
    let digest = md5::to_hex_string(&md5::compute_hash(identity.as_bytes()));
    let mut filename = ctx.file_path.file_name().unwrap_or_default().to_os_string();
    filename.push(format!(".{digest}.baad-part"));
    ctx.file_path.with_file_name(filename)
}
