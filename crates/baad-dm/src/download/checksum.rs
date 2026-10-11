use std::fs::File;
use std::io::Read;
use std::path::Path;

use md5_hasher::{Digest, Md5};

use crate::download::hash::{HashType, detect_hash_type};
use crate::error::Error;

const HASH_BUFFER_SIZE: usize = 256 * 1024;

/// A content checksum that can actually be computed for a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Checksum {
    Md5([u8; 16]),
    Crc32(u32),
    Crc64Xz(u64)
}

impl Checksum {
    /// Parses `value` as `hash_type`. Without an explicit type only MD5 hex and
    /// decimal CRC32 are inferred; CRC64 values are never guessed from shape.
    pub fn parse(value: &str, hash_type: Option<HashType>) -> Option<Self> {
        match hash_type.or_else(|| detect_hash_type(value))? {
            HashType::Md5 => parse_md5(value).map(Self::Md5),
            HashType::Crc32 => value.parse().ok().map(Self::Crc32),
            HashType::Crc64Xz => value.parse().ok().map(Self::Crc64Xz)
        }
    }

    pub const fn hash_type(self) -> HashType {
        match self {
            Self::Md5(_) => HashType::Md5,
            Self::Crc32(_) => HashType::Crc32,
            Self::Crc64Xz(_) => HashType::Crc64Xz
        }
    }

    /// Streams the file through the hasher instead of reading it into memory.
    pub fn matches_file(self, path: &Path) -> Result<bool, Error> {
        let mut file = File::open(path)?;
        let mut buffer = vec![0; HASH_BUFFER_SIZE];
        let mut hasher = Hasher::new(self);
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        Ok(hasher.finish() == self)
    }
}

enum Hasher {
    Md5(Box<Md5>),
    Crc32(crc32fast::Hasher),
    Crc64Xz(Crc64Xz)
}

impl Hasher {
    fn new(checksum: Checksum) -> Self {
        match checksum {
            Checksum::Md5(_) => Self::Md5(Box::default()),
            Checksum::Crc32(_) => Self::Crc32(crc32fast::Hasher::new()),
            Checksum::Crc64Xz(_) => Self::Crc64Xz(Crc64Xz::new())
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Md5(hasher) => hasher.update(bytes),
            Self::Crc32(hasher) => hasher.update(bytes),
            Self::Crc64Xz(hasher) => hasher.update(bytes)
        }
    }

    fn finish(self) -> Checksum {
        match self {
            Self::Md5(hasher) => Checksum::Md5(hasher.finalize().into()),
            Self::Crc32(hasher) => Checksum::Crc32(hasher.finalize()),
            Self::Crc64Xz(hasher) => Checksum::Crc64Xz(hasher.finish())
        }
    }
}

fn parse_md5(value: &str) -> Option<[u8; 16]> {
    if value.len() != 32 || !value.is_ascii() {
        return None;
    }
    let mut digest = [0; 16];
    let (pairs, _) = value.as_bytes().as_chunks::<2>();
    for (byte, pair) in digest.iter_mut().zip(pairs) {
        *byte = u8::from_str_radix(str::from_utf8(pair).ok()?, 16).ok()?;
    }
    Some(digest)
}

/// CRC-64/XZ (ECMA-182 polynomial, reflected, initial and final XOR all ones),
/// as used by the Japan launcher manifest.
pub struct Crc64Xz(u64);

const CRC64_XZ_TABLE: [u64; 256] = {
    let mut table = [0; 256];
    let mut index = 0;
    while index < 256 {
        let mut crc = index as u64;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 == 0 { crc >> 1 } else { (crc >> 1) ^ 0xC96C_5795_D787_0F42 };
            bit += 1;
        }
        table[index] = crc;
        index += 1;
    }
    table
};

impl Crc64Xz {
    pub const fn new() -> Self { Self(u64::MAX) }

    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = CRC64_XZ_TABLE[((self.0 ^ u64::from(byte)) & 0xFF) as usize] ^ (self.0 >> 8);
        }
    }

    pub const fn finish(&self) -> u64 { !self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crc64_xz(bytes: &[u8]) -> u64 {
        let mut hasher = Crc64Xz::new();
        hasher.update(bytes);
        hasher.finish()
    }

    #[test]
    fn crc64_xz_matches_reference_vectors() {
        // Check value from the CRC catalogue for CRC-64/XZ.
        assert_eq!(crc64_xz(b"123456789"), 0x995D_C9BB_DF19_39FA);
        assert_eq!(crc64_xz(b""), 0);
        let mut split = Crc64Xz::new();
        split.update(b"1234");
        split.update(b"56789");
        assert_eq!(split.finish(), 0x995D_C9BB_DF19_39FA);
    }

    #[test]
    fn crc64_is_only_used_when_requested() {
        let value = "5484324475028437734";
        assert_eq!(Checksum::parse(value, None), None);
        assert_eq!(
            Checksum::parse(value, Some(HashType::Crc64Xz)),
            Some(Checksum::Crc64Xz(5_484_324_475_028_437_734))
        );
        assert_eq!(Checksum::parse(value, Some(HashType::Crc32)), None);
    }

    #[test]
    fn explicit_types_are_parsed_strictly() {
        assert_eq!(Checksum::parse("not-a-digest", Some(HashType::Md5)), None);
        assert_eq!(Checksum::parse("-1", Some(HashType::Crc32)), None);
        assert_eq!(Checksum::parse("123", Some(HashType::Md5)), None);
        assert!(matches!(
            Checksum::parse("D41D8CD98F00B204E9800998ECF8427E", Some(HashType::Md5)),
            Some(Checksum::Md5(_))
        ));
    }
}
