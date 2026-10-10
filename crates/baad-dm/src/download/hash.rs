use std::fs;
use std::path::Path;

use bacy::crypto::md5;
use bacy::error::HashError;
use bacy::hash::{crc, sha};

use crate::error::Error;

const HASH_BUFFER_SIZE: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Sha1,
    Crc32
}

pub fn detect_hash_type(hash: &str) -> Option<HashType> {
    match hash.len() {
        32 if hash.chars().all(|character| character.is_ascii_hexdigit()) => Some(HashType::Md5),
        40 if hash.chars().all(|character| character.is_ascii_hexdigit()) => Some(HashType::Sha1),
        _ if hash.parse::<u32>().is_ok() => Some(HashType::Crc32),
        _ => None
    }
}

pub fn verify_hash(file_path: &Path, expected: Option<&String>) -> Result<bool, Error> {
    let Some(expected) = expected else {
        return Ok(true);
    };
    if !file_path.exists() {
        return Ok(false);
    }

    match detect_hash_type(expected) {
        Some(HashType::Md5) => {
            let data = fs::read(file_path)?;
            let hash_bytes = md5::compute_hash(&data);
            let calculated = md5::to_hex_string(&hash_bytes);
            Ok(calculated.eq_ignore_ascii_case(expected))
        }
        Some(HashType::Sha1) => match sha::compute_sha1_streaming(file_path, HASH_BUFFER_SIZE) {
            Ok(calculated) => Ok(calculated.eq_ignore_ascii_case(expected)),
            Err(HashError::InvalidPath) => Ok(false),
            Err(error) => Err(error.into())
        },
        Some(HashType::Crc32) => {
            let Ok(expected_crc) = expected.parse::<u32>() else {
                return Ok(false);
            };

            match crc::compute_streaming(file_path, HASH_BUFFER_SIZE, None) {
                Ok(actual_crc) => Ok(actual_crc == expected_crc),
                Err(HashError::InvalidPath) => Ok(false),
                Err(error) => Err(error.into())
            }
        }
        None => Ok(false)
    }
}
