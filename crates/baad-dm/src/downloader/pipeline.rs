use reqwest_middleware::reqwest::Url;

use crate::download::{DownloadSource, FetchOutcome};
use crate::error::Error;
use crate::output::{self, OutputJob};
use crate::segment_service;
use crate::transfer::{TransferContext, TransferRequest};

pub async fn fetch_source(
    source: &DownloadSource,
    transfer: &TransferContext<'_>,
    job: OutputJob
) -> Result<FetchOutcome, Error> {
    match source {
        DownloadSource::Url(url) => fetch_url(url, transfer, job).await,
        DownloadSource::Segments(segments) => {
            segment_service::download(transfer, segments, job).await
        }
    }
}

async fn fetch_url(
    url: &Url,
    transfer: &TransferContext<'_>,
    job: OutputJob
) -> Result<FetchOutcome, Error> {
    let path = job.destination.with_added_extension("baad-part");
    let lock =
        output::lock_file(&path.with_added_extension("lock"), &transfer.cancellation).await?;
    let passthrough = job.transform.is_none();

    transfer
        .fetch(&TransferRequest {
            url,
            path: &path,
            expected_size: if passthrough { job.expected_size } else { None },
            expected_hash: if passthrough { job.expected_hash.as_ref() } else { None },
            progress_index: 0
        })
        .await?;

    let size = job.finish_download(transfer.shared, path, lock).await?;

    Ok(FetchOutcome::success(size, transfer.config.resumable))
}
