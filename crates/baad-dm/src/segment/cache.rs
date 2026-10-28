use std::fmt::Write;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use bacy::hash::sha;
use tokio::fs;
use tracing::warn;

use crate::downloader::{TransferContext, lock_file};
use crate::error::Error;
use crate::segment::ServerSegment;

pub struct SegmentCache {
    pub directory: PathBuf,
    pub lock: Arc<File>
}

impl SegmentCache {
    pub async fn acquire(
        transfer: &TransferContext<'_>,
        segments: &[ServerSegment]
    ) -> Result<Self, Error> {
        let directory =
            transfer.config.directory.join(".baad-dm-cache").join(source_identity(segments));
        let lock = lock_file(&directory.with_extension("lock"), &transfer.cancellation).await?;
        Ok(Self { directory, lock })
    }

    pub async fn clean(&self) {
        if let Err(error) = fs::remove_dir_all(&self.directory).await {
            warn!(path = %self.directory.display(), cause = %error, "Failed to clean segment cache");
        }
    }
}

fn source_identity(segments: &[ServerSegment]) -> String {
    let mut descriptor = String::new();
    for segment in segments {
        let url = segment.url.as_str();
        descriptor.push_str(url.split_once(['?', '#']).map_or(url, |(source, _)| source));
        descriptor.push('|');
        match segment.size {
            Some(size) => {
                let _ = write!(descriptor, "{size}");
            }
            None => descriptor.push('?')
        }
        descriptor.push('|');
        descriptor.push_str(segment.hash.as_deref().unwrap_or("?"));
        descriptor.push('\n');
    }
    let hash = sha::compute(descriptor.as_bytes());
    hash.iter().fold(String::with_capacity(hash.len() * 2), |mut identity, byte| {
        let _ = write!(identity, "{byte:02x}");
        identity
    })
}
