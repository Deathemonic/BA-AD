use std::path::Path;

use crate::download::checksum::Checksum;
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Crc32,
    /// Decimal CRC-64/XZ. Never inferred; callers must request it explicitly.
    Crc64Xz
}

/// Infers MD5 (32 hex digits) or CRC32 (decimal `u32`) from a hash string.
pub fn detect_hash_type(hash: &str) -> Option<HashType> {
    match hash.len() {
        32 if hash.chars().all(|c| c.is_ascii_hexdigit()) => Some(HashType::Md5),
        _ if hash.parse::<u32>().is_ok() => Some(HashType::Crc32),
        _ => None
    }
}

/// Returns `false` when the file is missing or the hash format is unsupported.
pub fn verify_hash(file_path: &Path, expected: Option<&String>) -> Result<bool, Error> {
    let Some(expected) = expected else {
        return Ok(true);
    };

    if !file_path.exists() {
        return Ok(false);
    }

    Checksum::parse(expected, None).map_or(Ok(false), |checksum| checksum.matches_file(file_path))
}
