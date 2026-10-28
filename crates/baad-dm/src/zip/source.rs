use std::io::Read;

use bytes::Bytes;
use flate2::read::DeflateDecoder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use reqwest_middleware::reqwest::header::RANGE;
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::client::create_range_header;
use crate::error::Error;
use crate::zip::types::{
    COMPRESSION_DEFLATE,
    COMPRESSION_STORED,
    LOCAL_HEADER_MIN_SIZE,
    ZipFileInfo
};

#[derive(Clone, Copy)]
pub struct ZipSource<'a> {
    pub client: &'a ClientWithMiddleware,
    pub url: &'a Url,
    requests: Option<&'a Semaphore>
}

impl<'a> ZipSource<'a> {
    pub const fn new(
        client: &'a ClientWithMiddleware,
        url: &'a Url,
        requests: Option<&'a Semaphore>
    ) -> Self {
        Self { client, url, requests }
    }

    pub async fn extract_member(&self, info: &ZipFileInfo) -> Result<Bytes, Error> {
        let header = self
            .fetch_range(
                info.local_header_offset,
                info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 - 1
            )
            .await?;

        if header.len() < LOCAL_HEADER_MIN_SIZE {
            return Err(Error::Archive("Invalid local file header".into()));
        }

        let filename_len = u16::from_le_bytes([header[26], header[27]]) as u64;
        let extra_len = u16::from_le_bytes([header[28], header[29]]) as u64;
        let data_start =
            info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 + filename_len + extra_len;
        let data_end = data_start + info.compressed_size - 1;

        let compressed = self.fetch_range(data_start, data_end).await?;
        Self::decompress(&compressed, info.compression_method)
    }

    pub async fn acquire_permit(&self) -> Result<Option<SemaphorePermit<'_>>, Error> {
        match self.requests {
            Some(requests) => requests.acquire().await.map(Some).map_err(|_| Error::LimiterClosed),
            None => Ok(None)
        }
    }

    pub async fn fetch_range(&self, start: u64, end: u64) -> Result<Bytes, Error> {
        let permit = self.acquire_permit().await?;
        let res = self
            .client
            .get(self.url.as_str())
            .header(RANGE, create_range_header(start, Some(end)))
            .send()
            .await?;

        res.error_for_status_ref()?;
        let bytes = res.bytes().await?;
        drop(permit);
        Ok(bytes)
    }

    fn decompress(data: &[u8], method: u16) -> Result<Bytes, Error> {
        match method {
            COMPRESSION_STORED => Ok(Bytes::copy_from_slice(data)),
            COMPRESSION_DEFLATE => {
                let mut output = Vec::with_capacity(data.len() * 2);
                DeflateDecoder::new(data).read_to_end(&mut output).map_err(|e| {
                    Error::Archive(format!("Deflate decompression failed: {e}").into())
                })?;
                Ok(Bytes::from(output))
            }
            _ => Err(Error::UnsupportedCompression(method))
        }
    }
}
