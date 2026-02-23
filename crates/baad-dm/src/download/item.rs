use std::path::Path;

use bon::Builder;
use reqwest_middleware::reqwest::Url;

use crate::download::hash::verify_hash;
use crate::error::Error;

#[derive(Debug, Clone, Builder)]
pub struct Download {
    pub url: Url,
    pub filename: String,
    pub hash: Option<String>,
    pub target_file: Option<String>
}

impl Download {
    pub fn is_extraction(&self) -> bool { self.target_file.is_some() }

    pub fn verify_hash(&self, file_path: &Path) -> Result<bool, Error> {
        verify_hash(file_path, self.hash.as_ref())
    }
}

impl TryFrom<&str> for Download {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|e| Error::InvalidUrl {
            url: value.into(),
            reason: e.to_string().into()
        })?;

        let filename = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Error::InvalidUrl {
                url: value.into(),
                reason: "URL does not contain a filename".into()
            })?;

        let decoded_filename: String = form_urlencoded::parse(filename.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();

        Ok(Download::builder().url(url).filename(decoded_filename).build())
    }
}

impl TryFrom<&Url> for Download {
    type Error = Error;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        let filename = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Error::InvalidUrl {
                url: url.as_str().into(),
                reason: "URL does not contain a filename".into()
            })?;

        let decoded_filename: String = form_urlencoded::parse(filename.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();

        Ok(Download::builder().url(url.clone()).filename(decoded_filename).build())
    }
}
