use std::path::PathBuf;
use std::sync::Arc;

use futures::stream::{self, StreamExt};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::header::RANGE;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter, SeekFrom};

use crate::downloader::helpers::{FetchCtx, ensure_parent_dir};
use crate::downloader::progress::ProgressTracker;
use crate::error::Error;

const WRITE_BUFFER_SIZE: usize = 256 * 1024;

pub struct ChunkCtx {
    pub client: Arc<ClientWithMiddleware>,
    pub resolved_url: String,
    pub file_path: PathBuf
}

pub struct ChunkRange {
    pub start: u64,
    pub end: u64
}

pub async fn download_chunked(
    ctx: &FetchCtx<'_>,
    total_size: u64,
    chunk_count: usize,
    resolved_url: &str,
    max_concurrent_chunks: usize,
    progress: Option<Arc<ProgressTracker>>
) -> Result<(), Error> {
    let chunk_size = total_size / chunk_count as u64;

    ensure_parent_dir(&ctx.file_path).await?;

    let file = File::create(&ctx.file_path).await.map_err(Error::Io)?;

    file.set_len(total_size).await.map_err(Error::Io)?;

    drop(file);

    let ranges: Vec<_> = (0..chunk_count)
        .map(|i| {
            let start = i as u64 * chunk_size;
            let end =
                if i == chunk_count - 1 { total_size - 1 } else { (i as u64 + 1) * chunk_size - 1 };
            ChunkRange { start, end }
        })
        .collect();

    let concurrent_chunks = chunk_count.min(max_concurrent_chunks);

    let chunk_ctx = ChunkCtx {
        client: Arc::new(ctx.client.clone()),
        resolved_url: resolved_url.to_string(),
        file_path: ctx.file_path.clone()
    };
    let chunk_ctx = Arc::new(chunk_ctx);

    let results: Vec<_> = stream::iter(ranges)
        .map(|range| {
            let chunk_ctx = Arc::clone(&chunk_ctx);
            let progress = progress.clone();
            async move { download_chunk(&chunk_ctx, range, progress).await }
        })
        .buffer_unordered(concurrent_chunks)
        .collect()
        .await;

    if let Some(Err(e)) = results.into_iter().find(|r| r.is_err()) {
        return Err(e);
    }

    Ok(())
}

async fn download_chunk(
    chunk_ctx: &ChunkCtx,
    range: ChunkRange,
    progress: Option<Arc<ProgressTracker>>
) -> Result<(), Error> {
    let res = chunk_ctx
        .client
        .get(&chunk_ctx.resolved_url)
        .header(RANGE, format!("bytes={}-{}", range.start, range.end))
        .send()
        .await?;

    res.error_for_status_ref()?;

    let mut file = OpenOptions::new().write(true).open(&chunk_ctx.file_path).await?;

    file.seek(SeekFrom::Start(range.start)).await?;

    let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, file);
    let mut stream = res.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer.write_all(&chunk).await?;

        if let Some(ref p) = progress {
            p.add_bytes(chunk.len() as u64);
        }
    }

    writer.flush().await.map_err(Error::from)
}
