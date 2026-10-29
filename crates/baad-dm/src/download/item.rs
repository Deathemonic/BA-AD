use std::error::Error as StdError;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;

use bon::Builder;
use derive_more::Debug;
use reqwest_middleware::reqwest::Url;

use self::download_builder::{IsUnset, SetSource, State};
use crate::download::{hash, validation};
use crate::error::Error;
use crate::segment::ServerSegment;

pub type TransformError = Box<dyn StdError + Send + Sync>;

#[derive(Debug, Clone)]
pub enum DownloadSource {
    Url(Url),
    Segments(Vec<ServerSegment>)
}

#[derive(Builder, Clone, Debug)]
pub struct Download {
    #[builder(setters(vis = ""))]
    pub source: DownloadSource,
    pub filename: String,
    pub hash: Option<String>,
    pub target_file: Option<String>,
    pub size: Option<u64>,
    #[debug(skip)]
    pub transform: Option<Arc<dyn OutputTransform>>
}

pub trait OutputTransform: Send + Sync {
    fn transform(&self, input: &mut dyn Read, output: &mut dyn Write)
    -> Result<(), TransformError>;
}

impl Download {
    pub const fn is_extraction(&self) -> bool { self.target_file.is_some() }

    pub const fn url(&self) -> Option<&Url> {
        match &self.source {
            DownloadSource::Url(url) => Some(url),
            DownloadSource::Segments(_) => None
        }
    }

    pub fn verify_hash(&self, file_path: &Path) -> Result<bool, Error> {
        hash::verify_hash(file_path, self.hash.as_ref())
    }
}

impl<S: State> DownloadBuilder<S>
where
    S::Source: IsUnset
{
    pub fn url(self, url: Url) -> DownloadBuilder<SetSource<S>> {
        self.source(DownloadSource::Url(url))
    }

    pub fn segments(self, segments: Vec<ServerSegment>) -> DownloadBuilder<SetSource<S>> {
        self.source(DownloadSource::Segments(segments))
    }
}

impl TryFrom<&str> for Download {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|error| Error::InvalidUrl {
            url: value.into(),
            reason: error.to_string().into()
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

fn extract_filename(url: &Url) -> Result<String, Error> {
    let filename = url
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|segment| !segment.is_empty())
        .ok_or_else(|| Error::InvalidUrl {
            url: url.as_str().into(),
            reason: "URL does not contain a filename".into()
        })?;

    let invalid = || Error::InvalidUrl {
        url: url.as_str().into(),
        reason: "URL filename is not a safe UTF-8 path component".into()
    };

    if filename.match_indices('%').any(|(offset, _)| {
        !filename
            .as_bytes()
            .get(offset + 1..offset + 3)
            .is_some_and(|digits| digits.iter().all(u8::is_ascii_hexdigit))
    }) {
        return Err(invalid());
    }

    let filename = percent_encoding::percent_decode_str(filename)
        .decode_utf8()
        .map_err(|_| invalid())?
        .into_owned();

    if filename.contains(['/', '\\']) {
        return Err(invalid());
    }

    validation::validate_filename(&filename)?;

    Ok(filename)
}
