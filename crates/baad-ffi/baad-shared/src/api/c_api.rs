//! C ABI for `baad-shared`. Fallible functions return `i32` (`0` success;
//! `-2` null, `-3` invalid argument); owned values are freed with their
//! `baad_shared_*_free` pair. Generated plumbing, mirrors, constants and the
//! observer callback live in `c_shadow.rs`.

#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::c_char;
use std::ptr;

#[repr(i32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BaadSharedClientErrorCode {
    Success = 0,
    InvalidProxy = 1,
    BuildFailed = 2,
    NullPointer = -2,
    InvalidArgument = -3
}

unsafe fn write_string(out: *mut *mut c_char, value: &str) -> i32 {
    if out.is_null() {
        return NULL_POINTER;
    }
    *out = export_string(value);
    0
}

/// # Safety
/// `out` must be a valid pointer to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_platform_patch_pack(
    platform: i32,
    out: *mut *mut c_char
) -> i32 {
    let Some(platform) = BaadSharedPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    write_string(out, platform.patch_pack())
}

/// # Safety
/// `out` must be a valid pointer to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_platform_media_path(
    platform: i32,
    out: *mut *mut c_char
) -> i32 {
    let Some(platform) = BaadSharedPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    write_string(out, platform.media_path())
}

/// # Safety
/// `out` must be a valid pointer to a `char*` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_platform_display_name(
    platform: i32,
    out: *mut *mut c_char
) -> i32 {
    let Some(platform) = BaadSharedPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    write_string(out, platform.display_name())
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_shared_platform_ensure_supported(platform: i32) -> i32 {
    let Some(platform) = BaadSharedPlatform::to_rust(platform) else {
        return INVALID_ARGUMENT;
    };
    match platform.ensure_supported() {
        Ok(()) => 0,
        Err(error) => BaadSharedServerConfigErrorCode::from(&error) as i32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn baad_shared_china_media_type_is_valid(value: i32) -> bool {
    baad_shared::ChinaMediaType::from_repr(value).is_some()
}

/// # Safety
/// `out_market_game_id` and `out_market_code` must be valid pointers to
/// `char*` slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_market_config_for_global(
    platform: i32,
    build_type: i32,
    out_market_game_id: *mut *mut c_char,
    out_market_code: *mut *mut c_char
) -> i32 {
    let (Some(platform), Some(build_type)) =
        (BaadSharedPlatform::to_rust(platform), BaadSharedBuildType::to_rust(build_type))
    else {
        return INVALID_ARGUMENT;
    };
    if out_market_game_id.is_null() || out_market_code.is_null() {
        return NULL_POINTER;
    }

    match baad_shared::MarketConfig::for_global(platform, build_type) {
        Ok(config) => {
            *out_market_game_id = export_string(config.market_game_id);
            *out_market_code = export_string(config.market_code);
            0
        }
        Err(error) => BaadSharedServerConfigErrorCode::from(&error) as i32
    }
}

/// # Safety
/// `proxy` and `user_agent` must each be null or a valid NUL-terminated
/// string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_init_client(
    proxy: *const c_char,
    user_agent: *const c_char
) -> i32 {
    let (proxy, user_agent) = match (import_string_opt(proxy), import_string_opt(user_agent)) {
        (Ok(proxy), Ok(user_agent)) => (proxy, user_agent),
        (Err(code), _) | (_, Err(code)) => return code
    };

    let mut builder = reqwest::Client::builder();

    if let Some(proxy_url) = proxy {
        match reqwest::Proxy::all(proxy_url) {
            Ok(configured) => builder = builder.proxy(configured),
            Err(_) => return BaadSharedClientErrorCode::InvalidProxy as i32
        }
    }

    if let Some(agent) = user_agent {
        builder = builder.user_agent(agent);
    }

    match builder.build() {
        Ok(client) => {
            baad_shared::set_client(client);
            0
        }
        Err(_) => BaadSharedClientErrorCode::BuildFailed as i32
    }
}

/// # Safety
/// `text` must be a valid NUL-terminated string and `out` a valid pointer to
/// a `char*` slot. `*out` is set to null when no version is found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn baad_shared_extract_version(
    text: *const c_char,
    out: *mut *mut c_char
) -> i32 {
    let text = match import_string(text) {
        Ok(text) => text,
        Err(code) => return code
    };
    if out.is_null() {
        return NULL_POINTER;
    }

    *out = match baad_shared::REGEX_VERSION.find(text) {
        Some(found) => export_string(found.as_str()),
        None => ptr::null_mut()
    };
    0
}

include!(concat!(env!("OUT_DIR"), "/c_shadow.rs"));
