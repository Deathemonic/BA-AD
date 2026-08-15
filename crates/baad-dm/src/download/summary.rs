use std::sync::Arc;

use baad_shared::DownloadStatus;
use bon::Builder;
use reqwest_middleware::reqwest::StatusCode;

use crate::download::Download;
use crate::error::Error;

#[derive(Debug)]
pub(crate) enum FetchOutcome {
    Success { size: u64, resumable: bool },
    Skipped { reason: &'static str, size: u64 },
    Failed { error: String, status_code: StatusCode }
}

impl FetchOutcome {
    pub(crate) const fn success(size: u64, resumable: bool) -> Self {
        Self::Success { size, resumable }
    }

    pub(crate) const fn skipped(reason: &'static str, size: u64) -> Self {
        Self::Skipped { reason, size }
    }

    pub(crate) fn failed(error: &impl ToString, status_code: StatusCode) -> Self {
        Self::Failed {
            error: error.to_string(),
            status_code
        }
    }

    pub(crate) fn from_result(result: Result<u64, Error>, resumable: bool) -> Self {
        match result {
            Ok(size) => Self::success(size, resumable),
            Err(e) => Self::failed(&e, StatusCode::PARTIAL_CONTENT)
        }
    }
}

#[derive(Debug, Clone, Builder)]
pub struct Summary {
    pub download: Arc<Download>,
    #[builder(default = StatusCode::OK)]
    pub status_code: StatusCode,
    #[builder(default)]
    pub size: u64,
    #[builder(default)]
    pub status: DownloadStatus,
    #[builder(default)]
    pub resumable: bool
}

impl Summary {
    pub const fn is_success(&self) -> bool { matches!(self.status, DownloadStatus::Success) }

    pub fn for_download(download: Download) -> Self {
        Self::builder().download(Arc::new(download)).build()
    }
}
