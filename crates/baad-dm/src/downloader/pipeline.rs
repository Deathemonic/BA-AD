use std::path::Path;
use std::sync::Arc;

use reqwest_middleware::reqwest::Url;

use crate::download::{Download, DownloadSource, FetchOutcome};
use crate::error::Error;
use crate::output::{OutputJob, lock_file};
use crate::segment_service;
use crate::transfer::{TransferContext, TransferRequest};

pub async fn fetch_source(
    download: &Download,
    transfer: &TransferContext<'_>,
    destination: &Path
) -> Result<FetchOutcome, Error> {
    match &download.source {
        DownloadSource::Url(url) => fetch_url(download, transfer, destination, url).await,
        DownloadSource::Segments(segments) => {
            segment_service::download(transfer, download, segments, destination).await
        }
    }
}

async fn fetch_url(
    download: &Download,
    transfer: &TransferContext<'_>,
    destination: &Path,
    url: &Url
) -> Result<FetchOutcome, Error> {
    let path = destination.with_added_extension("baad-part");
    let lock = lock_file(&path.with_added_extension("lock"), &transfer.cancellation).await?;
    let passthrough = download.transform.is_none();

    transfer
        .fetch(&TransferRequest {
            url,
            path: &path,
            expected_size: if passthrough { download.size } else { None },
            expected_hash: if passthrough { download.hash.as_ref() } else { None },
            progress_index: 0
        })
        .await?;

    let job =
        OutputJob::new(download, destination.to_path_buf(), Arc::clone(&transfer.cancellation));
    let size = job.finish_download(transfer.shared, path, lock).await?;

    Ok(FetchOutcome::success(size, transfer.config.resumable))
}
