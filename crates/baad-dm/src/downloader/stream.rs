use std::sync::Arc;

use futures::StreamExt;
use reqwest_middleware::reqwest::header::RANGE;
use reqwest_middleware::reqwest::{Response, StatusCode};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::client::create_range_header;
use crate::downloader::helpers::{FetchCtx, StreamOpts, ensure_parent_dir, validate_range};
use crate::downloader::progress::ProgressTracker;
use crate::error::Error;

const WRITE_BUFFER_SIZE: usize = 256 * 1024;

pub async fn download_stream(
    ctx: &FetchCtx<'_>,
    opts: StreamOpts,
    progress: Option<Arc<ProgressTracker>>
) -> Result<u64, Error> {
    let mut req = ctx.client.get(ctx.download.url.as_str());

    let resuming = opts.resumable && opts.size_on_disk > 0;
    if resuming {
        req = req.header(RANGE, create_range_header(opts.size_on_disk, None));
    }

    let res = req.send().await.map_err(Error::HttpMiddleware)?;

    let status = res.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::HttpStatus(status));
    }

    let expected_bytes = if resuming {
        Some(validate_range(&res, opts.size_on_disk, None, opts.total_size)?)
    } else if status != StatusCode::OK {
        return Err(Error::HttpStatus(status));
    } else {
        res.content_length()
    };

    ensure_parent_dir(&ctx.file_path).await?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(resuming)
        .truncate(!resuming)
        .open(&ctx.file_path)
        .await
        .map_err(Error::Io)?;

    let downloaded = stream_to_file(file, res, opts.size_on_disk, progress).await?;
    if expected_bytes.is_some_and(|expected| downloaded - opts.size_on_disk != expected) {
        return Err(Error::DownloadFailed(
            "Response body length does not match requested range".into()
        ));
    }
    Ok(downloaded)
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

        if let Some(p) = &progress {
            p.add_bytes(chunk.len() as u64);
        }
    }

    writer.flush().await.map_err(Error::Io)?;
    Ok(downloaded)
}
