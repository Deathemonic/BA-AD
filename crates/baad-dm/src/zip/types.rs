pub const EOCD_SIGNATURE: &[u8; 4] = b"\x50\x4b\x05\x06";
pub const CENTRAL_DIR_SIGNATURE: &[u8; 4] = b"\x50\x4b\x01\x02";

pub const COMPRESSION_STORED: u16 = 0;
pub const COMPRESSION_DEFLATE: u16 = 8;

pub const EOCD_MIN_SIZE: usize = 22;
pub const CENTRAL_DIR_ENTRY_MIN_SIZE: usize = 46;
pub const LOCAL_HEADER_MIN_SIZE: usize = 30;
pub const EOCD_SEARCH_SIZE: u64 = 65536;

#[derive(Debug, Clone)]
pub struct ZipFileInfo {
    pub compression_method: u16,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    pub local_header_offset: u64
}
