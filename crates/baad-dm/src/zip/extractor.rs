use std::collections::HashMap;
use std::io::Read;

use flate2::read::DeflateDecoder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use reqwest_middleware::reqwest::header::RANGE;

use crate::client::create_range_header;
use crate::error::Error;
use crate::zip::types::*;

struct EocdInfo {
    cd_size: u64,
    cd_offset: u64
}

#[derive(Debug)]
pub struct ZipIndex {
    entries: HashMap<String, ZipFileInfo>
}

impl ZipIndex {
    pub fn get(&self, name: &str) -> Option<&ZipFileInfo> { self.entries.get(name) }

    pub fn names(&self) -> impl Iterator<Item = &str> { self.entries.keys().map(String::as_str) }
}

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
            .ok_or_else(|| Error::Archive("Could not determine ZIP file size".into()))?;

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
        let index = self.build_index().await?;
        let info = index
            .get(target)
            .ok_or_else(|| Error::Archive(format!("File '{target}' not found in ZIP").into()))?;
        Self::extract_member(self.client, self.url, info).await
    }

    pub async fn build_index(&self) -> Result<ZipIndex, Error> {
        let eocd = self.read_eocd().await?;
        let cd_data = Self::fetch_range(
            self.client,
            self.url,
            eocd.cd_offset,
            eocd.cd_offset + eocd.cd_size - 1
        )
        .await?;

        Ok(ZipIndex {
            entries: Self::parse_central_directory(&cd_data)
        })
    }

    pub async fn extract_member(
        client: &ClientWithMiddleware,
        url: &Url,
        info: &ZipFileInfo
    ) -> Result<Vec<u8>, Error> {
        let header = Self::fetch_range(
            client,
            url,
            info.local_header_offset,
            info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 - 1
        )
        .await?;

        if header.len() < LOCAL_HEADER_MIN_SIZE {
            return Err(Error::Archive("Invalid local file header".into()));
        }

        let filename_len = read_u16_le(&header, 26) as u64;
        let extra_len = read_u16_le(&header, 28) as u64;
        let data_start =
            info.local_header_offset + LOCAL_HEADER_MIN_SIZE as u64 + filename_len + extra_len;
        let data_end = data_start + info.compressed_size - 1;

        let compressed = Self::fetch_range(client, url, data_start, data_end).await?;
        Self::decompress(&compressed, info.compression_method)
    }

    async fn read_eocd(&self) -> Result<EocdInfo, Error> {
        let search_size = self.zip_size.min(EOCD_SEARCH_SIZE);
        let start = self.zip_size - search_size;

        let data = Self::fetch_range(self.client, self.url, start, self.zip_size - 1).await?;

        let offset = data
            .windows(4)
            .rposition(|w| w == EOCD_SIGNATURE)
            .ok_or_else(|| Error::Archive("Could not find End of Central Directory".into()))?;

        let eocd = &data[offset..];
        if eocd.len() < EOCD_MIN_SIZE {
            return Err(Error::Archive("Invalid EOCD record".into()));
        }

        Ok(EocdInfo {
            cd_size: read_u32_le(eocd, 12) as u64,
            cd_offset: read_u32_le(eocd, 16) as u64
        })
    }

    fn parse_central_directory(cd: &[u8]) -> HashMap<String, ZipFileInfo> {
        let mut entries = HashMap::new();
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

            entries.insert(filename.into_owned(), ZipFileInfo {
                compression_method: read_u16_le(cd, offset + 10),
                compressed_size: read_u32_le(cd, offset + 20) as u64,
                uncompressed_size: read_u32_le(cd, offset + 24) as u64,
                local_header_offset: read_u32_le(cd, offset + 42) as u64
            });

            offset += CENTRAL_DIR_ENTRY_MIN_SIZE + filename_len + extra_len + comment_len;
        }

        entries
    }

    async fn fetch_range(
        client: &ClientWithMiddleware,
        url: &Url,
        start: u64,
        end: u64
    ) -> Result<Vec<u8>, Error> {
        let res = client
            .get(url.as_str())
            .header(RANGE, create_range_header(start, Some(end)))
            .send()
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    fn decompress(data: &[u8], method: u16) -> Result<Vec<u8>, Error> {
        match method {
            COMPRESSION_STORED => Ok(data.to_vec()),
            COMPRESSION_DEFLATE => {
                let mut output = Vec::with_capacity(data.len() * 2);
                DeflateDecoder::new(data).read_to_end(&mut output).map_err(|e| {
                    Error::Archive(format!("Deflate decompression failed: {e}").into())
                })?;
                Ok(output)
            }
            _ => Err(Error::UnsupportedCompression(method))
        }
    }
}

#[inline]
const fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

#[inline]
const fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}
