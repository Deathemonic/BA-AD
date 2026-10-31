use std::sync::Arc;

use tokio::{fs, task};

use crate::download::FetchOutcome;
use crate::error::Error;
use crate::output::{self, OutputJob};
use crate::segment::ServerSegment;
use crate::segment_service::cache::SegmentCache;
use crate::segment_service::{OrderedSegmentReader, fetch_segments};
use crate::transfer::TransferContext;

pub async fn download(
    transfer: &TransferContext<'_>,
    segments: &[ServerSegment],
    job: OutputJob
) -> Result<FetchOutcome, Error> {
    let cache = SegmentCache::acquire(transfer, segments).await?;

    if !transfer.config.overwrite
        && output::verify_file(
            &transfer.shared.blocking,
            &job.destination,
            job.expected_size,
            job.expected_hash.as_ref()
        )
        .await?
    {
        let size = fs::metadata(&job.destination).await?.len();

        return Ok(FetchOutcome::skipped("File exists with matching metadata", size));
    }

    let paths = fetch_segments(transfer, segments, &cache.directory).await?;
    let limiter = if job.transform.is_some() {
        &transfer.shared.transforms
    } else {
        &transfer.shared.blocking
    };
    let permit = limiter.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;

    if let Some(parent) = job.destination.parent() {
        fs::create_dir_all(parent).await?;
    }

    let cache_lock = Arc::clone(&cache.lock);

    let size = task::spawn_blocking(move || {
        let _permit = permit;
        let _cache_lock = cache_lock;

        job.run(&mut OrderedSegmentReader::new(paths))
    })
    .await??;

    cache.clean().await;

    Ok(FetchOutcome::success(size, transfer.config.resumable))
}
