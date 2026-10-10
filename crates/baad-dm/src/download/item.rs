use std::path::Path;

use bon::Builder;
use reqwest_middleware::reqwest::Url;

use crate::download::checksum::Checksum;
use crate::download::hash::HashType;
use crate::error::Error;

#[derive(Debug, Clone, Builder)]
pub struct Download {
    pub url: Url,
    pub filename: String,
    pub hash: Option<String>,
    /// Algorithm for `hash`. When omitted, only MD5 hex and decimal CRC32
    /// are inferred.
    pub hash_type: Option<HashType>,
    pub target_file: Option<String>,
    pub size: Option<u64>
}

impl Download {
    pub const fn is_extraction(&self) -> bool { self.target_file.is_some() }

    /// Returns `false` when the hash is unsupported or does not match.
    pub fn verify_hash(&self, file_path: &Path) -> Result<bool, Error> {
        let Some(hash) = self.hash.as_deref() else {
            return Ok(true);
        };
        if !file_path.exists() {
            return Ok(false);
        }
        Checksum::parse(hash, self.hash_type)
            .map_or(Ok(false), |checksum| checksum.matches_file(file_path))
    }

    pub(crate) fn verify_file(
        &self,
        file_path: &Path,
        server_size: Option<u64>
    ) -> Result<u64, Error> {
        let size = file_path.metadata()?.len();
        if let Some(expected) = self.size.or(server_size)
            && size != expected
        {
            return Err(Error::DownloadFailed(
                format!("Size mismatch: expected {expected} bytes, got {size}").into()
            ));
        }
        // Hashes in an unrecognized format cannot be checked; the size check
        // above still applies.
        let checksum = self.hash.as_deref().and_then(|hash| Checksum::parse(hash, self.hash_type));
        if let Some(checksum) = checksum
            && !checksum.matches_file(file_path)?
        {
            return Err(Error::DownloadFailed(
                format!(
                    "File does not match expected {:?} hash: {}",
                    checksum.hash_type(),
                    self.hash.as_deref().unwrap_or_default()
                )
                .into()
            ));
        }
        Ok(size)
    }
}

fn extract_filename(url: &Url) -> Result<String, Error> {
    let filename = url
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::InvalidUrl {
            url: url.as_str().into(),
            reason: "URL does not contain a filename".into()
        })?;

    Ok(form_urlencoded::parse(filename.as_bytes()).map(|(key, val)| [key, val].concat()).collect())
}

impl TryFrom<&str> for Download {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|e| Error::InvalidUrl {
            url: value.into(),
            reason: e.to_string().into()
        })?;

        Self::try_from(&url)
    }
}

impl TryFrom<&Url> for Download {
    type Error = Error;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        let filename = extract_filename(url)?;

        Ok(Self::builder().url(url.clone()).filename(filename).build())
    }
}
