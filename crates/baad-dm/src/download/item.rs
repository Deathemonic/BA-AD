use crate::error::Error;

use std::path::Path;

use reqwest_middleware::reqwest::Url;

#[derive(Debug, Clone)]
pub struct Download {
    pub url: Url,
    pub filename: String,
    pub hash: Option<String>,
    pub target_file: Option<String>,
}

impl Download {
    pub fn new(url: Url, filename: impl Into<String>) -> Self {
        Self {
            url,
            filename: filename.into(),
            hash: None,
            target_file: None,
        }
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn with_target_file(mut self, target: impl Into<String>) -> Self {
        self.target_file = Some(target.into());
        self
    }

    pub fn is_extraction(&self) -> bool {
        self.target_file.is_some()
    }

    pub fn verify_hash(&self, file_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(super::hash::verify_hash(file_path, self.hash.as_ref())?)
    }
}

impl TryFrom<&str> for Download {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|e| Error::InvalidUrl {
            url: value.into(),
            reason: e.to_string().into(),
        })?;

        let filename = url
            .path_segments()
            .and_then(|segments| segments.last())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Error::InvalidUrl {
                url: value.into(),
                reason: "URL does not contain a filename".into(),
            })?;

        let decoded_filename: String = form_urlencoded::parse(filename.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();

        Ok(Download {
            url,
            filename: decoded_filename,
            hash: None,
            target_file: None,
        })
    }
}

impl TryFrom<&Url> for Download {
    type Error = Error;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        let filename = url
            .path_segments()
            .and_then(|segments| segments.last())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Error::InvalidUrl {
                url: url.as_str().into(),
                reason: "URL does not contain a filename".into(),
            })?;

        let decoded_filename: String = form_urlencoded::parse(filename.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();

        Ok(Download {
            url: url.clone(),
            filename: decoded_filename,
            hash: None,
            target_file: None,
        })
    }
}
