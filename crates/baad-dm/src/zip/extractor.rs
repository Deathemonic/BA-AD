use std::collections::HashMap;

use bytes::Bytes;
use fastcat::fconcat;
use reqwest::header::ACCEPT_ENCODING;
use tokio::task;

use crate::error::Error;
use crate::zip::ZipSource;
use crate::zip::types::{
    CENTRAL_DIR_ENTRY_MIN_SIZE,
    CENTRAL_DIR_SIGNATURE,
    EOCD_MIN_SIZE,
    EOCD_SEARCH_SIZE,
    EOCD_SIGNATURE,
    ZipFileInfo
};

#[derive(Debug)]
pub struct ZipIndex {
    entries: HashMap<String, ZipFileInfo>,
    pub total_size: u64
}

pub struct ZipExtractor<'a> {
    source: ZipSource<'a>,
    zip_size: u64
}

struct EocdInfo {
    cd_size: u64,
    cd_offset: u64
}

impl ZipIndex {
    pub fn get(&self, name: &str) -> Option<&ZipFileInfo> { self.entries.get(name) }

    pub fn names(&self) -> impl Iterator<Item = &str> { self.entries.keys().map(String::as_str) }
}

impl<'a> ZipExtractor<'a> {
    pub async fn new(mut source: ZipSource<'a>) -> Result<Self, Error> {
        let permit = source.acquire_permit().await?;
        let head = source
            .client
            .head(source.url.clone())
            .header(ACCEPT_ENCODING, "identity")
            .send()
            .await?;
        head.error_for_status_ref()?;
        let headers = head.headers();

        let zip_size = headers
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
            .ok_or(Error::UnknownZipSize)?;

        let accepts_ranges = headers
            .get("accept-ranges")
            .and_then(|h| h.to_str().ok())
            .is_some_and(|v| v.contains("bytes"));

        if !accepts_ranges {
            return Err(Error::RangeNotSupported);
        }

        drop(permit);

        if zip_size < EOCD_MIN_SIZE as u64 {
            return Err(Error::InvalidZipEocd);
        }

        source.total_size = Some(zip_size);

        Ok(Self { source, zip_size })
    }

    pub async fn extract_file(&self, target: &str) -> Result<Bytes, Error> {
        let index = self.build_index().await?;
        let info = index.get(target).ok_or_else(|| {
            Error::Archive(fconcat!("File '", target, "' not found in ZIP").into())
        })?;
        self.source.extract_member(info).await
    }

    pub async fn build_index(&self) -> Result<ZipIndex, Error> {
        let eocd = self.read_eocd().await?;
        if eocd.cd_size > self.source.limits.max_directory {
            return Err(Error::ZipDirectoryTooLarge);
        }

        let end = eocd
            .cd_offset
            .checked_add(eocd.cd_size)
            .filter(|end| *end <= self.zip_size)
            .ok_or(Error::ZipDirectoryOutOfBounds)?;

        if eocd.cd_size == 0 {
            return Ok(ZipIndex {
                entries: HashMap::new(),
                total_size: self.zip_size
            });
        }

        let permit = self.source.acquire_blocking_permit().await?;
        let cd_data = self.source.fetch_range(eocd.cd_offset, end - 1).await?;

        let total_size = self.zip_size;

        task::spawn_blocking(move || {
            let _permit = permit;

            Ok(ZipIndex {
                entries: Self::parse_central_directory(&cd_data)?,
                total_size
            })
        })
        .await?
    }

    async fn read_eocd(&self) -> Result<EocdInfo, Error> {
        let search_size = self.zip_size.min(EOCD_SEARCH_SIZE);
        let start = self.zip_size - search_size;

        let data = self.source.fetch_range(start, self.zip_size - 1).await?;

        let offset =
            data.windows(4).rposition(|w| w == EOCD_SIGNATURE).ok_or(Error::InvalidZipEocd)?;

        let eocd = &data[offset..];
        if eocd.len() < EOCD_MIN_SIZE {
            return Err(Error::InvalidZipEocd);
        }

        let comment_size = read_u16_le(eocd, 20) as usize;

        if eocd.len() != EOCD_MIN_SIZE + comment_size {
            return Err(Error::InvalidZipEocd);
        }

        Ok(EocdInfo {
            cd_size: read_u32_le(eocd, 12) as u64,
            cd_offset: read_u32_le(eocd, 16) as u64
        })
    }

    fn parse_central_directory(cd: &[u8]) -> Result<HashMap<String, ZipFileInfo>, Error> {
        let mut entries = HashMap::new();
        let mut offset = 0;

        while offset + CENTRAL_DIR_ENTRY_MIN_SIZE <= cd.len() {
            if &cd[offset..offset + 4] != CENTRAL_DIR_SIGNATURE {
                return Err(Error::InvalidZipDirectory);
            }

            let filename_len = read_u16_le(cd, offset + 28) as usize;
            let extra_len = read_u16_le(cd, offset + 30) as usize;
            let comment_len = read_u16_le(cd, offset + 32) as usize;

            let name_start = offset + CENTRAL_DIR_ENTRY_MIN_SIZE;
            let entry_end = name_start + filename_len + extra_len + comment_len;

            if entry_end > cd.len() {
                return Err(Error::InvalidZipDirectory);
            }

            let filename = String::from_utf8_lossy(&cd[name_start..name_start + filename_len]);

            entries.insert(filename.into_owned(), ZipFileInfo {
                compression_method: read_u16_le(cd, offset + 10),
                compressed_size: read_u32_le(cd, offset + 20) as u64,
                uncompressed_size: read_u32_le(cd, offset + 24) as u64,
                local_header_offset: read_u32_le(cd, offset + 42) as u64
            });

            offset = entry_end;
        }

        if offset != cd.len() {
            return Err(Error::InvalidZipDirectory);
        }

        Ok(entries)
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
