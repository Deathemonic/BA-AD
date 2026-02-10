use std::sync::Arc;

use futures::StreamExt;
use reqwest_middleware::reqwest::Response;
use reqwest_middleware::reqwest::header::RANGE;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::downloader::helpers::{FetchCtx, StreamOpts, ensure_parent_dir};
use crate::downloader::progress::ProgressTracker;
use crate::error::Error;

const WRITE_BUFFER_SIZE: usize = 256 * 1024;

pub async fn download_stream(
    ctx: &FetchCtx<'_>,
    opts: StreamOpts,
    progress: Option<Arc<ProgressTracker>>
) -> Result<u64, Error> {
    let mut req = ctx.client.get(ctx.download.url.as_str());

    if opts.resumable && opts.size_on_disk > 0 {
        req = req.header(RANGE, format!("bytes={}-", opts.size_on_disk));
    }

    let res = req.send().await.map_err(Error::HttpMiddleware)?;

    let status = res.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::HttpStatus(status));
    }

    ensure_parent_dir(&ctx.file_path).await?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(opts.resumable && opts.size_on_disk > 0)
        .truncate(!(opts.resumable && opts.size_on_disk > 0))
        .open(&ctx.file_path)
        .await
        .map_err(Error::Io)?;

    stream_to_file(file, res, opts.size_on_disk, progress).await
}

pub async fn stream_to_file(
    file: File,
    res: Response,
    initial_size: u64,
    progress: Option<Arc<ProgressTracker>>
) -> Result<u64, Error> {
    let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, file);
    let mut downloaded = initial_size;
    let mut stream = res.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(Error::Http)?;
        writer.write_all(&chunk).await.map_err(Error::Io)?;
        downloaded += chunk.len() as u64;

        if let Some(ref p) = progress {
            p.add_bytes(chunk.len() as u64);
        }
    }

    writer.flush().await.map_err(Error::Io)?;
    Ok(downloaded)
}
