use std::path::Path;

use bacy::crypto::md5;
use bacy::error::HashError;
use bacy::hash::{crc, crc64, sha};

use crate::error::Error;

const HASH_BUFFER_SIZE: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Sha1,
    Crc32(u32),
    Crc64(u64)
}

pub fn detect_hash_type(hash: &str) -> Option<HashType> {
    match hash.len() {
        32 if hash.chars().all(|c| c.is_ascii_hexdigit()) => Some(HashType::Md5),
        40 if hash.chars().all(|c| c.is_ascii_hexdigit()) => Some(HashType::Sha1),
        _ => hash
            .parse::<u32>()
            .map(HashType::Crc32)
            .or_else(|_| hash.parse::<u64>().map(HashType::Crc64))
            .ok()
    }
}

pub fn validate_hash(hash: Option<&str>) -> Result<(), Error> {
    if let Some(hash) = hash
        && detect_hash_type(hash).is_none()
    {
        return Err(Error::UnsupportedHash(hash.into()));
    }

    Ok(())
}

pub fn verify_hash(file_path: &Path, expected: Option<&String>) -> Result<bool, Error> {
    let Some(expected) = expected else {
        return Ok(true);
    };
    if !file_path.exists() {
        return Ok(false);
    }

    match detect_hash_type(expected) {
        Some(HashType::Md5) => match md5::compute_streaming(file_path, HASH_BUFFER_SIZE) {
            Ok(calculated) => Ok(calculated.eq_ignore_ascii_case(expected)),
            Err(HashError::InvalidPath) => Ok(false),
            Err(error) => Err(error.into())
        },
        Some(HashType::Sha1) => match sha::compute_streaming(file_path, HASH_BUFFER_SIZE) {
            Ok(calculated) => Ok(calculated.eq_ignore_ascii_case(expected)),
            Err(HashError::InvalidPath) => Ok(false),
            Err(error) => Err(error.into())
        },
        Some(HashType::Crc32(crc)) => match crc::compute_streaming(file_path, HASH_BUFFER_SIZE) {
            Ok(calculated) => Ok(calculated == crc),
            Err(HashError::InvalidPath) => Ok(false),
            Err(error) => Err(error.into())
        },
        Some(HashType::Crc64(crc)) => match crc64::compute_streaming(file_path, HASH_BUFFER_SIZE) {
            Ok(calculated) => Ok(calculated == crc),
            Err(HashError::InvalidPath) => Ok(false),
            Err(error) => Err(error.into())
        },
        None => Ok(false)
    }
}
