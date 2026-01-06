use super::Download;

use bon::Builder;
use reqwest_middleware::reqwest::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    NotStarted,
    Success,
    Skipped(String),
    Failed(String),
    HashMismatch(String),
}

#[derive(Debug, Clone, Builder)]
pub struct Summary {
    pub download: Download,
    #[builder(default = StatusCode::OK)]
    pub status_code: StatusCode,
    #[builder(default)]
    pub size: u64,
    #[builder(default)]
    pub status: Status,
    #[builder(default)]
    pub resumable: bool,
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

    pub fn is_success(&self) -> bool {
        matches!(self.status, Status::Success)
    }
}
