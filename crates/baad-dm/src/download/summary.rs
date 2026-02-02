use std::sync::Arc;

use bon::Builder;
use reqwest_middleware::reqwest::StatusCode;

use crate::download::Download;
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    NotStarted,
    Success,
    Skipped(String),
    Failed(String),
    HashMismatch(String)
}

#[derive(Debug)]
pub(crate) enum FetchOutcome {
    Success { size: u64, resumable: bool },
    Skipped { reason: &'static str, size: u64 },
    Failed { error: String, status_code: StatusCode }
}

impl FetchOutcome {
    pub(crate) fn success(size: u64, resumable: bool) -> Self { Self::Success { size, resumable } }

    pub(crate) fn skipped(reason: &'static str, size: u64) -> Self {
        Self::Skipped { reason, size }
    }

    pub(crate) fn failed(error: impl ToString, status_code: StatusCode) -> Self {
        Self::Failed {
            error: error.to_string(),
            status_code
        }
    }

    pub(crate) fn from_result(result: Result<u64, Error>, resumable: bool) -> Self {
        match result {
            Ok(size) => Self::success(size, resumable),
            Err(e) => Self::failed(e, StatusCode::PARTIAL_CONTENT)
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
    pub status: Status,
    #[builder(default)]
    pub resumable: bool
}

impl Summary {
    pub fn success(mut self) -> Self {
        self.status = Status::Success;
        self
    }

    pub fn failed(mut self, reason: impl ToString) -> Self {
        self.status = Status::Failed(reason.to_string());
        self
    }

    pub fn skipped(mut self, reason: impl ToString) -> Self {
        self.status = Status::Skipped(reason.to_string());
        self
    }

    pub fn hash_mismatch(mut self, reason: impl ToString) -> Self {
        self.status = Status::HashMismatch(reason.to_string());
        self
    }

    pub fn is_success(&self) -> bool { matches!(self.status, Status::Success) }

    pub fn for_download(download: Download) -> Self {
        Self::builder().download(Arc::new(download)).build()
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_resumable(mut self, resumable: bool) -> Self {
        self.resumable = resumable;
        self
    }
}
