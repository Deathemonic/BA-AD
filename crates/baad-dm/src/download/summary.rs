use std::sync::Arc;

use baad_shared::ProgressStatus;
use bon::Builder;
use reqwest_middleware::reqwest::StatusCode;

use crate::download::Download;

#[derive(Debug)]
pub enum FetchOutcome {
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
}

#[derive(Debug, Clone, Builder)]
pub struct Summary {
    pub download: Arc<Download>,
    #[builder(default = StatusCode::OK)]
    pub status_code: StatusCode,
    #[builder(default)]
    pub size: u64,
    #[builder(default)]
    pub status: ProgressStatus,
    #[builder(default)]
    pub resumable: bool
}

impl Summary {
    pub const fn is_success(&self) -> bool { matches!(self.status, ProgressStatus::Success) }

    pub fn for_download(download: Download) -> Self {
        Self::builder().download(Arc::new(download)).build()
    }
}
