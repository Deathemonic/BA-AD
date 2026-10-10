use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use baad_shared::{DownloadEvent, DownloadStatus};
use bacy::crypto::md5;
use bacy::hash::crc;
use futures::stream::{self, StreamExt};
use reqwest_middleware::reqwest::StatusCode;
use tokio::{fs, task};
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
use crate::zip::{ZipCache, ZipExtractor, ZipFileInfo};

const HASH_BUFFER_SIZE: usize = 256 * 1024;

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
        if ctx.download.is_extraction() {
            return self.extract_zip(ctx).await;
        }

        let staging = Staging::new(&ctx.file_path);
        if !self.config.overwrite
            && let Ok(size) = verify_file(ctx.download, &ctx.file_path, None).await
        {
            staging.discard().await;
            return Ok(FetchOutcome::skipped("File exists with matching size and hash", size));
        }

        let (supports_ranges, content_length, resolved_url) =
            check_server(&ctx.client, ctx.download)
                .await
                .map_err(|e| FetchOutcome::failed(&e, StatusCode::BAD_REQUEST))?;

        let resumable = supports_ranges && self.config.resumable;
        let total_size = content_length.unwrap_or(0);
        let chunk_size = 8 * 1024 * 1024;
        let chunk_count = if supports_ranges && total_size >= self.config.chunk_threshold {
            let calculated = (total_size / chunk_size).max(1) as usize;
            calculated.clamp(1, self.config.max_chunks_per_file)
        } else {
            1
        };

        let staged_ctx = FetchCtx {
            client: Arc::clone(&ctx.client),
            download: ctx.download,
            file_path: staging.path.clone(),
            cache: ctx.cache
        };
        let can_resume =
            !self.config.overwrite && resumable && chunk_count == 1 && ctx.download.hash.is_some();
        let size_on_disk = match staging.prepare(ctx.download, can_resume, content_length).await {
            Ok(size) => size,
            Err(error) => return Ok(FetchOutcome::from_result(Err(Error::Io(error)), resumable))
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
                let result = match verify_file(ctx.download, &staging.path, content_length).await {
                    Ok(size) => staging.publish(&ctx.file_path).await.map(|()| size),
                    Err(error) => Err(error)
                };
                if result.is_err() {
                    staging.discard().await;
                }
                result
            }
            Err(error) => {
                // A failed chunked download contains holes; only retain a
                // sequential prefix after a network or I/O interruption.
                let partial_size =
                    fs::metadata(&staging.path).await.map_or(0, |metadata| metadata.len());
                let keep_partial = can_resume
                    && matches!(error, Error::Http(_) | Error::Io(_))
                    && content_length.is_some_and(|size| partial_size > 0 && partial_size < size);
                if !keep_partial {
                    staging.discard().await;
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

        // ZIP metadata on the download describes the archive; the member is
        // checked against its own central directory entry instead.
        if !self.config.overwrite && member_matches(&ctx.file_path, info).await {
            return Ok(FetchOutcome::skipped(
                "File exists with matching size and CRC",
                info.uncompressed_size
            ));
        }

        let data = ZipExtractor::extract_member(&ctx.client, &ctx.download.url, info)
            .await
            .map_err(|e| FetchOutcome::failed(&e, StatusCode::NOT_FOUND))?;

        let size = data.len() as u64;
        if size != info.uncompressed_size || crc32fast::hash(&data) != info.crc32 {
            return Err(FetchOutcome::failed(
                &format!("Extracted '{target}' does not match ZIP size and CRC"),
                StatusCode::UNPROCESSABLE_ENTITY
            ));
        }

        let staging = Staging::new(&ctx.file_path);
        let written = async {
            ensure_parent_dir(&staging.path).await?;
            staging.unmark().await?;
            fs::write(&staging.path, &data).await?;
            staging.publish(&ctx.file_path).await
        }
        .await;
        if let Err(error) = written {
            staging.discard().await;
            return Err(FetchOutcome::failed(&error, StatusCode::INTERNAL_SERVER_ERROR));
        }

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

/// Downloads are written to `<name>.baad-part` and only renamed into place
/// after validation. A sequential partial is kept for resume together with a
/// `<name>.baad-part.id` marker naming the URL, hash and size it belongs to.
struct Staging {
    path: PathBuf,
    marker: PathBuf
}

impl Staging {
    fn new(file_path: &Path) -> Self {
        let mut path = file_path.as_os_str().to_os_string();
        path.push(".baad-part");
        let mut marker = path.clone();
        marker.push(".id");
        Self {
            path: path.into(),
            marker: marker.into()
        }
    }

    /// Returns the prefix length to resume from. Only sequential prefixes are
    /// resumable, and only when the marker proves they belong to this exact
    /// download. Chunked downloads preallocate the file and never get a
    /// marker.
    async fn prepare(
        &self,
        download: &Download,
        can_resume: bool,
        content_length: Option<u64>
    ) -> io::Result<u64> {
        if !can_resume {
            self.unmark().await?;
            return Ok(0);
        }
        let identity = staging_identity(download);
        let partial_size =
            if fs::read_to_string(&self.marker).await.is_ok_and(|marker| marker == identity) {
                fs::metadata(&self.path).await.map_or(0, |metadata| metadata.len())
            } else {
                0
            };
        ensure_parent_dir(&self.marker).await.map_err(io::Error::other)?;
        fs::write(&self.marker, identity).await?;
        Ok(if content_length.is_some_and(|size| partial_size < size) { partial_size } else { 0 })
    }

    async fn unmark(&self) -> io::Result<()> { remove_if_exists(&self.marker).await }

    async fn publish(&self, destination: &Path) -> Result<(), Error> {
        fs::rename(&self.path, destination).await?;
        let _ = self.unmark().await;
        Ok(())
    }

    async fn discard(&self) {
        let _ = remove_if_exists(&self.path).await;
        let _ = self.unmark().await;
    }
}

async fn remove_if_exists(path: &Path) -> io::Result<()> {
    match fs::remove_file(path).await {
        Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error),
        _ => Ok(())
    }
}

fn staging_identity(download: &Download) -> String {
    let identity = format!(
        "{}\n{:?}\n{:?}\n{:?}",
        download.url, download.hash, download.hash_type, download.size
    );
    md5::to_hex_string(&md5::compute_hash(identity.as_bytes()))
}

/// Hashing reads the whole file, so keep it off the async workers.
async fn verify_file(
    download: &Download,
    path: &Path,
    server_size: Option<u64>
) -> Result<u64, Error> {
    let download = download.clone();
    let path = path.to_path_buf();
    task::spawn_blocking(move || download.verify_file(&path, server_size))
        .await
        .map_err(|error| Error::Io(io::Error::other(error)))?
}

async fn member_matches(path: &Path, info: &ZipFileInfo) -> bool {
    if fs::metadata(path).await.map_or(true, |metadata| metadata.len() != info.uncompressed_size) {
        return false;
    }
    let path = path.to_path_buf();
    task::spawn_blocking(move || crc::compute_streaming(&path, HASH_BUFFER_SIZE, None))
        .await
        .is_ok_and(|actual| actual.is_ok_and(|actual| actual == info.crc32))
}
