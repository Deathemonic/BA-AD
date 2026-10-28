use std::path::Path;
use std::sync::Arc;

use tokio::fs;
use tokio::task::spawn_blocking;

use crate::download::{Download, FetchOutcome};
use crate::downloader::{OutputJob, TransferContext};
use crate::error::Error;
use crate::segment::cache::SegmentCache;
use crate::segment::{OrderedSegmentReader, ServerSegment, fetch_segments};

pub async fn download(
    transfer: &TransferContext<'_>,
    download: &Download,
    segments: &[ServerSegment],
    destination: &Path
) -> Result<FetchOutcome, Error> {
    let cache = SegmentCache::acquire(transfer, segments).await?;
    if !transfer.config.overwrite
        && transfer.shared.verify_file(destination, download.size, download.hash.as_ref()).await?
    {
        let size = fs::metadata(destination).await?.len();
        return Ok(FetchOutcome::skipped("File exists with matching metadata", size));
    }

    let paths = fetch_segments(transfer, segments, &cache.directory).await?;
    let limiter = if download.transform.is_some() {
        &transfer.shared.transforms
    } else {
        &transfer.shared.blocking
    };
    let permit = limiter.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).await?;
    }
    let job =
        OutputJob::new(download, destination.to_path_buf(), Arc::clone(&transfer.cancellation));
    let cache_lock = Arc::clone(&cache.lock);

    let size = spawn_blocking(move || {
        let _permit = permit;
        let _cache_lock = cache_lock;
        job.run(&mut OrderedSegmentReader::new(paths))
    })
    .await??;

    cache.clean().await;
    Ok(FetchOutcome::success(size, transfer.config.resumable))
}
