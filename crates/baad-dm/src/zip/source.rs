use std::io::Read;
use std::sync::{Arc, LazyLock};

use better_default::Default;
use bytes::Bytes;
use flate2::read::DeflateDecoder;
use futures::StreamExt;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use reqwest_middleware::reqwest::header::{ACCEPT_ENCODING, RANGE};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, SemaphorePermit};
use tokio::task;

use crate::client::{create_range_header, validate_range_response};
use crate::error::Error;
use crate::zip::types::{
    COMPRESSION_DEFLATE,
    COMPRESSION_STORED,
    LOCAL_HEADER_MIN_SIZE,
    ZipFileInfo
};

const DEFAULT_MAX_RANGE: u64 = 512 * 1024 * 1024;
const DEFAULT_MAX_MEMBER: u64 = 512 * 1024 * 1024;
const DEFAULT_MAX_DIRECTORY: u64 = 16 * 1024 * 1024;
const DEFAULT_BLOCKING_CONCURRENCY: usize = 2;

static BLOCKING: LazyLock<Arc<Semaphore>> =
    LazyLock::new(|| Arc::new(Semaphore::new(DEFAULT_BLOCKING_CONCURRENCY)));

#[derive(Clone, Copy, Debug, Default)]
pub struct ZipLimits {
    #[default(DEFAULT_MAX_RANGE)]
    pub max_range: u64,
    #[default(DEFAULT_MAX_MEMBER)]
    pub max_member: u64,
    #[default(DEFAULT_MAX_DIRECTORY)]
    pub max_directory: u64
}

#[derive(Clone, Copy)]
pub struct ZipSource<'a> {
    pub client: &'a ClientWithMiddleware,
    pub url: &'a Url,
    requests: Option<&'a Semaphore>,
    blocking: Option<&'a Arc<Semaphore>>,
    pub limits: ZipLimits,
    pub total_size: Option<u64>
}

impl<'a> ZipSource<'a> {
    pub fn new(
        client: &'a ClientWithMiddleware,
        url: &'a Url,
        requests: Option<&'a Semaphore>
    ) -> Self {
        Self {
            client,
            url,
            requests,
            blocking: None,
            total_size: None,
            limits: ZipLimits::default()
        }
    }

    pub const fn with_blocking_limiter(mut self, blocking: &'a Arc<Semaphore>) -> Self {
        self.blocking = Some(blocking);

        self
    }

    pub async fn extract_member(&self, info: &ZipFileInfo) -> Result<Bytes, Error> {
        if info.compressed_size > self.limits.max_range
            || info.uncompressed_size > self.limits.max_member
        {
            return Err(Error::ZipMemberTooLarge);
        }

        let permit = self.acquire_blocking_permit().await?;
        let header_end = info
            .local_header_offset
            .checked_add(LOCAL_HEADER_MIN_SIZE as u64 - 1)
            .ok_or(Error::ZipMemberOutOfBounds)?;
        let header = self.fetch_range(info.local_header_offset, header_end).await?;

        if header.len() < LOCAL_HEADER_MIN_SIZE || &header[..4] != b"PK\x03\x04" {
            return Err(Error::InvalidZipLocalHeader);
        }

        let filename_len = u16::from_le_bytes([header[26], header[27]]) as u64;
        let extra_len = u16::from_le_bytes([header[28], header[29]]) as u64;
        let data_start = header_end
            .checked_add(1)
            .and_then(|start| start.checked_add(filename_len))
            .and_then(|start| start.checked_add(extra_len))
            .ok_or(Error::ZipMemberOutOfBounds)?;
        let data_end =
            data_start.checked_add(info.compressed_size).ok_or(Error::ZipMemberOutOfBounds)?;

        if self.total_size.is_some_and(|total| data_end > total) {
            return Err(Error::ZipMemberOutOfBounds);
        }

        let compressed = if info.compressed_size == 0 {
            Bytes::new()
        } else {
            self.fetch_range(data_start, data_end - 1).await?
        };
        let info = info.clone();

        task::spawn_blocking(move || {
            let _permit = permit;

            Self::decompress(compressed, &info)
        })
        .await?
    }

    pub async fn acquire_permit(&self) -> Result<Option<SemaphorePermit<'_>>, Error> {
        match self.requests {
            Some(requests) => requests.acquire().await.map(Some).map_err(|_| Error::LimiterClosed),
            None => Ok(None)
        }
    }

    pub async fn acquire_blocking_permit(&self) -> Result<OwnedSemaphorePermit, Error> {
        Arc::clone(self.blocking.unwrap_or(&BLOCKING))
            .acquire_owned()
            .await
            .map_err(|_| Error::LimiterClosed)
    }

    pub async fn fetch_range(&self, start: u64, end: u64) -> Result<Bytes, Error> {
        let expected = end
            .checked_sub(start)
            .and_then(|size| size.checked_add(1))
            .filter(|size| *size <= self.limits.max_range)
            .ok_or(Error::ZipRangeTooLarge)?;
        let permit = self.acquire_permit().await?;
        let res = self
            .client
            .get(self.url.as_str())
            .header(ACCEPT_ENCODING, "identity")
            .header(RANGE, create_range_header(start, Some(end)))
            .send()
            .await?;

        res.error_for_status_ref()?;
        validate_range_response(&res, start, Some(end), self.total_size)?;
        let mut stream = res.bytes_stream();
        let mut bytes = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            if chunk.len() as u64 > expected - bytes.len() as u64 {
                return Err(Error::ZipRangeSizeMismatch);
            }

            bytes.extend_from_slice(&chunk);
        }

        if bytes.len() as u64 != expected {
            return Err(Error::ZipRangeSizeMismatch);
        }

        drop(permit);

        Ok(Bytes::from(bytes))
    }

    fn decompress(data: Bytes, info: &ZipFileInfo) -> Result<Bytes, Error> {
        let output = match info.compression_method {
            COMPRESSION_STORED => data,
            COMPRESSION_DEFLATE => {
                let limit =
                    info.uncompressed_size.checked_add(1).ok_or(Error::ZipExpandedSizeOverflow)?;
                let mut output = Vec::new();
                DeflateDecoder::new(data.as_ref()).take(limit).read_to_end(&mut output).map_err(
                    |e| Error::Archive(format!("Deflate decompression failed: {e}").into())
                )?;

                Bytes::from(output)
            }
            method => return Err(Error::UnsupportedCompression(method))
        };

        if output.len() as u64 != info.uncompressed_size {
            return Err(Error::ZipExpandedSizeMismatch);
        }

        Ok(output)
    }
}
