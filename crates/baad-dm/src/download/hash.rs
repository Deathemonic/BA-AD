use std::fs::File;
use std::io::Read;
use std::path::Path;

use bacy::crypto::md5;
use bacy::error::HashError;
use bacy::hash::crc;
use md5_hasher::{Digest, Md5};

use crate::error::Error;

const HASH_BUFFER_SIZE: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Crc32
}

pub fn detect_hash_type(hash: &str) -> Option<HashType> {
    match hash.len() {
        32 if hash.chars().all(|c| c.is_ascii_hexdigit()) => Some(HashType::Md5),
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
            let calculated = md5::to_hex_string(&md5_streaming(file_path)?);
            Ok(calculated.eq_ignore_ascii_case(expected))
        }
        Some(HashType::Crc32) => {
            let Ok(expected_crc) = expected.parse::<u32>() else {
                return Ok(false);
            };
            match crc::compute_streaming(file_path, HASH_BUFFER_SIZE, None) {
                Ok(actual_crc) => Ok(actual_crc == expected_crc),
                Err(HashError::InvalidPath) => Ok(false),
                Err(e) => Err(e.into())
            }
        }
        None => Ok(false)
    }
}

fn md5_streaming(file_path: &Path) -> Result<[u8; 16], Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Md5::new();
    let mut buffer = vec![0; HASH_BUFFER_SIZE];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher.finalize().into())
}
