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
    /// Expected content checksum. Opaque version identifiers (such as catalog
    /// `.hash` files) must not be passed here.
    pub hash: Option<String>,
    /// Algorithm for `hash`. When omitted, only MD5 hex and decimal CRC32
    /// are inferred.
    pub hash_type: Option<HashType>,
    pub target_file: Option<String>,
    pub size: Option<u64>
}

/// What a successful file check actually established.
#[derive(Debug, Clone, Copy)]
pub struct Verified {
    pub size: u64,
    pub size_checked: bool,
    pub checksum: Option<HashType>
}

impl Verified {
    /// Existing files are reused only when some property was checked.
    pub const fn reason(&self) -> Option<&'static str> {
        match (self.size_checked, self.checksum.is_some()) {
            (true, true) => Some("File exists with matching size and hash"),
            (false, true) => Some("File exists with matching hash"),
            (true, false) => Some("File exists with matching size"),
            (false, false) => None
        }
    }
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

    /// Fails when a hash is present but cannot be checked, so an unsupported
    /// value is never mistaken for verification.
    pub(crate) fn checksum(&self) -> Result<Option<Checksum>, Error> {
        let Some(hash) = self.hash.as_deref() else {
            return Ok(None);
        };
        Checksum::parse(hash, self.hash_type).map(Some).ok_or_else(|| {
            let kind = self.hash_type.map_or_else(|| "unrecognized".into(), |t| format!("{t:?}"));
            Error::DownloadFailed(format!("Cannot verify {kind} hash: {hash}").into())
        })
    }

    pub(crate) fn verify_file(
        &self,
        file_path: &Path,
        server_size: Option<u64>
    ) -> Result<Verified, Error> {
        let checksum = self.checksum()?;
        let size = file_path.metadata()?.len();
        let expected_size = self.size.or(server_size);
        if let Some(expected) = expected_size
            && size != expected
        {
            return Err(Error::DownloadFailed(
                format!("Size mismatch: expected {expected} bytes, got {size}").into()
            ));
        }
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
        Ok(Verified {
            size,
            size_checked: expected_size.is_some(),
            checksum: checksum.map(Checksum::hash_type)
        })
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
