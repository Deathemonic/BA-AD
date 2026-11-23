use std::path::Path;
use std::sync::Arc;

use baad_shared::{ProgressEvent, ProgressStatus, ProgressUnit};
use futures::stream::{self, StreamExt};
use reqwest_middleware::reqwest::StatusCode;
use tracing::{error, info, warn};

use crate::client::{HttpClientConfig, create_http_client};
use crate::download::{Download, FetchOutcome, Summary};
use crate::downloader::config::DownloaderConfig;
use crate::downloader::job::DownloadJob;
use crate::error::Error;
use crate::execution::{Cancellation, CancellationGuard, SharedExecution};
use crate::zip::ZipLimits;

#[derive(Clone, Debug)]
pub struct Downloader<'a> {
    config: DownloaderConfig<'a>,
    zip_limits: ZipLimits
}

impl<'a> Downloader<'a> {
    pub fn new(config: DownloaderConfig<'a>) -> Self {
        Self {
            config,
            zip_limits: ZipLimits::default()
        }
    }

    pub const fn with_zip_limits(mut self, limits: ZipLimits) -> Self {
        self.zip_limits = limits;

        self
    }

    pub async fn download(&self, downloads: &[Download]) -> Vec<Summary> {
        let shared = match self.shared_execution() {
            Ok(shared) => shared,
            Err(error) => {
                return downloads
                    .iter()
                    .map(|download| self.failed_download(download, &error))
                    .collect();
            }
        };

        stream::iter(downloads)
            .map(|download| self.run_download(&shared, download))
            .buffer_unordered(self.config.concurrent_downloads.get())
            .collect()
            .await
    }

    async fn run_download(&self, shared: &SharedExecution, download: &Download) -> Summary {
        let cancellation = Arc::new(Cancellation::default());
        let mut cancellation_guard = CancellationGuard::new(Arc::clone(&cancellation));
        let filename: Arc<str> = download.filename.as_str().into();
        let total = download.network_total().unwrap_or(0);

        self.config.observer.on_event(ProgressEvent::Started {
            id: Arc::clone(&filename),
            label: "Downloading".into(),
            unit: ProgressUnit::Bytes,
            total
        });

        let outcome = match download.validate() {
            Ok(()) => {
                let job = DownloadJob {
                    shared,
                    config: &self.config,
                    download,
                    zip_limits: self.zip_limits,
                    destination: self.config.directory.join(&download.filename),
                    cancellation
                };

                job.run(Arc::clone(&filename), total).await.unwrap_or_else(|error| {
                    FetchOutcome::failed(&error, StatusCode::PARTIAL_CONTENT)
                })
            }
            Err(error) => FetchOutcome::failed(&error, StatusCode::BAD_REQUEST)
        };

        cancellation_guard.disarm();

        self.finalize(Self::create_summary(download.clone(), outcome), filename)
    }

    fn shared_execution(&self) -> Result<SharedExecution, Error> {
        let client_config = HttpClientConfig::builder()
            .retries(0)
            .maybe_proxy(self.config.proxy.clone())
            .maybe_headers(self.config.headers.clone())
            .http1_only(self.config.http1_only)
            .build();

        let client = create_http_client(client_config)?;

        Ok(SharedExecution::new(
            client,
            self.config.max_concurrent_requests.get(),
            self.config.max_concurrent_transforms.get()
        ))
    }

    fn failed_download(&self, download: &Download, error: &impl ToString) -> Summary {
        let filename: Arc<str> = download.filename.as_str().into();

        self.config.observer.on_event(ProgressEvent::Started {
            id: Arc::clone(&filename),
            label: "Downloading".into(),
            unit: ProgressUnit::Bytes,
            total: download.network_total().unwrap_or(0)
        });

        self.finalize(
            Self::create_summary(
                download.clone(),
                FetchOutcome::failed(error, StatusCode::INTERNAL_SERVER_ERROR)
            ),
            filename
        )
    }

    fn create_summary(download: Download, outcome: FetchOutcome) -> Summary {
        let download = Arc::new(download);

        match outcome {
            FetchOutcome::Success { size, resumable } => Summary {
                download,
                status_code: StatusCode::OK,
                size,
                status: ProgressStatus::Success,
                resumable
            },
            FetchOutcome::Skipped { reason, size } => Summary {
                download,
                status_code: StatusCode::OK,
                size,
                status: ProgressStatus::Skipped(reason.into()),
                resumable: false
            },
            FetchOutcome::Failed { error, status_code } => Summary {
                download,
                status_code,
                size: 0,
                status: ProgressStatus::Failed(error.into()),
                resumable: false
            }
        }
    }

    fn finalize(&self, summary: Summary, filename: Arc<str>) -> Summary {
        self.config.observer.on_event(ProgressEvent::Completed {
            id: filename,
            status: summary.status.clone()
        });

        if self.config.log_results {
            log_summary(&summary);
        }

        summary
    }
}

fn log_summary(summary: &Summary) {
    let name = Path::new(&summary.download.filename)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&summary.download.filename);

    match &summary.status {
        ProgressStatus::Success => info!(file = name, success = true, "Downloaded"),
        ProgressStatus::Failed(error) => error!(file = name, cause = %error, "Failed"),
        ProgressStatus::Skipped(reason) => warn!(file = name, cause = %reason, "Skipped"),
        ProgressStatus::HashMismatch(reason) => {
            error!(file = name, cause = %reason, "Hash mismatch");
        }
        ProgressStatus::NotStarted => {}
    }
}
