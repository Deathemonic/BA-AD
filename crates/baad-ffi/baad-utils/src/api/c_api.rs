//! C ABI for `baad-utils`. Fallible functions return `i32` (`0` success;
//! `-2` null, `-3` invalid argument, `-4` runtime unavailable); owned values
//! are freed with their `baad_utils_*_free` pair. Async calls block on an
//! internal tokio runtime; generated plumbing lives in `c_shadow.rs`.

#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::c_char;
use std::path::PathBuf;
use std::ptr;

use super::core;

/// # Safety
/// `config` must be null (defaults) or point to a valid config struct.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_init_logging(config: *const BaadUtilsLoggingConfig) -> i32 {
    let values = match config.is_null() {
        true => baad_utils_logging_config_default(),
        false => ptr::read(config)
    };

    match baad_utils::config::init_logging(values.to_native()) {
        Ok(()) => 0,
        Err(error) => BaadUtilsConfigErrorCode::from(&error) as i32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_utils_flush_logs() { baad_utils::flush_logs(); }

/// # Safety
/// `level` must be `0` trace, `1` debug, `2` info, `3` warn or `4` error;
/// `message` must be a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_log(level: i32, success: bool, message: *const c_char) -> i32 {
    let Some(level) = core::LogLevel::from_repr(level) else {
        return INVALID_ARGUMENT;
    };
    let message = match import_string(message) {
        Ok(message) => message,
        Err(code) => return code
    };
    core::log_message(level, success, message, None);
    0
}

/// # Safety
/// `message`, `name` and `value` must be valid NUL-terminated strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_log_with_field(
    level: i32,
    success: bool,
    message: *const c_char,
    name: *const c_char,
    value: *const c_char
) -> i32 {
    let Some(level) = core::LogLevel::from_repr(level) else {
        return INVALID_ARGUMENT;
    };
    let (message, name, value) =
        match (import_string(message), import_string(name), import_string(value)) {
            (Ok(message), Ok(name), Ok(value)) => (message, name, value),
            (Err(code), _, _) | (_, Err(code), _) | (_, _, Err(code)) => return code
        };
    core::log_message(level, success, message, Some(&core::join_fields(&[(name, value)])));
    0
}

/// # Safety
/// `message` must be a valid NUL-terminated string. When `len` is non-zero,
/// `names` and `values` must each point to `len` readable NUL-terminated
/// string pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_log_with_fields(
    level: i32,
    success: bool,
    message: *const c_char,
    names: *const *const c_char,
    values: *const *const c_char,
    len: usize
) -> i32 {
    let Some(level) = core::LogLevel::from_repr(level) else {
        return INVALID_ARGUMENT;
    };
    let message = match import_string(message) {
        Ok(message) => message,
        Err(code) => return code
    };
    if len > 0 && (names.is_null() || values.is_null()) {
        return NULL_POINTER;
    }

    let mut fields = Vec::new();
    for index in 0..len {
        let name = match import_string(*names.add(index)) {
            Ok(name) => name,
            Err(code) => return code
        };
        let value = match import_string(*values.add(index)) {
            Ok(value) => value,
            Err(code) => return code
        };
        fields.push((name, value));
    }

    let rendered = match fields.is_empty() {
        true => None,
        false => Some(core::join_fields(&fields))
    };
    core::log_message(level, success, message, rendered.as_deref());
    0
}

/// # Safety
/// `name` must be a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_set_app_name(name: *const c_char) -> i32 {
    let name = match import_string(name) {
        Ok(name) => name,
        Err(code) => return code
    };
    match baad_utils::file::set_app_name(name) {
        Ok(()) => 0,
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_set_data_dir(path: *const c_char) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    match baad_utils::file::set_data_dir(PathBuf::from(path)) {
        Ok(()) => 0,
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `out` must be a valid pointer to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_data_dir(out: *mut *mut c_char) -> i32 {
    if out.is_null() {
        return NULL_POINTER;
    }
    match baad_utils::file::data_dir() {
        Ok(path) => {
            *out = export_string(&path.to_string_lossy());
            0
        }
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `filename` must be a valid NUL-terminated string and `out` a valid pointer
/// to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_get_data_path(
    filename: *const c_char,
    out: *mut *mut c_char
) -> i32 {
    let filename = match import_string(filename) {
        Ok(filename) => filename,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }
    match baad_utils::file::get_data_path(filename) {
        Ok(path) => {
            *out = export_string(&path.to_string_lossy());
            0
        }
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string and `out` a valid pointer to
/// a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_filename_or(path: *const c_char, out: *mut *mut c_char) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }
    *out = export_string(baad_utils::file::filename_or(path));
    0
}

/// # Safety
/// `path` must be a valid NUL-terminated string, `out_data` a valid slot. On
/// success the caller owns the buffer and must free it with
/// `baad_utils_bytes_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_load_file(
    path: *const c_char,
    out_data: *mut BaadUtilsBytes
) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if out_data.is_null() {
        return NULL_POINTER;
    }
    *out_data = BaadUtilsBytes::empty();

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    match runtime.block_on(baad_utils::file::load_file(PathBuf::from(path).as_path())) {
        Ok(data) => {
            *out_data = BaadUtilsBytes::from_vec(data);
            0
        }
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string; `data` must point to `len`
/// readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_save_file(
    path: *const c_char,
    data: *const u8,
    len: usize
) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if data.is_null() && len > 0 {
        return NULL_POINTER;
    }
    let content = match len {
        0 => &[][..],
        _ => std::slice::from_raw_parts(data, len)
    };

    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };

    match runtime.block_on(baad_utils::file::save_file(PathBuf::from(path).as_path(), content)) {
        Ok(()) => 0,
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_create_parent_dir(path: *const c_char) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    match runtime.block_on(baad_utils::file::create_parent_dir(PathBuf::from(path).as_path())) {
        Ok(()) => 0,
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` may be null (defaults to `./output`); `out` must be a valid pointer
/// to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_get_output_dir(
    path: *const c_char,
    out: *mut *mut c_char
) -> i32 {
    let path = match import_string_opt(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    let result =
        runtime.block_on(baad_utils::file::get_output_dir(path.map(PathBuf::from).as_deref()));
    match result {
        Ok(dir) => {
            *out = export_string(&dir.to_string_lossy());
            0
        }
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string and `out_empty` a valid
/// pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_is_dir_empty(path: *const c_char, out_empty: *mut bool) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if out_empty.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    match runtime.block_on(baad_utils::file::is_dir_empty(PathBuf::from(path).as_path())) {
        Ok(empty) => {
            *out_empty = empty;
            0
        }
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_clear_all(path: *const c_char) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    match runtime.block_on(baad_utils::file::clear_all(PathBuf::from(path).as_path())) {
        Ok(()) => 0,
        Err(error) => BaadUtilsFileErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` must be a valid NUL-terminated string and `out` a valid pointer to
/// a `char*` slot. On success the caller frees `*out` with
/// `baad_utils_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_json_load(path: *const c_char, out: *mut *mut c_char) -> i32 {
    let path = match import_string(path) {
        Ok(path) => path,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    match runtime.block_on(core::json_load_string(PathBuf::from(path).as_path())) {
        Ok(json) => {
            *out = export_string(&json);
            0
        }
        Err(error) => BaadUtilsJsonErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `path` and `json` must be valid NUL-terminated strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_json_save(path: *const c_char, json: *const c_char) -> i32 {
    let (path, json) = match (import_string(path), import_string(json)) {
        (Ok(path), Ok(json)) => (path, json),
        (Err(code), _) | (_, Err(code)) => return code
    };
    let Some(runtime) = runtime() else {
        return RUNTIME_UNAVAILABLE;
    };
    match runtime.block_on(core::json_save_string(PathBuf::from(path).as_path(), json)) {
        Ok(()) => 0,
        Err(error) => BaadUtilsJsonErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `url` must be a valid NUL-terminated string and `out` a valid pointer to a
/// `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_fetch_version(
    url: *const c_char,
    out: *mut *mut c_char
) -> i32 {
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
    match runtime.block_on(baad_utils::network::fetch_version(url)) {
        Ok(version) => {
            *out = export_string(&version);
            0
        }
        Err(error) => BaadUtilsNetworkErrorCode::from(&error) as i32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_utils_format_bytes(value: u64) -> *mut c_char {
    export_string(&baad_utils::formatter::HumanBytes(value).to_string())
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_utils_terminal_is_terminal() -> bool {
    baad_utils::progress::terminal::is_terminal()
}

/// # Safety
/// `out_width` and `out_height` must be valid pointers. Returns false when no
/// terminal size is available.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_utils_terminal_size(
    out_width: *mut u64,
    out_height: *mut u64
) -> bool {
    if out_width.is_null() || out_height.is_null() {
        return false;
    }
    match baad_utils::progress::terminal::size() {
        Some((width, height)) => {
            *out_width = width as u64;
            *out_height = height as u64;
            true
        }
        None => false
    }
}

include!(concat!(env!("OUT_DIR"), "/c_shadow.rs"));
