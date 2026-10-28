use std::path::{Path, PathBuf};

use futures::stream::{self, StreamExt, TryStreamExt};
use tokio::fs;

use crate::downloader::{TransferContext, TransferRequest};
use crate::error::Error;
use crate::segment::ServerSegment;

pub async fn fetch_segments(
    transfer: &TransferContext<'_>,
    segments: &[ServerSegment],
    cache_dir: &Path
) -> Result<Vec<PathBuf>, Error> {
    fs::create_dir_all(cache_dir).await?;

    let paths: Vec<_> =
        (0..segments.len()).map(|index| cache_dir.join(format!("segment-{index}"))).collect();

    let result = stream::iter(segments.iter().zip(&paths).enumerate())
        .map(Ok::<_, Error>)
        .try_for_each_concurrent(
            transfer.config.max_concurrent_segments.get(),
            |(index, (segment, path))| async move {
                transfer
                    .fetch(&TransferRequest {
                        url: &segment.url,
                        path,
                        expected_size: segment.size,
                        expected_hash: segment.hash.as_ref(),
                        progress_index: index
                    })
                    .await
            }
        )
        .await;

    if result.is_err() {
        transfer.cancellation.cancel();
    }

    result.map(|_| paths)
}
