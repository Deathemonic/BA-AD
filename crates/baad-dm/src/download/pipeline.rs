use std::path::Path;
use std::sync::Arc;

use crate::download::{Download, DownloadSource, FetchOutcome, validate_hash};
use crate::downloader::{OutputJob, TransferContext, TransferRequest, lock_file};
use crate::error::Error;
use crate::segment;

impl Download {
    pub fn network_total(&self) -> Option<u64> {
        match &self.source {
            DownloadSource::Url(_) if self.transform.is_none() => self.size,
            DownloadSource::Url(_) => None,
            DownloadSource::Segments(segments) => segment::network_total(segments)
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        validate_hash(self.hash.as_ref())?;
        if self.target_file.is_some() && self.transform.is_some() {
            return Err(Error::ZipTransformNotSupported);
        }
        match &self.source {
            DownloadSource::Url(_) => Ok(()),
            DownloadSource::Segments(segments) => segment::validate_segments(self, segments)
        }
    }

    pub fn progress_limits(&self) -> Vec<Option<u64>> {
        match &self.source {
            DownloadSource::Url(_) => vec![if self.transform.is_none() { self.size } else { None }],
            DownloadSource::Segments(segments) => segment::progress_limits(segments)
        }
    }

    pub async fn fetch_source(
        &self,
        transfer: &TransferContext<'_>,
        destination: &Path
    ) -> Result<FetchOutcome, Error> {
        match &self.source {
            DownloadSource::Url(url) => {
                let path = destination.with_added_extension("baad-part");
                let lock =
                    lock_file(&path.with_added_extension("lock"), &transfer.cancellation).await?;
                let passthrough = self.transform.is_none();
                transfer
                    .fetch(&TransferRequest {
                        url,
                        path: &path,
                        expected_size: if passthrough { self.size } else { None },
                        expected_hash: if passthrough { self.hash.as_ref() } else { None },
                        progress_index: 0
                    })
                    .await?;

                let job = OutputJob::new(
                    self,
                    destination.to_path_buf(),
                    Arc::clone(&transfer.cancellation)
                );
                let size = job.finish_download(transfer.shared, path, lock).await?;
                Ok(FetchOutcome::success(size, transfer.config.resumable))
            }
            DownloadSource::Segments(segments) => {
                segment::download(transfer, self, segments, destination).await
            }
        }
    }
}
