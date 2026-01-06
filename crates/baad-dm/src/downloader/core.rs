use super::config::DownloaderConfig;
use crate::client::{create_http_client, get_content_length, HttpClientConfig};
use crate::download::{Download, Status, Summary};
use crate::progress::ProgressDisplay;
use crate::zip::ZipExtractor;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use futures::stream::{self, StreamExt};
use indicatif::ProgressBar;
use reqwest_middleware::reqwest::header::RANGE;
use reqwest_middleware::reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter, SeekFrom};
use tracing::debug;

const WRITE_BUFFER_SIZE: usize = 256 * 1024;
const CHUNK_SIZE: u64 = 8 * 1024 * 1024;

type FetchResult = Result<Summary, Summary>;

struct FetchCtx<'a> {
    client: &'a ClientWithMiddleware,
    download: &'a Download,
    progress: &'a ProgressDisplay,
    file_path: PathBuf,
}

struct ChunkCtx {
    client: Arc<ClientWithMiddleware>,
    resolved_url: String,
    file_path: PathBuf,
}

struct StreamOpts {
    size_on_disk: u64,
    total_size: u64,
    resumable: bool,
}

struct ChunkRange {
    start: u64,
    end: u64,
}

#[derive(Clone, Debug)]
pub struct Downloader {
    config: DownloaderConfig,
}

impl Downloader {
    pub fn new(config: DownloaderConfig) -> Self {
        Self { config }
    }

    pub async fn download(&self, downloads: &[Download]) -> Vec<Summary> {
        let mut client_config = HttpClientConfig::new().with_retries(self.config.retries);

        if let Some(ref proxy) = self.config.proxy {
            client_config = client_config.with_proxy(proxy.clone());
        }

        if let Some(ref headers) = self.config.headers {
            client_config = client_config.with_headers(headers.clone());
        }

        let Ok(client) = create_http_client(client_config) else {
            return downloads
                .iter()
                .map(|d| {
                    self.make_summary(d, StatusCode::INTERNAL_SERVER_ERROR, 0, false)
                        .failed("Failed to create HTTP client")
                })
                .collect();
        };

        let progress = ProgressDisplay::new(
            self.config.style_options.clone(),
            downloads.len(),
            downloads.len() == 1,
        );

        let summaries = stream::iter(downloads)
            .map(|d| {
                let ctx = FetchCtx {
                    client: &client,
                    download: d,
                    progress: &progress,
                    file_path: self.config.directory.join(&d.filename),
                };
                self.fetch(ctx)
            })
            .buffer_unordered(self.config.concurrent_downloads)
            .collect::<Vec<_>>()
            .await;

        progress.finish();
        summaries
    }

    async fn fetch(&self, ctx: FetchCtx<'_>) -> Summary {
        let result = self.fetch_inner(&ctx).await;
        ctx.progress.increment_main();
        self.complete(result.unwrap_or_else(|e| e))
    }

    async fn fetch_inner(&self, ctx: &FetchCtx<'_>) -> FetchResult {
        if !self.config.overwrite
            && ctx.file_path.exists()
            && let Ok(true) = ctx.download.verify_hash(&ctx.file_path)
        {
            let size = fs::metadata(&ctx.file_path).await.map(|m| m.len()).unwrap_or(0);
            return Ok(self
                .make_summary(ctx.download, StatusCode::OK, size, false)
                .skipped("File exists with matching hash"));
        }

        if !self.config.overwrite
            && ctx.file_path.exists()
            && let Ok(false) = ctx.download.verify_hash(&ctx.file_path)
        {
            debug!("Hash mismatch, will redownload");
            let _ = fs::remove_file(&ctx.file_path).await;
        }

        if ctx.download.is_extraction() {
            return self.extract_zip(ctx).await;
        }

        let (resumable, content_length, resolved_url) = self.check_server(ctx.client, ctx.download).await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::BAD_REQUEST, 0, false)
                .failed(e)
        })?;

        let size_on_disk = match resumable && ctx.file_path.exists() {
            true => fs::metadata(&ctx.file_path).await.map(|m| m.len()).unwrap_or(0),
            false => 0,
        };

        if let Some(len) = content_length
            && len == size_on_disk
            && size_on_disk > 0
        {
            return Ok(self
                .make_summary(ctx.download, StatusCode::OK, len, resumable)
                .skipped("Already fully downloaded"));
        }

        let total_size = content_length.unwrap_or(0);
        let opts = StreamOpts { size_on_disk, total_size, resumable };

        let chunk_count = if resumable && total_size >= self.config.chunk_threshold {
            let calculated = (total_size / CHUNK_SIZE).max(1) as usize;
            calculated.clamp(1, self.config.max_chunks_per_file)
        } else {
            1
        };

        if chunk_count > 1 {
            self.download_chunked(ctx, opts.total_size, chunk_count, &resolved_url).await
        } else {
            self.download_stream(ctx, opts).await
        }
    }

    async fn check_server(
        &self,
        client: &ClientWithMiddleware,
        download: &Download,
    ) -> Result<(bool, Option<u64>, String), String> {
        let res = client.head(download.url.clone()).send().await.map_err(|e| e.to_string())?;
        
        let resolved_url = res.url().to_string();
        let headers = res.headers();

        let resumable = headers
            .get("accept-ranges")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v != "none");

        let content_length = headers
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        Ok((resumable, content_length, resolved_url))
    }

    async fn download_stream(&self, ctx: &FetchCtx<'_>, opts: StreamOpts) -> FetchResult {
        let mut req = ctx.client.get(ctx.download.url.as_str());

        if opts.resumable && opts.size_on_disk > 0 {
            req = req.header(RANGE, format!("bytes={}-", opts.size_on_disk));
        }

        let res = req.send().await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::BAD_REQUEST, 0, opts.resumable)
                .failed(e)
        })?;

        res.error_for_status_ref().map_err(|e| {
            self.make_summary(ctx.download, res.status(), 0, opts.resumable)
                .failed(e)
        })?;

        let actual_size = get_content_length(&res).unwrap_or(opts.total_size);
        let pb = ctx.progress.create_child(actual_size, opts.size_on_disk);

        self.ensure_parent_dir(&ctx.file_path).await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, opts.resumable)
                .failed(e)
        })?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(opts.resumable && opts.size_on_disk > 0)
            .truncate(!(opts.resumable && opts.size_on_disk > 0))
            .open(&ctx.file_path)
            .await
            .map_err(|e| {
                self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, opts.resumable)
                    .failed(e)
            })?;

        let downloaded = self.stream_to_file(file, res, opts.size_on_disk, &pb).await;
        ctx.progress.finish_child(pb);

        match downloaded {
            Ok(size) => Ok(self.make_summary(ctx.download, StatusCode::OK, size, opts.resumable).success()),
            Err((size, e)) => Err(self
                .make_summary(ctx.download, StatusCode::PARTIAL_CONTENT, size, opts.resumable)
                .failed(e)),
        }
    }

    async fn stream_to_file(
        &self,
        file: File,
        res: reqwest_middleware::reqwest::Response,
        initial_size: u64,
        pb: &ProgressBar,
    ) -> Result<u64, (u64, String)> {
        let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, file);
        let mut downloaded = initial_size;
        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| (downloaded, e.to_string()))?;
            writer.write_all(&chunk).await.map_err(|e| (downloaded, e.to_string()))?;
            downloaded += chunk.len() as u64;
            pb.inc(chunk.len() as u64);
        }

        writer.flush().await.map_err(|e| (downloaded, e.to_string()))?;
        Ok(downloaded)
    }

    async fn download_chunked(&self, ctx: &FetchCtx<'_>, total_size: u64, chunk_count: usize, resolved_url: &str) -> FetchResult {
        let chunk_size = total_size / chunk_count as u64;

        self.ensure_parent_dir(&ctx.file_path).await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, true)
                .failed(e)
        })?;

        let file = File::create(&ctx.file_path).await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, true)
                .failed(e)
        })?;

        file.set_len(total_size).await.map_err(|e| {
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, true)
                .failed(e)
        })?;

        drop(file);

        let pb = ctx.progress.create_child(total_size, 0);

        let ranges: Vec<_> = (0..chunk_count)
            .map(|i| {
                let start = i as u64 * chunk_size;
                let end = if i == chunk_count - 1 { total_size - 1 } else { (i as u64 + 1) * chunk_size - 1 };
                ChunkRange { start, end }
            })
            .collect();

        let concurrent_chunks = chunk_count.min(self.config.max_concurrent_chunks);
        
        let chunk_ctx = ChunkCtx {
            client: Arc::new(ctx.client.clone()),
            resolved_url: resolved_url.to_string(),
            file_path: ctx.file_path.clone(),
        };
        let chunk_ctx = Arc::new(chunk_ctx);

        let results: Vec<_> = stream::iter(ranges)
            .map(|range| {
                let chunk_ctx = Arc::clone(&chunk_ctx);
                let pb = pb.clone();
                async move { Self::download_chunk(&chunk_ctx, range, &pb).await }
            })
            .buffer_unordered(concurrent_chunks)
            .collect()
            .await;

        ctx.progress.finish_child(pb);

        if let Some(Err(e)) = results.into_iter().find(|r| r.is_err()) {
            return Err(self
                .make_summary(ctx.download, StatusCode::PARTIAL_CONTENT, 0, true)
                .failed(e));
        }

        Ok(self.make_summary(ctx.download, StatusCode::OK, total_size, true).success())
    }

    async fn download_chunk(chunk_ctx: &ChunkCtx, range: ChunkRange, pb: &ProgressBar) -> Result<(), String> {
        let res = chunk_ctx
            .client
            .get(&chunk_ctx.resolved_url)
            .header(RANGE, format!("bytes={}-{}", range.start, range.end))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        res.error_for_status_ref().map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .write(true)
            .open(&chunk_ctx.file_path)
            .await
            .map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(range.start)).await.map_err(|e| e.to_string())?;

        let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, file);
        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| e.to_string())?;
            writer.write_all(&chunk).await.map_err(|e| e.to_string())?;
            pb.inc(chunk.len() as u64);
        }

        writer.flush().await.map_err(|e| e.to_string())
    }

    async fn extract_zip(&self, ctx: &FetchCtx<'_>) -> FetchResult {
        let Some(ref target) = ctx.download.target_file else {
            return Err(self
                .make_summary(ctx.download, StatusCode::BAD_REQUEST, 0, false)
                .failed("No target file for ZIP extraction"));
        };

        let pb = ctx.progress.create_child(0, 0);

        let extractor = ZipExtractor::new(ctx.client, &ctx.download.url).await.map_err(|e| {
            ctx.progress.finish_child(pb.clone());
            self.make_summary(ctx.download, StatusCode::BAD_REQUEST, 0, false).failed(e)
        })?;

        let data = extractor.extract_file(target).await.map_err(|e| {
            ctx.progress.finish_child(pb.clone());
            self.make_summary(ctx.download, StatusCode::NOT_FOUND, 0, false).failed(e)
        })?;

        let size = data.len() as u64;

        self.ensure_parent_dir(&ctx.file_path).await.map_err(|e| {
            ctx.progress.finish_child(pb.clone());
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, false)
                .failed(e)
        })?;

        fs::write(&ctx.file_path, &data).await.map_err(|e| {
            ctx.progress.finish_child(pb.clone());
            self.make_summary(ctx.download, StatusCode::INTERNAL_SERVER_ERROR, 0, false)
                .failed(e)
        })?;

        ctx.progress.finish_child(pb);
        Ok(self.make_summary(ctx.download, StatusCode::OK, size, false).success())
    }

    async fn ensure_parent_dir(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    #[inline]
    fn make_summary(&self, download: &Download, status_code: StatusCode, size: u64, resumable: bool) -> Summary {
        Summary {
            download: download.clone(),
            status_code,
            size,
            status: Status::NotStarted,
            resumable,
        }
    }

    fn complete(&self, summary: Summary) -> Summary {
        if let Some(ref callback) = self.config.on_complete {
            callback(&summary);
        }
        summary
    }
}
