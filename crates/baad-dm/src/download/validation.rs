use crate::download::{Download, DownloadSource, validate_hash};
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
        validate_filename(&self.filename)?;
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
}

pub fn validate_filename(filename: &str) -> Result<(), Error> {
    let invalid_component = filename.split(['/', '\\']).any(|component| {
        component.is_empty()
            || component == "."
            || component == ".."
            || component.ends_with(['.', ' '])
    });

    if invalid_component || filename.contains([':', '\0']) {
        return Err(Error::InvalidOutputPath);
    }

    Ok(())
}
