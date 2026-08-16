//! C ABI for `baad`, mirroring its Rust API. Fallible functions return `i32`
//! (`0` success; `-2` null, `-3` invalid argument, `-4` runtime unavailable);
//! owned strings/buffers/handles are freed with their `baad_*_free` pair.
//! Nested catalogs cross as JSON (`_json` suffix), download lists as
//! `BaadDownloads` handles. Async calls block on an internal tokio runtime;
//! generated plumbing and re-exports live in `c_shadow.rs`.

#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::c_char;
use std::path::PathBuf;
use std::ptr;
use std::sync::{Mutex, PoisonError};

use baad::api::{NexonClient, RoStarClient, YoStarClient};
use baad::catalog::{Catalog, ChinaCatalog, GlobalCatalog, JapanCatalog};
use baad::cdn::{ChinaCdn, GlobalCdn, JapanCdn};
use baad::download::{ResourceFilter, download_file};
use baad::strategy::{ChinaStrategy, GlobalStrategy, JapanStrategy};
use baad_shared::{DownloadAsset, DownloadMedia, DownloadTable, Downloads};
use serde::de::DeserializeOwned;
use tokio::runtime::Runtime;

use super::core;

pub const BAAD_DOWNLOAD_KIND_ASSETS: i32 = 0;
pub const BAAD_DOWNLOAD_KIND_TABLES: i32 = 1;
pub const BAAD_DOWNLOAD_KIND_MEDIA: i32 = 2;

pub const BAAD_HASH_KIND_CRC: i32 = 0;
pub const BAAD_HASH_KIND_MD5: i32 = 1;

const DEFAULT_DOWNLOAD_LIMIT: u32 = 32;
const DEFAULT_DOWNLOAD_RETRIES: u32 = 3;

unsafe fn catalog_failure(out: *mut *mut c_char, error: &baad::CatalogError) -> i32 {
    write_error(out, &error.to_string());
    BaadCatalogErrorCode::from(error) as i32
}

unsafe fn import_json<T: DeserializeOwned>(
    value: *const c_char,
    out_error: *mut *mut c_char
) -> Result<T, i32> {
    let json = import_string(value)?;
    serde_json::from_str(json).map_err(|error| {
        write_error(out_error, &error.to_string());
        INVALID_ARGUMENT
    })
}

unsafe fn write_json<T: serde::Serialize>(
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
    result: Result<T, baad::CatalogError>
) -> i32 {
    let value = match result {
        Ok(value) => value,
        Err(error) => return catalog_failure(out_error, &error)
    };

    match serde_json::to_string(&value) {
        Ok(json) => {
            *out_json = export_string(&json);
            0
        }
        Err(error) => catalog_failure(out_error, &baad::CatalogError::from(error))
    }
}

/// Packs the requested categories into the `category` argument taken by the
/// catalog constructors. Selecting nothing selects every category.
#[unsafe(no_mangle)]
pub extern "C" fn baad_category(assets: bool, tables: bool, media: bool) -> u8 {
    core::category_bits(assets, tables, media)
}

/// # Safety
/// `pattern` must be a valid NUL-terminated string, `method` a
/// `BaadFilterMethod` value, and `out_filter` a valid slot. On success the
/// caller owns the handle and must free it with `baad_resource_filter_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_resource_filter_new(
    pattern: *const c_char,
    method: i32,
    out_filter: *mut *mut BaadResourceFilter
) -> i32 {
    let pattern = match import_string(pattern) {
        Ok(pattern) => pattern,
        Err(code) => return code
    };
    if out_filter.is_null() {
        return NULL_POINTER;
    }
    *out_filter = ptr::null_mut();

    let Some(method) = BaadFilterMethod::to_rust(method) else {
        return INVALID_ARGUMENT;
    };

    match ResourceFilter::new(pattern, method) {
        Ok(matcher) => {
            *out_filter = Box::into_raw(Box::new(BaadResourceFilter {
                inner: Mutex::new(matcher)
            }));
            0
        }
        Err(error) => BaadFilterErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `filter` must be a live handle, `path` a valid NUL-terminated string and
/// `out_matches` a valid pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_resource_filter_matches(
    filter: *const BaadResourceFilter,
    path: *const c_char,
    out_matches: *mut bool
) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if filter.is_null() || out_matches.is_null() {
        return NULL_POINTER;
    }

    let matcher = (*filter).inner.lock().unwrap_or_else(PoisonError::into_inner);
    *out_matches = matcher.matches(path);
    0
}

/// `kind` is one of `BAAD_DOWNLOAD_KIND_*`.
///
/// # Safety
/// `downloads` must be a live handle and `out_count` a valid pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_downloads_count(
    downloads: *const BaadDownloads,
    kind: i32,
    out_count: *mut usize
) -> i32 {
    if downloads.is_null() || out_count.is_null() {
        return NULL_POINTER;
    }
    let downloads = &(*downloads).inner;

    *out_count = match kind {
        BAAD_DOWNLOAD_KIND_ASSETS => downloads.assets.len(),
        BAAD_DOWNLOAD_KIND_TABLES => downloads.tables.len(),
        BAAD_DOWNLOAD_KIND_MEDIA => downloads.media.len(),
        _ => return INVALID_ARGUMENT
    };
    0
}

#[repr(C)]
pub struct BaadDownloadEntry {
    pub url: *mut c_char,
    pub path: *mut c_char,
    pub hash: *mut c_char,
    pub hash_kind: i32,
    pub size: i64,
    pub bundle_files: BaadStringArray
}

impl BaadDownloadEntry {
    fn new(
        url: &str,
        path: &str,
        hash: &baad_shared::HashValue,
        size: i64,
        bundle_files: &[String]
    ) -> Self {
        let hash_kind = match hash {
            baad_shared::HashValue::Crc(_) => BAAD_HASH_KIND_CRC,
            baad_shared::HashValue::Md5(_) => BAAD_HASH_KIND_MD5
        };

        Self {
            url: export_string(url),
            path: export_string(path),
            hash: export_string(&hash.as_string()),
            hash_kind,
            size,
            bundle_files: BaadStringArray::from_strings(bundle_files)
        }
    }

    fn asset(asset: &DownloadAsset) -> Self {
        Self::new(&asset.url, &asset.path, &asset.hash, asset.size, &asset.bundle_files)
    }

    fn table(table: &DownloadTable) -> Self {
        Self::new(&table.url, &table.path, &table.hash, table.size, &table.bundle_files)
    }

    fn media(media: &DownloadMedia) -> Self {
        Self::new(&media.url, &media.path, &media.hash, media.size, &[])
    }
}

/// # Safety
/// `downloads` must be a live handle and `out_entry` a valid slot. On success
/// the caller owns the entry and must free it with `baad_download_entry_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_downloads_entry(
    downloads: *const BaadDownloads,
    kind: i32,
    index: usize,
    out_entry: *mut BaadDownloadEntry
) -> i32 {
    if downloads.is_null() || out_entry.is_null() {
        return NULL_POINTER;
    }
    let downloads = &(*downloads).inner;

    let entry = match kind {
        BAAD_DOWNLOAD_KIND_ASSETS => downloads.assets.get(index).map(BaadDownloadEntry::asset),
        BAAD_DOWNLOAD_KIND_TABLES => downloads.tables.get(index).map(BaadDownloadEntry::table),
        BAAD_DOWNLOAD_KIND_MEDIA => downloads.media.get(index).map(BaadDownloadEntry::media),
        _ => return INVALID_ARGUMENT
    };

    entry.map_or(INVALID_ARGUMENT, |entry| {
        *out_entry = entry;
        0
    })
}

/// # Safety
/// `entry` must have been returned by `baad_downloads_entry` and not freed
/// before.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_download_entry_free(entry: BaadDownloadEntry) {
    baad_string_free(entry.url);
    baad_string_free(entry.path);
    baad_string_free(entry.hash);
    baad_string_array_free(entry.bundle_files);
}

unsafe fn write_downloads(
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char,
    result: Result<Downloads, baad::CatalogError>
) -> i32 {
    match result {
        Ok(downloads) => {
            *out_downloads = Box::into_raw(Box::new(BaadDownloads { inner: downloads }));
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

unsafe fn write_catalog_url(
    out_url: *mut *mut c_char,
    out_up_to_date: *mut bool,
    out_error: *mut *mut c_char,
    result: Result<(String, bool), baad::CatalogError>
) -> i32 {
    match result {
        Ok((url, up_to_date)) => {
            *out_url = export_string(&url);
            *out_up_to_date = up_to_date;
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Build `category` with `baad_category`; zero selects every category.
///
/// # Safety
/// `platform` must be a `BaadPlatform` value and `out_catalog` a valid slot. On
/// success the caller owns the handle and must free it with
/// `baad_japan_catalog_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_catalog_new(
    category: u8,
    platform: i32,
    out_catalog: *mut *mut BaadJapanCatalog,
    out_error: *mut *mut c_char
) -> i32 {
    if out_catalog.is_null() {
        return NULL_POINTER;
    }
    *out_catalog = ptr::null_mut();

    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };

    match JapanCatalog::new(core::resource_category(category), platform) {
        Ok(inner) => {
            *out_catalog = Box::into_raw(Box::new(BaadJapanCatalog { inner }));
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Blocks the calling thread.
///
/// # Safety
/// `catalog` must be a live handle; `out_url` and `out_up_to_date` valid slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_catalog_get_catalog_url(
    catalog: *const BaadJapanCatalog,
    out_url: *mut *mut c_char,
    out_up_to_date: *mut bool,
    out_error: *mut *mut c_char
) -> i32 {
    if catalog.is_null() || out_url.is_null() || out_up_to_date.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.get_catalog_url());
    write_catalog_url(out_url, out_up_to_date, out_error, result)
}

/// Blocks the calling thread.
///
/// # Safety
/// `catalog` must be a live handle and `out_downloads` a valid slot. On success
/// the caller owns the handle and must free it with `baad_downloads_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_catalog_prepare_downloads(
    catalog: *const BaadJapanCatalog,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    if catalog.is_null() || out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.prepare_downloads());
    write_downloads(out_downloads, out_error, result)
}

/// Build `category` with `baad_category`; zero selects every category.
///
/// # Safety
/// `platform` must be a `BaadPlatform` value, `build_type` a `BaadBuildType`
/// value and `out_catalog` a valid slot. On success the caller owns the handle
/// and must free it with `baad_global_catalog_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_catalog_new(
    category: u8,
    platform: i32,
    build_type: i32,
    out_catalog: *mut *mut BaadGlobalCatalog,
    out_error: *mut *mut c_char
) -> i32 {
    if out_catalog.is_null() {
        return NULL_POINTER;
    }
    *out_catalog = ptr::null_mut();

    let (Some(platform), Some(build_type)) =
        (BaadPlatform::to_rust(platform), BaadBuildType::to_rust(build_type))
    else {
        return INVALID_ARGUMENT;
    };

    match GlobalCatalog::new(core::resource_category(category), platform, build_type) {
        Ok(inner) => {
            *out_catalog = Box::into_raw(Box::new(BaadGlobalCatalog { inner }));
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Blocks the calling thread.
///
/// # Safety
/// `catalog` must be a live handle, `version` a valid NUL-terminated string,
/// and `out_url`/`out_up_to_date` valid slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_catalog_get_catalog_url(
    catalog: *const BaadGlobalCatalog,
    version: *const c_char,
    out_url: *mut *mut c_char,
    out_up_to_date: *mut bool,
    out_error: *mut *mut c_char
) -> i32 {
    let version = match import_string(version) {
        Ok(version) => version,
        Err(code) => return code
    };
    if catalog.is_null() || out_url.is_null() || out_up_to_date.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.get_catalog_url(version));
    write_catalog_url(out_url, out_up_to_date, out_error, result)
}

/// Blocks the calling thread and returns the catalog data as JSON.
///
/// # Safety
/// `catalog` must be a live handle, `catalog_url` a valid NUL-terminated
/// string and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_catalog_fetch_catalogs_json(
    catalog: *const BaadGlobalCatalog,
    catalog_url: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if catalog.is_null() || out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.fetch_catalogs(catalog_url));
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread.
///
/// # Safety
/// `catalog` must be a live handle and `out_downloads` a valid slot. On success
/// the caller owns the handle and must free it with `baad_downloads_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_catalog_prepare_downloads(
    catalog: *const BaadGlobalCatalog,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    if catalog.is_null() || out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.prepare_downloads());
    write_downloads(out_downloads, out_error, result)
}

/// Build `category` with `baad_category`; zero selects every category.
///
/// # Safety
/// `platform` must be a `BaadPlatform` value and `out_catalog` a valid slot. On
/// success the caller owns the handle and must free it with
/// `baad_china_catalog_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_catalog_new(
    category: u8,
    platform: i32,
    out_catalog: *mut *mut BaadChinaCatalog,
    out_error: *mut *mut c_char
) -> i32 {
    if out_catalog.is_null() {
        return NULL_POINTER;
    }
    *out_catalog = ptr::null_mut();

    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };

    match ChinaCatalog::new(core::resource_category(category), platform) {
        Ok(inner) => {
            *out_catalog = Box::into_raw(Box::new(BaadChinaCatalog { inner }));
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Blocks the calling thread.
///
/// # Safety
/// `catalog` must be a live handle and `out_downloads` a valid slot. On success
/// the caller owns the handle and must free it with `baad_downloads_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_catalog_prepare_downloads(
    catalog: *const BaadChinaCatalog,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    if catalog.is_null() || out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on((*catalog).inner.prepare_downloads());
    write_downloads(out_downloads, out_error, result)
}

/// # Safety
/// `catalog_url` must be a valid NUL-terminated string, `platform` a
/// `BaadPlatform` value and `out_cdn` a valid slot. On success the caller owns
/// the handle and must free it with `baad_japan_cdn_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_new(
    catalog_url: *const c_char,
    platform: i32,
    out_cdn: *mut *mut BaadJapanCdn
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_cdn.is_null() {
        return NULL_POINTER;
    }
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };

    *out_cdn = Box::into_raw(Box::new(BaadJapanCdn {
        inner: JapanCdn::new(String::from(catalog_url), platform)
    }));
    0
}

unsafe fn cdn_fetch_json<T, F>(
    cdn_is_null: bool,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
    fetch: F
) -> i32
where
    T: serde::Serialize,
    F: FnOnce(&'static Runtime) -> Result<T, baad::CatalogError>
{
    if cdn_is_null || out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    write_json(out_json, out_error, fetch(runtime))
}

/// Blocks the calling thread and returns `JapanResources` as JSON. Build
/// `category` with `baad_category`; zero selects every category.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_fetch_json(
    cdn: *const BaadJapanCdn,
    category: u8,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        let resources = runtime.block_on((*cdn).inner.fetch(core::resource_category(category)))?;
        Ok(serde_json::json!({
            "assets": resources.assets,
            "table": resources.table,
            "media": resources.media
        }))
    })
}

/// Blocks the calling thread and returns `BundlePatchPackInfo` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_fetch_assets_json(
    cdn: *const BaadJapanCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_assets())
    })
}

/// Blocks the calling thread and returns `TableCatalog` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_fetch_table_json(
    cdn: *const BaadJapanCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_table())
    })
}

/// Blocks the calling thread and returns `MediaCatalog` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_fetch_media_json(
    cdn: *const BaadJapanCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_media())
    })
}

/// Blocks the calling thread and returns `JapanAddressable` as JSON.
///
/// # Safety
/// `url` must be a valid NUL-terminated string and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_fetch_addressable_json(
    url: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let url = match import_string(url) {
        Ok(url) => url,
        Err(code) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(JapanCdn::fetch_addressable(url));
    write_json(out_json, out_error, result)
}

/// Extracts the catalog URL from `JapanAddressable` JSON.
///
/// # Safety
/// `addressable_json` must be a valid NUL-terminated string and `out_url` a
/// valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_cdn_extract_catalog_url(
    addressable_json: *const c_char,
    out_url: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    if out_url.is_null() {
        return NULL_POINTER;
    }
    let addressable: baad_shared::JapanAddressable = match import_json(addressable_json, out_error)
    {
        Ok(addressable) => addressable,
        Err(code) => return code
    };

    match JapanCdn::extract_catalog_url(&addressable) {
        Ok(url) => {
            *out_url = export_string(url);
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// # Safety
/// `catalog_url` must be a valid NUL-terminated string, `platform` a
/// `BaadPlatform` value and `out_cdn` a valid slot. On success the caller owns
/// the handle and must free it with `baad_global_cdn_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_cdn_new(
    catalog_url: *const c_char,
    platform: i32,
    out_cdn: *mut *mut BaadGlobalCdn
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_cdn.is_null() {
        return NULL_POINTER;
    }
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };

    *out_cdn = Box::into_raw(Box::new(BaadGlobalCdn {
        inner: GlobalCdn::new(String::from(catalog_url), platform)
    }));
    0
}

/// Blocks the calling thread and returns `GlobalCatalogData` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_cdn_fetch_json(
    cdn: *const BaadGlobalCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch())
    })
}

/// # Safety
/// `resource_path` must be a valid NUL-terminated string and `out` a valid
/// pointer to a `char*` slot. `*out` is set to null when no base URL is found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_cdn_derive_base_url(
    resource_path: *const c_char,
    out: *mut *mut c_char
) -> i32 {
    let resource_path = match import_string(resource_path) {
        Ok(resource_path) => resource_path,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }

    *out = GlobalCdn::derive_base_url(resource_path).map_or(ptr::null_mut(), export_string);
    0
}

/// # Safety
/// All strings must be valid NUL-terminated strings, `platform` a
/// `BaadPlatform` value and `out_cdn` a valid slot. On success the caller owns
/// the handle and must free it with `baad_china_cdn_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_cdn_new(
    catalog_url: *const c_char,
    platform: i32,
    resource_version: *const c_char,
    table_version: *const c_char,
    media_version: *const c_char,
    out_cdn: *mut *mut BaadChinaCdn
) -> i32 {
    let (catalog_url, resource_version, table_version, media_version) = match (
        import_string(catalog_url),
        import_string(resource_version),
        import_string(table_version),
        import_string(media_version)
    ) {
        (Ok(catalog_url), Ok(resource_version), Ok(table_version), Ok(media_version)) => {
            (catalog_url, resource_version, table_version, media_version)
        }
        (Err(code), ..) | (_, Err(code), ..) | (_, _, Err(code), _) | (.., Err(code)) => {
            return code;
        }
    };
    if out_cdn.is_null() {
        return NULL_POINTER;
    }
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };

    *out_cdn = Box::into_raw(Box::new(BaadChinaCdn {
        inner: ChinaCdn::new(
            String::from(catalog_url),
            platform,
            String::from(resource_version),
            String::from(table_version),
            String::from(media_version)
        )
    }));
    0
}

/// Blocks the calling thread and returns `ChinaResources` as JSON. Build
/// `category` with `baad_category`; zero selects every category.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_cdn_fetch_json(
    cdn: *const BaadChinaCdn,
    category: u8,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        let resources = runtime.block_on((*cdn).inner.fetch(core::resource_category(category)))?;
        Ok(serde_json::json!({
            "assets": resources.assets,
            "table": resources.table,
            "media": resources.media
        }))
    })
}

/// Blocks the calling thread and returns `BundleCatalogCN` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_cdn_fetch_assets_json(
    cdn: *const BaadChinaCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_assets())
    })
}

/// Blocks the calling thread and returns `TableCatalogCN` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_cdn_fetch_table_json(
    cdn: *const BaadChinaCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_table())
    })
}

/// Blocks the calling thread and returns `MediaCatalogCN` as JSON.
///
/// # Safety
/// `cdn` must be a live handle and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_cdn_fetch_media_json(
    cdn: *const BaadChinaCdn,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    cdn_fetch_json(cdn.is_null(), out_json, out_error, |runtime| {
        runtime.block_on((*cdn).inner.fetch_media())
    })
}

/// Blocks the calling thread and returns `GlobalAddressable` as JSON.
///
/// # Safety
/// `version` and `build_number` must be valid NUL-terminated strings, and
/// `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_nexon_client_get_addressable_json(
    platform: i32,
    build_type: i32,
    version: *const c_char,
    build_number: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let (version, build_number) = match (import_string(version), import_string(build_number)) {
        (Ok(version), Ok(build_number)) => (version, build_number),
        (Err(code), _) | (_, Err(code)) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }

    let (Some(platform), Some(build_type)) =
        (BaadPlatform::to_rust(platform), BaadBuildType::to_rust(build_type))
    else {
        return INVALID_ARGUMENT;
    };

    let market_config = match baad_shared::MarketConfig::for_global(platform, build_type) {
        Ok(market_config) => market_config,
        Err(error) => return catalog_failure(out_error, &baad::CatalogError::from(error))
    };

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result =
        runtime.block_on(NexonClient::new().get_addressable(&market_config, version, build_number));
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread and returns `GlobalCatalogData` as JSON.
///
/// # Safety
/// `resource_path` must be a valid NUL-terminated string and `out_json` a valid
/// slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_nexon_client_get_catalog_json(
    resource_path: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let resource_path = match import_string(resource_path) {
        Ok(resource_path) => resource_path,
        Err(code) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(NexonClient::new().get_catalog(resource_path));
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread and returns `GameBaseConfig` as JSON.
///
/// # Safety
/// `out_json` must be a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_yostar_client_get_base_config_json(
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(YoStarClient::new().get_base_config());
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread and returns `Domain` as JSON.
///
/// # Safety
/// `out_json` must be a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_yostar_client_get_domain_json(
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(YoStarClient::new().get_domain());
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread and returns `GameJsonConfig` as JSON.
///
/// # Safety
/// `version` and `file_path` must be valid NUL-terminated strings and
/// `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_yostar_client_get_json_config_json(
    version: *const c_char,
    file_path: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let (version, file_path) = match (import_string(version), import_string(file_path)) {
        (Ok(version), Ok(file_path)) => (version, file_path),
        (Err(code), _) | (_, Err(code)) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(YoStarClient::new().get_json_config(version, file_path));
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread and returns `GameJsonData` as JSON.
///
/// # Safety
/// `url` must be a valid NUL-terminated string and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_yostar_client_get_json_data_json(
    url: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let url = match import_string(url) {
        Ok(url) => url,
        Err(code) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(YoStarClient::new().get_json_data(url));
    write_json(out_json, out_error, result)
}

/// Blocks the calling thread.
///
/// # Safety
/// `out_url`, `out_path`, `out_hash` and `out_size` must be valid pointers to
/// `char*` slots. `out_size` receives the size as reported by the launcher.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_yostar_client_get_resources_asset(
    out_url: *mut *mut c_char,
    out_path: *mut *mut c_char,
    out_hash: *mut *mut c_char,
    out_size: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    if out_url.is_null() || out_path.is_null() || out_hash.is_null() || out_size.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    match runtime.block_on(YoStarClient::new().get_resources_asset()) {
        Ok((url, file)) => {
            *out_url = export_string(&url);
            *out_path = export_string(&file.path);
            *out_hash = export_string(&file.hash);
            *out_size = export_string(&file.size);
            0
        }
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Blocks the calling thread and returns `ChinaState` as JSON.
///
/// # Safety
/// `version` must be a valid NUL-terminated string and `out_json` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_rostar_client_get_state_json(
    version: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char
) -> i32 {
    let version = match import_string(version) {
        Ok(version) => version,
        Err(code) => return code
    };
    if out_json.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(RoStarClient::new().get_state(version));
    write_json(out_json, out_error, result)
}

unsafe fn export_downloads(out_downloads: *mut *mut BaadDownloads, downloads: Downloads) -> i32 {
    *out_downloads = Box::into_raw(Box::new(BaadDownloads { inner: downloads }));
    0
}

/// Builds asset downloads from `BundlePatchPackInfo` JSON. On success the
/// caller owns the handle (assets only) and must free it with
/// `baad_downloads_free`.
///
/// # Safety
/// `packing_json` and `catalog_url` must be valid NUL-terminated strings,
/// `platform` a `BaadPlatform` value and `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_strategy_build_asset_downloads(
    packing_json: *const c_char,
    catalog_url: *const c_char,
    platform: i32,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    let packing: baad_shared::BundlePatchPackInfo = match import_json(packing_json, out_error) {
        Ok(packing) => packing,
        Err(code) => return code
    };

    let assets = JapanStrategy::build_asset_downloads(packing, catalog_url, platform);
    export_downloads(out_downloads, Downloads {
        assets,
        tables: Vec::new(),
        media: Vec::new()
    })
}

/// Builds media downloads from `MediaCatalog` JSON. On success the caller owns
/// the handle (media only) and must free it with `baad_downloads_free`.
///
/// # Safety
/// `catalog_json` and `catalog_url` must be valid NUL-terminated strings,
/// `platform` a `BaadPlatform` value and `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_strategy_build_media_downloads(
    catalog_json: *const c_char,
    catalog_url: *const c_char,
    platform: i32,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    let catalog: baad_shared::MediaCatalog = match import_json(catalog_json, out_error) {
        Ok(catalog) => catalog,
        Err(code) => return code
    };

    let media = JapanStrategy::build_media_downloads(catalog, catalog_url, platform);
    export_downloads(out_downloads, Downloads {
        assets: Vec::new(),
        tables: Vec::new(),
        media
    })
}

/// Builds table downloads from `TableCatalog` JSON. On success the caller owns
/// the handle (tables only) and must free it with `baad_downloads_free`.
///
/// # Safety
/// `catalog_json` and `catalog_url` must be valid NUL-terminated strings and
/// `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_japan_strategy_build_table_downloads(
    catalog_json: *const c_char,
    catalog_url: *const c_char,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let catalog: baad_shared::TableCatalog = match import_json(catalog_json, out_error) {
        Ok(catalog) => catalog,
        Err(code) => return code
    };

    let tables = JapanStrategy::build_table_downloads(catalog, catalog_url);
    export_downloads(out_downloads, Downloads {
        assets: Vec::new(),
        tables,
        media: Vec::new()
    })
}

/// Builds downloads from a JSON array of `Resource` entries. Build `category`
/// with `baad_category`; zero selects every category. On success the caller
/// owns the handle and must free it with `baad_downloads_free`.
///
/// # Safety
/// `resources_json` and `base_url` must be valid NUL-terminated strings and
/// `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_global_strategy_build_downloads(
    resources_json: *const c_char,
    base_url: *const c_char,
    category: u8,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let base_url = match import_string(base_url) {
        Ok(base_url) => base_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let resources: Vec<baad_shared::Resource> = match import_json(resources_json, out_error) {
        Ok(resources) => resources,
        Err(code) => return code
    };

    let downloads =
        GlobalStrategy::build_downloads(resources, base_url, core::resource_category(category));
    export_downloads(out_downloads, downloads)
}

/// Builds asset downloads from `BundleCatalogCN` JSON. On success the caller
/// owns the handle (assets only) and must free it with `baad_downloads_free`.
///
/// # Safety
/// `catalog_json` and `catalog_url` must be valid NUL-terminated strings,
/// `platform` a `BaadPlatform` value and `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_strategy_build_asset_downloads(
    catalog_json: *const c_char,
    catalog_url: *const c_char,
    platform: i32,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let Some(platform) = BaadPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    let catalog: baad_shared::BundleCatalogCN = match import_json(catalog_json, out_error) {
        Ok(catalog) => catalog,
        Err(code) => return code
    };

    let assets = ChinaStrategy::build_asset_downloads(catalog, catalog_url, platform);
    export_downloads(out_downloads, Downloads {
        assets,
        tables: Vec::new(),
        media: Vec::new()
    })
}

/// Builds media downloads from `MediaCatalogCN` JSON. On success the caller
/// owns the handle (media only) and must free it with `baad_downloads_free`.
///
/// # Safety
/// `catalog_json` and `catalog_url` must be valid NUL-terminated strings and
/// `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_strategy_build_media_downloads(
    catalog_json: *const c_char,
    catalog_url: *const c_char,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let catalog: baad_shared::MediaCatalogCN = match import_json(catalog_json, out_error) {
        Ok(catalog) => catalog,
        Err(code) => return code
    };

    let media = ChinaStrategy::build_media_downloads(catalog, catalog_url);
    export_downloads(out_downloads, Downloads {
        assets: Vec::new(),
        tables: Vec::new(),
        media
    })
}

/// Builds table downloads from `TableCatalogCN` JSON. On success the caller
/// owns the handle (tables only) and must free it with `baad_downloads_free`.
///
/// # Safety
/// `catalog_json` and `catalog_url` must be valid NUL-terminated strings and
/// `out_downloads` a valid slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_china_strategy_build_table_downloads(
    catalog_json: *const c_char,
    catalog_url: *const c_char,
    out_downloads: *mut *mut BaadDownloads,
    out_error: *mut *mut c_char
) -> i32 {
    let catalog_url = match import_string(catalog_url) {
        Ok(catalog_url) => catalog_url,
        Err(code) => return code
    };
    if out_downloads.is_null() {
        return NULL_POINTER;
    }
    *out_downloads = ptr::null_mut();
    let catalog: baad_shared::TableCatalogCN = match import_json(catalog_json, out_error) {
        Ok(catalog) => catalog,
        Err(code) => return code
    };

    let tables = ChinaStrategy::build_table_downloads(catalog, catalog_url);
    export_downloads(out_downloads, Downloads {
        assets: Vec::new(),
        tables,
        media: Vec::new()
    })
}

#[repr(C)]
pub struct BaadDownloaderOptions {
    pub output_dir: *const c_char,
    pub limit: u32,
    pub retries: u32,
    pub proxy: *const c_char
}

#[unsafe(no_mangle)]
pub const extern "C" fn baad_downloader_options_default(
    output_dir: *const c_char
) -> BaadDownloaderOptions {
    BaadDownloaderOptions {
        output_dir,
        limit: DEFAULT_DOWNLOAD_LIMIT,
        retries: DEFAULT_DOWNLOAD_RETRIES,
        proxy: ptr::null()
    }
}

/// Blocks the calling thread until every download finishes. Progress events
/// fire on the observer registered via `baad_set_observer`.
///
/// # Safety
/// `options` must point to a valid options struct, `downloads` to a live
/// handle, and `filter` to a live handle or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_resource_downloader_download(
    options: *const BaadDownloaderOptions,
    downloads: *const BaadDownloads,
    filter: *const BaadResourceFilter,
    out_error: *mut *mut c_char
) -> i32 {
    if options.is_null() || downloads.is_null() {
        return NULL_POINTER;
    }
    let options = &*options;

    let (output_dir, proxy) =
        match (import_string(options.output_dir), import_string_opt(options.proxy)) {
            (Ok(output_dir), Ok(proxy)) => (output_dir, proxy),
            (Err(code), _) | (_, Err(code)) => return code
        };

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(core::run_download(
        core::DownloadOptions {
            output_dir: PathBuf::from(output_dir),
            limit: options.limit as usize,
            retries: options.retries,
            proxy: proxy.map(String::from)
        },
        (*downloads).inner.clone(),
        filter.as_ref().map(|filter| &filter.inner)
    ));

    match result {
        Ok(()) => 0,
        Err(error) => catalog_failure(out_error, &error)
    }
}

/// Blocks the calling thread.
///
/// # Safety
/// `url` and `output_path` must be valid NUL-terminated strings; `hash` must be
/// null or a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_download_file(
    url: *const c_char,
    output_path: *const c_char,
    hash: *const c_char,
    retries: u32,
    out_error: *mut *mut c_char
) -> i32 {
    let (url, output_path, hash) =
        match (import_string(url), import_string(output_path), import_string_opt(hash)) {
            (Ok(url), Ok(output_path), Ok(hash)) => (url, output_path, hash),
            (Err(code), ..) | (_, Err(code), _) | (_, _, Err(code)) => return code
        };

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    let result = runtime.block_on(download_file(
        url,
        PathBuf::from(output_path).as_path(),
        hash.map(String::from),
        retries
    ));

    match result {
        Ok(()) => 0,
        Err(error) => catalog_failure(out_error, &error)
    }
}

include!(concat!(env!("OUT_DIR"), "/c_shadow.rs"));
