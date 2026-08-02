//! C ABI for `baad-dm`. Fallible functions return `i32` (`0` success;
//! `-2` null, `-3` invalid argument, `-4` runtime unavailable); owned values
//! are freed with their `baad_dm_*_free` pair. Download/zip calls block on an
//! internal tokio runtime; generated plumbing lives in `c_shadow.rs`.

#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::c_char;
use std::path::PathBuf;
use std::ptr;

use super::core;

unsafe fn download_failure(out: *mut *mut c_char, error: &baad_dm::Error) -> i32 {
    write_error(out, &error.to_string());
    BaadDmErrorCode::from(error) as i32
}

pub const BAAD_DM_HASH_TYPE_NONE: i32 = -1;

/// # Safety
/// `hash` must be a valid NUL-terminated string. Returns `BaadDmHashType` as
/// `i32`, or `BAAD_DM_HASH_TYPE_NONE` when no hash type matches.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_detect_hash_type(hash: *const c_char) -> i32 {
    let Ok(hash) = import_string(hash) else {
        return BAAD_DM_HASH_TYPE_NONE;
    };
    baad_dm::detect_hash_type(hash)
        .map_or(BAAD_DM_HASH_TYPE_NONE, |hash_type| BaadDmHashType::from(hash_type) as i32)
}

/// # Safety
/// `file_path` must be a valid NUL-terminated string, `expected` null or a
/// valid NUL-terminated string, `out_matches` a valid pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_verify_hash(
    file_path: *const c_char,
    expected: *const c_char,
    out_matches: *mut bool
) -> i32 {
    let file_path = match import_string(file_path) {
        Ok(value) => value,
        Err(code) => return code
    };
    let expected = match import_string_opt(expected) {
        Ok(value) => value.map(String::from),
        Err(code) => return code
    };
    if out_matches.is_null() {
        return NULL_POINTER;
    }

    match baad_dm::verify_hash(PathBuf::from(file_path).as_path(), expected.as_ref()) {
        Ok(matches) => {
            *out_matches = matches;
            0
        }
        Err(error) => BaadDmErrorCode::from(&error) as i32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_dm_create_range_header(start: u64, end: u64, has_end: bool) -> *mut c_char {
    export_string(&baad_dm::client::create_range_header(start, has_end.then_some(end)))
}

#[repr(C)]
pub struct BaadDmDownload {
    pub url: *const c_char,
    pub filename: *const c_char,
    pub hash: *const c_char,
    pub target_file: *const c_char,
    pub size: u64,
    pub has_size: bool
}

unsafe fn import_download(download: &BaadDmDownload) -> Result<baad_dm::Download, i32> {
    let url = import_string(download.url)?;
    let filename = import_string(download.filename)?;
    let hash = import_string_opt(download.hash)?;
    let target_file = import_string_opt(download.target_file)?;

    let parsed = core::parse_url(url).map_err(|error| BaadDmErrorCode::from(&error) as i32)?;

    Ok(baad_dm::Download::builder()
        .url(parsed)
        .filename(String::from(filename))
        .maybe_hash(hash.map(String::from))
        .maybe_target_file(target_file.map(String::from))
        .maybe_size(download.has_size.then_some(download.size))
        .build())
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum BaadDmSummaryStatus {
    NotStarted = 0,
    Success = 1,
    Skipped = 2,
    Failed = 3,
    HashMismatch = 4
}

#[repr(C)]
pub struct BaadDmSummary {
    pub filename: *mut c_char,
    pub status_code: u16,
    pub size: u64,
    pub status: i32,
    pub reason: *mut c_char,
    pub resumable: bool
}

#[repr(C)]
pub struct BaadDmSummaryArray {
    pub ptr: *mut BaadDmSummary,
    pub len: usize,
    pub cap: usize
}

impl BaadDmSummaryArray {
    fn from_summaries(summaries: Vec<baad_dm::Summary>) -> Self {
        let mut exported: Vec<BaadDmSummary> = summaries
            .iter()
            .map(|summary| {
                let (status, reason) = match &summary.status {
                    baad_shared::DownloadStatus::NotStarted => {
                        (BaadDmSummaryStatus::NotStarted, ptr::null_mut())
                    }
                    baad_shared::DownloadStatus::Success => {
                        (BaadDmSummaryStatus::Success, ptr::null_mut())
                    }
                    baad_shared::DownloadStatus::Skipped(reason) => {
                        (BaadDmSummaryStatus::Skipped, export_string(reason))
                    }
                    baad_shared::DownloadStatus::Failed(reason) => {
                        (BaadDmSummaryStatus::Failed, export_string(reason))
                    }
                    baad_shared::DownloadStatus::HashMismatch(reason) => {
                        (BaadDmSummaryStatus::HashMismatch, export_string(reason))
                    }
                };

                BaadDmSummary {
                    filename: export_string(&summary.download.filename),
                    status_code: summary.status_code.as_u16(),
                    size: summary.size,
                    status: status as i32,
                    reason,
                    resumable: summary.resumable
                }
            })
            .collect();

        let ptr = exported.as_mut_ptr();
        let len = exported.len();
        let cap = exported.capacity();
        std::mem::forget(exported);
        BaadDmSummaryArray { ptr, len, cap }
    }

    const fn empty() -> Self {
        BaadDmSummaryArray {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0
        }
    }
}

/// # Safety
/// `array` must have been returned by this library and not freed before.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_summary_array_free(array: BaadDmSummaryArray) {
    if array.ptr.is_null() {
        return;
    }
    let summaries = Vec::from_raw_parts(array.ptr, array.len, array.cap);
    for summary in summaries {
        baad_dm_string_free(summary.filename);
        baad_dm_string_free(summary.reason);
    }
}

#[repr(C)]
pub struct BaadDmDownloaderConfig {
    pub directory: *const c_char,
    pub concurrent_downloads: u32,
    pub max_chunks_per_file: u32,
    pub max_concurrent_chunks: u32,
    pub chunk_threshold: u64,
    pub retries: u32,
    pub resumable: bool,
    pub overwrite: bool,
    pub proxy: *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_dm_downloader_config_default(
    directory: *const c_char
) -> BaadDmDownloaderConfig {
    BaadDmDownloaderConfig {
        directory,
        concurrent_downloads: 32,
        max_chunks_per_file: 16,
        max_concurrent_chunks: 8,
        chunk_threshold: 10 * 1024 * 1024,
        retries: 3,
        resumable: true,
        overwrite: false,
        proxy: ptr::null()
    }
}

unsafe fn import_options(config: &BaadDmDownloaderConfig) -> Result<core::DownloadOptions, i32> {
    let directory = import_string(config.directory)?;
    let proxy = import_string_opt(config.proxy)?;

    Ok(core::DownloadOptions {
        directory: PathBuf::from(directory),
        concurrent_downloads: config.concurrent_downloads as usize,
        max_chunks_per_file: config.max_chunks_per_file as usize,
        max_concurrent_chunks: config.max_concurrent_chunks as usize,
        chunk_threshold: config.chunk_threshold,
        retries: config.retries,
        resumable: config.resumable,
        overwrite: config.overwrite,
        proxy: core::create_proxy(proxy).map_err(|error| BaadDmErrorCode::from(&error) as i32)?
    })
}

/// Blocks the calling thread until every download finishes. Progress events
/// fire on the observer registered via `baad_dm_set_observer`.
///
/// # Safety
/// `config` must point to a valid config whose strings are NUL-terminated,
/// `downloads` to `count` valid download structs, `out_summaries` to a valid
/// slot. On success the caller owns the array and must free it with
/// `baad_dm_summary_array_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_download(
    config: *const BaadDmDownloaderConfig,
    downloads: *const BaadDmDownload,
    count: usize,
    out_summaries: *mut BaadDmSummaryArray
) -> i32 {
    if config.is_null() || downloads.is_null() || out_summaries.is_null() {
        return NULL_POINTER;
    }
    *out_summaries = BaadDmSummaryArray::empty();

    let options = match import_options(&*config) {
        Ok(options) => options,
        Err(code) => return code
    };

    let mut items = Vec::with_capacity(count);
    for download in std::slice::from_raw_parts(downloads, count) {
        match import_download(download) {
            Ok(item) => items.push(item),
            Err(code) => return code
        }
    }

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let summaries = runtime.block_on(core::run_download(options, items));
    *out_summaries = BaadDmSummaryArray::from_summaries(summaries);
    0
}

/// # Safety
/// `url` and `target` must be valid NUL-terminated strings, `out_data` a
/// valid slot. On success the caller owns the buffer and must free it with
/// `baad_dm_bytes_free`. `out_error` may be null; otherwise on failure it
/// receives a message the caller frees with `baad_dm_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_zip_extract_file(
    url: *const c_char,
    target: *const c_char,
    out_data: *mut BaadDmBytes,
    out_error: *mut *mut c_char
) -> i32 {
    let (url, target) = match (import_string(url), import_string(target)) {
        (Ok(url), Ok(target)) => (url, target),
        (Err(code), _) | (_, Err(code)) => return code
    };
    if out_data.is_null() {
        return NULL_POINTER;
    }
    *out_data = BaadDmBytes::empty();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(async {
        let client = core::default_client()?;
        let parsed = core::parse_url(url)?;
        let extractor = baad_dm::ZipExtractor::new(&client, &parsed).await?;
        extractor.extract_file(target).await
    });

    match result {
        Ok(data) => {
            *out_data = BaadDmBytes::from_vec(data);
            0
        }
        Err(error) => download_failure(out_error, &error)
    }
}

/// # Safety
/// `url` must be a valid NUL-terminated string, `out_names` a valid slot. On
/// success the caller owns the array and must free it with
/// `baad_dm_string_array_free`. `out_error` may be null; otherwise on failure
/// it receives a message the caller frees with `baad_dm_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_zip_list_files(
    url: *const c_char,
    out_names: *mut BaadDmStringArray,
    out_error: *mut *mut c_char
) -> i32 {
    let url = match import_string(url) {
        Ok(url) => url,
        Err(code) => return code
    };
    if out_names.is_null() {
        return NULL_POINTER;
    }
    *out_names = BaadDmStringArray::empty();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(async {
        let client = core::default_client()?;
        let parsed = core::parse_url(url)?;
        let extractor = baad_dm::ZipExtractor::new(&client, &parsed).await?;
        let index = extractor.build_index().await?;
        Ok::<_, baad_dm::Error>(index.names().map(String::from).collect::<Vec<_>>())
    });

    match result {
        Ok(names) => {
            *out_names = BaadDmStringArray::from_strings(&names);
            0
        }
        Err(error) => download_failure(out_error, &error)
    }
}

#[repr(C)]
pub struct BaadDmZipFileInfo {
    pub compression_method: u16,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    pub local_header_offset: u64,
    pub found: bool
}

/// # Safety
/// `url` and `target` must be valid NUL-terminated strings, `out_info` a
/// valid slot. `found` is false when the archive lacks the target. `out_error`
/// may be null; otherwise on failure it receives a message the caller frees
/// with `baad_dm_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_zip_file_info(
    url: *const c_char,
    target: *const c_char,
    out_info: *mut BaadDmZipFileInfo,
    out_error: *mut *mut c_char
) -> i32 {
    let (url, target) = match (import_string(url), import_string(target)) {
        (Ok(url), Ok(target)) => (url, target),
        (Err(code), _) | (_, Err(code)) => return code
    };
    if out_info.is_null() {
        return NULL_POINTER;
    }

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(async {
        let client = core::default_client()?;
        let parsed = core::parse_url(url)?;
        let extractor = baad_dm::ZipExtractor::new(&client, &parsed).await?;
        let index = extractor.build_index().await?;
        Ok::<_, baad_dm::Error>(index.get(target).cloned())
    });

    match result {
        Ok(info) => {
            *out_info = match info {
                Some(info) => BaadDmZipFileInfo {
                    compression_method: info.compression_method,
                    compressed_size: info.compressed_size,
                    uncompressed_size: info.uncompressed_size,
                    local_header_offset: info.local_header_offset,
                    found: true
                },
                None => BaadDmZipFileInfo {
                    compression_method: 0,
                    compressed_size: 0,
                    uncompressed_size: 0,
                    local_header_offset: 0,
                    found: false
                }
            };
            0
        }
        Err(error) => download_failure(out_error, &error)
    }
}

/// # Safety
/// `url` must be a valid NUL-terminated string and `out` a valid pointer to a
/// `char*` slot. On success the caller frees `*out` with
/// `baad_dm_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_dm_resolve_url(url: *const c_char, out: *mut *mut c_char) -> i32 {
    let url = match import_string(url) {
        Ok(url) => url,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(async {
        let client = core::default_client()?;
        baad_dm::client::resolve_url(&client, url).await
    });

    match result {
        Ok(resolved) => {
            *out = export_string(&resolved);
            0
        }
        Err(error) => BaadDmErrorCode::from(&error) as i32
    }
}

include!(concat!(env!("OUT_DIR"), "/c_shadow.rs"));
