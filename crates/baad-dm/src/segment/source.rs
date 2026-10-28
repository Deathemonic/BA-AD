use reqwest_middleware::reqwest::Url;

use crate::download::{Download, validate_hash};
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct ServerSegment {
    pub url: Url,
    pub size: Option<u64>,
    pub hash: Option<String>
}

pub fn validate_segments(download: &Download, segments: &[ServerSegment]) -> Result<(), Error> {
    if segments.is_empty() {
        return Err(Error::EmptySegments);
    }

    if download.target_file.is_some() {
        return Err(Error::ZipSegmentsNotSupported);
    }

    segments.iter().try_fold(0_u64, |total, segment| {
        validate_hash(segment.hash.as_ref())?;
        segment
            .size
            .map_or(Ok(total), |size| total.checked_add(size).ok_or(Error::SegmentSizeOverflow))
    })?;
    Ok(())
}

pub fn network_total(segments: &[ServerSegment]) -> Option<u64> {
    segments.iter().try_fold(0_u64, |total, segment| total.checked_add(segment.size?))
}

pub fn progress_limits(segments: &[ServerSegment]) -> Vec<Option<u64>> {
    segments.iter().map(|segment| segment.size).collect()
}
