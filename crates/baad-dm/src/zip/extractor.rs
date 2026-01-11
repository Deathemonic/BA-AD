use std::io::Read;

use flate2::read::DeflateDecoder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;

use super::types::*;
use crate::error::Error;

pub struct ZipExtractor<'a> {
    client: &'a ClientWithMiddleware,
    url: &'a Url,
    zip_size: u64
}

impl<'a> ZipExtractor<'a> {
    pub async fn new(client: &'a ClientWithMiddleware, url: &'a Url) -> Result<Self, Error> {
        let head = client.head(url.clone()).send().await?;
        let headers = head.headers();

        let zip_size = headers
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| Error::Archive {
                message: "Could not determine ZIP file size".into(),
                source: None
            })?;

        let accepts_ranges = headers
            .get("accept-ranges")
            .and_then(|h| h.to_str().ok())
            .is_some_and(|v| v.contains("bytes"));

        if !accepts_ranges {
            return Err(Error::RangeNotSupported);
        }

        Ok(Self { client, url, zip_size })
    }

    pub async fn extract_file(&self, target: &str) -> Result<Vec<u8>, Error> {
        let eocd = self.read_eocd().await?;
        let file_info = self.find_file(&eocd, target).await?;
        let compressed = self.read_file_data(&file_info).await?;
        self.decompress(&compressed, file_info.compression_method)
    }

    async fn read_eocd(&self) -> Result<EocdInfo, Error> {
        let search_size = self.zip_size.min(EOCD_SEARCH_SIZE);
        let start = self.zip_size - search_size;

        let data = self.fetch_range(start, self.zip_size - 1).await?;

        let offset =
            data.windows(4).rposition(|w| w == EOCD_SIGNATURE).ok_or_else(|| Error::Archive {
                message: "Could not find End of Central Directory".into(),
                source: None
            })?;

        let eocd = &data[offset..];
        if eocd.len() < EOCD_MIN_SIZE {
            return Err(Error::Archive {
                message: "Invalid EOCD record".into(),
                source: None
            });
        }

        Ok(EocdInfo {
            cd_size: read_u32_le(eocd, 12) as u64,
            cd_offset: read_u32_le(eocd, 16) as u64
        })
    }

    async fn find_file(&self, eocd: &EocdInfo, target: &str) -> Result<ZipFileInfo, Error> {
        let cd_data = self.fetch_range(eocd.cd_offset, eocd.cd_offset + eocd.cd_size - 1).await?;

        self.parse_central_directory(&cd_data, target)?.ok_or_else(|| Error::Archive {
            message: format!("File '{}' not found in ZIP", target).into(),
            source: None
        })
    }

    async fn read_file_data(&self, info: &ZipFileInfo) -> Result<Vec<u8>, Error> {
        let header = self
            .fetch_range(
                info.local_header_offset,
                info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 - 1
            )
            .await?;

        if header.len() < LOCAL_HEADER_MIN_SIZE {
            return Err(Error::Archive {
                message: "Invalid local file header".into(),
                source: None
            });
        }

        let filename_len = read_u16_le(&header, 26) as u64;
        let extra_len = read_u16_le(&header, 28) as u64;
        let data_start =
            info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 + filename_len + extra_len;
        let data_end = data_start + info.compressed_size - 1;

        self.fetch_range(data_start, data_end).await
    }

    fn parse_central_directory(
        &self,
        cd: &[u8],
        target: &str
    ) -> Result<Option<ZipFileInfo>, Error> {
        let mut offset = 0;

        while offset + CENTRAL_DIR_ENTRY_MIN_SIZE <= cd.len() {
            if &cd[offset..offset + 4] != CENTRAL_DIR_SIGNATURE {
                break;
            }

            let filename_len = read_u16_le(cd, offset + 28) as usize;
            let extra_len = read_u16_le(cd, offset + 30) as usize;
            let comment_len = read_u16_le(cd, offset + 32) as usize;

            let name_start = offset + CENTRAL_DIR_ENTRY_MIN_SIZE;
            if name_start + filename_len > cd.len() {
                break;
            }

            let filename = String::from_utf8_lossy(&cd[name_start..name_start + filename_len]);

            if filename == target {
                return Ok(Some(ZipFileInfo {
                    compression_method: read_u16_le(cd, offset + 10),
                    compressed_size: read_u32_le(cd, offset + 20) as u64,
                    uncompressed_size: read_u32_le(cd, offset + 24) as u64,
                    local_header_offset: read_u32_le(cd, offset + 42) as u64
                }));
            }

            offset += CENTRAL_DIR_ENTRY_MIN_SIZE + filename_len + extra_len + comment_len;
        }

        Ok(None)
    }

    async fn fetch_range(&self, start: u64, end: u64) -> Result<Vec<u8>, Error> {
        let res = self
            .client
            .get(self.url.as_str())
            .header("Range", format!("bytes={}-{}", start, end))
            .send()
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    fn decompress(&self, data: &[u8], method: u16) -> Result<Vec<u8>, Error> {
        match method {
            COMPRESSION_STORED => Ok(data.to_vec()),
            COMPRESSION_DEFLATE => {
                let mut output = Vec::with_capacity(data.len() * 2);
                DeflateDecoder::new(data).read_to_end(&mut output).map_err(|e| Error::Archive {
                    message: "Deflate decompression failed".into(),
                    source: Some(Box::new(e))
                })?;
                Ok(output)
            }
            _ => Err(Error::UnsupportedCompression(method))
        }
    }
}

struct EocdInfo {
    cd_size: u64,
    cd_offset: u64
}

#[inline]
fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

#[inline]
fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}
