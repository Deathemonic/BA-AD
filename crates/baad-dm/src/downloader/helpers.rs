use std::path::{Path, PathBuf};
use std::sync::Arc;

use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::{Response, StatusCode};
use tokio::fs;

use crate::client::{parse_accept_ranges, parse_content_length};
use crate::download::Download;
use crate::error::Error;
use crate::zip::ZipCache;

#[derive(Debug)]
pub struct FetchCtx<'a> {
    pub client: Arc<ClientWithMiddleware>,
    pub download: &'a Download,
    pub file_path: PathBuf,
    pub cache: &'a ZipCache
}

pub struct StreamOpts {
    pub size_on_disk: u64,
    pub resumable: bool,
    pub total_size: Option<u64>
}

pub fn validate_range(
    response: &Response,
    start: u64,
    end: Option<u64>,
    total: Option<u64>
) -> Result<u64, Error> {
    if response.status() != StatusCode::PARTIAL_CONTENT {
        return Err(Error::RangeNotSupported);
    }

    let range = response.headers().get("content-range").and_then(|value| value.to_str().ok());
    let parsed = range
        .and_then(|value| value.strip_prefix("bytes "))
        .and_then(|value| value.split_once('/'))
        .and_then(|(bounds, total)| {
            let (start, end) = bounds.split_once('-')?;
            Some((start.parse::<u64>().ok()?, end.parse::<u64>().ok()?, total.parse::<u64>().ok()?))
        });

    match parsed {
        Some((actual_start, actual_end, actual_total))
            if actual_start == start
                && actual_start <= actual_end
                && actual_end < actual_total
                && end.is_none_or(|expected| actual_end == expected)
                && total.is_none_or(|expected| actual_total == expected)
                && (end.is_some() || actual_end == actual_total - 1) =>
        {
            Ok(actual_end - actual_start + 1)
        }
        _ => Err(Error::DownloadFailed("Invalid Content-Range response".into()))
    }
}

pub async fn check_server(
    client: &ClientWithMiddleware,
    download: &Download
) -> Result<(bool, Option<u64>, String), Error> {
    let res = client.head(download.url.clone()).send().await?;

    let resolved_url = res.url().to_string();
    // Some servers reject HEAD while serving GET normally. Error response
    // lengths describe the error page and must not validate the asset.
    if !res.status().is_success() {
        return Ok((false, None, resolved_url));
    }
    let headers = res.headers();

    let resumable = parse_accept_ranges(headers);
    let content_length = parse_content_length(headers);

    Ok((resumable, content_length, resolved_url))
}

pub async fn ensure_parent_dir(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(())
}
