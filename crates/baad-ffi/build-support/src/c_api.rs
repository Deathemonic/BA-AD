use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum};

use crate::config::Config;
use crate::consts::{ConstType, GeneratedConst};
use crate::context::{Context, ErrorCode, Remote, RemoteItem};
use crate::observer::render_observer_c;
use crate::syntax::{crate_path, enum_is_fieldless};

pub(crate) fn render_c(config: &Config, context: &Context) -> TokenStream {
    let plumbing = render_c_plumbing(config);
    let enums = context.remotes.iter().filter_map(|remote| {
        let item = match &remote.item {
            RemoteItem::Enum(item) => item,
            _ => return None
        };
        if !enum_is_fieldless(item) {
            return None;
        }
        Some(render_c_enum(config, remote, item))
    });

    let error_codes = context.error_codes.iter().map(|error| render_c_error_code(config, error));
    let const_getters = context.consts.iter().map(|generated| render_c_const(config, generated));
    let observer = config.observer.then(|| render_observer_c(config));

    quote! {
        #plumbing
        #(#enums)*
        #(#error_codes)*
        #(#const_getters)*
        #observer
    }
}

fn render_c_plumbing(config: &Config) -> TokenStream {
    let bytes_type = format_ident!("{}Bytes", config.c_types_prefix);
    let string_array_type = format_ident!("{}StringArray", config.c_types_prefix);
    let string_free = format_ident!("{}_string_free", config.c_prefix);
    let bytes_free = format_ident!("{}_bytes_free", config.c_prefix);
    let string_array_free = format_ident!("{}_string_array_free", config.c_prefix);
    let string_free_doc =
        format!(" On success the caller owns each string and must free it with `{string_free}`.");
    let runtime = config.c_runtime.then(|| {
        quote! {
            #[allow(dead_code)]
            fn runtime() -> Option<&'static tokio::runtime::Runtime> {
                static RUNTIME: std::sync::OnceLock<Option<tokio::runtime::Runtime>> =
                    std::sync::OnceLock::new();
                RUNTIME.get_or_init(|| tokio::runtime::Runtime::new().ok()).as_ref()
            }
        }
    });

    quote! {
        #[allow(dead_code)]
        const NULL_POINTER: i32 = -2;
        #[allow(dead_code)]
        const INVALID_ARGUMENT: i32 = -3;
        #[allow(dead_code)]
        const RUNTIME_UNAVAILABLE: i32 = -4;

        #runtime

        #[allow(dead_code)]
        fn export_string(value: &str) -> *mut std::ffi::c_char {
            std::ffi::CString::new(value)
                .map_or(std::ptr::null_mut(), std::ffi::CString::into_raw)
        }

        #[allow(dead_code)]
        unsafe fn import_string<'a>(value: *const std::ffi::c_char) -> Result<&'a str, i32> {
            if value.is_null() {
                return Err(NULL_POINTER);
            }
            unsafe { std::ffi::CStr::from_ptr(value) }.to_str().map_err(|_| INVALID_ARGUMENT)
        }

        #[allow(dead_code)]
        unsafe fn import_string_opt<'a>(
            value: *const std::ffi::c_char
        ) -> Result<Option<&'a str>, i32> {
            if value.is_null() {
                return Ok(None);
            }
            unsafe { import_string(value) }.map(Some)
        }

        #[allow(dead_code)]
        unsafe fn write_error(out: *mut *mut std::ffi::c_char, message: &str) {
            if !out.is_null() {
                unsafe { *out = export_string(message) };
            }
        }

        #[repr(C)]
        pub struct #bytes_type {
            pub ptr: *mut u8,
            pub len: usize,
            pub cap: usize
        }

        #[allow(dead_code)]
        impl #bytes_type {
            fn from_vec(mut value: Vec<u8>) -> Self {
                let ptr = value.as_mut_ptr();
                let len = value.len();
                let cap = value.capacity();
                std::mem::forget(value);
                Self { ptr, len, cap }
            }

            const fn empty() -> Self {
                Self {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                    cap: 0
                }
            }
        }

        #[repr(C)]
        pub struct #string_array_type {
            pub ptr: *mut *mut std::ffi::c_char,
            pub len: usize,
            pub cap: usize
        }

        #[allow(dead_code)]
        impl #string_array_type {
            fn from_strs(values: &[&str]) -> Self {
                Self::from_pointers(values.iter().copied().map(export_string).collect())
            }

            fn from_strings(values: &[String]) -> Self {
                Self::from_pointers(values.iter().map(|value| export_string(value)).collect())
            }

            fn from_pointers(mut pointers: Vec<*mut std::ffi::c_char>) -> Self {
                let ptr = pointers.as_mut_ptr();
                let len = pointers.len();
                let cap = pointers.capacity();
                std::mem::forget(pointers);
                Self { ptr, len, cap }
            }

            const fn empty() -> Self {
                Self {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                    cap: 0
                }
            }
        }

        /// # Safety
        /// `value` must be a pointer previously returned by this library, or null.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #string_free(value: *mut std::ffi::c_char) {
            if !value.is_null() {
                drop(unsafe { std::ffi::CString::from_raw(value) });
            }
        }

        /// # Safety
        /// `bytes` must have been returned by this library and not freed before.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #bytes_free(bytes: #bytes_type) {
            if !bytes.ptr.is_null() {
                drop(unsafe { Vec::from_raw_parts(bytes.ptr, bytes.len, bytes.cap) });
            }
        }

        #[doc = #string_free_doc]
        /// # Safety
        /// `array` must have been returned by this library and not freed before.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #string_array_free(array: #string_array_type) {
            if array.ptr.is_null() {
                return;
            }
            let pointers = unsafe { Vec::from_raw_parts(array.ptr, array.len, array.cap) };
            for pointer in pointers {
                unsafe { #string_free(pointer) };
            }
        }
    }
}

fn render_c_enum(config: &Config, remote: &Remote, item: &ItemEnum) -> TokenStream {
    let crate_path = crate_path(&remote.crate_path);
    let name = format_ident!("{}{}", config.c_types_prefix, item.ident);
    let source = format_ident!("{}", remote.source);

    let variants: Vec<_> = item.variants.iter().map(|variant| &variant.ident).collect();
    let discriminants: Vec<TokenStream> = item
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| match &variant.discriminant {
            Some((_, expression)) => quote! { #expression },
            None => {
                let index = index as i32;
                quote! { #index }
            }
        })
        .collect();
    let from_arms = variants.iter().map(|variant| {
        quote! { #crate_path::#source::#variant => Self::#variant }
    });
    let repr_arms = variants.iter().map(|variant| {
        quote! { value if value == #name::#variant as i32 => Some(#crate_path::#source::#variant) }
    });

    quote! {
        #[repr(i32)]
        #[derive(Clone, Copy)]
        #[allow(dead_code)]
        pub enum #name {
            #(#variants = #discriminants,)*
        }

        #[allow(dead_code)]
        impl #name {
            pub fn to_rust(value: i32) -> Option<#crate_path::#source> {
                match value {
                    #(#repr_arms,)*
                    _ => None
                }
            }
        }

        impl From<#crate_path::#source> for #name {
            fn from(value: #crate_path::#source) -> Self {
                match value {
                    #(#from_arms,)*
                }
            }
        }
    }
}

fn render_c_error_code(config: &Config, error: &ErrorCode) -> TokenStream {
    let crate_path = crate_path(&error.crate_path);
    let source = &error.item.ident;
    let name = format_ident!("{}{}Code", config.c_types_prefix, source);

    let variants: Vec<_> = error.item.variants.iter().map(|variant| &variant.ident).collect();
    let discriminants: Vec<TokenStream> =
        (1..=variants.len() as i32).map(|value| quote! { #value }).collect();
    let from_arms = error.item.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let pattern = match &variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(_) => quote! { (..) },
            Fields::Named(_) => quote! { { .. } }
        };
        quote! { #crate_path::#source::#ident #pattern => Self::#ident }
    });

    quote! {
        #[repr(i32)]
        #[derive(Clone, Copy)]
        #[allow(dead_code)]
        pub enum #name {
            Success = 0,
            #(#variants = #discriminants,)*
            NullPointer = -2,
            InvalidArgument = -3,
            RuntimeUnavailable = -4
        }

        impl From<&#crate_path::#source> for #name {
            fn from(value: &#crate_path::#source) -> Self {
                match value {
                    #(#from_arms,)*
                }
            }
        }
    }
}

fn render_c_const(config: &Config, generated: &GeneratedConst) -> TokenStream {
    let crate_path = crate_path(&generated.crate_path);
    let function = format_ident!("{}_const_{}", config.c_prefix, generated.field.target);
    let source = &generated.field.source;
    let bytes_type = format_ident!("{}Bytes", config.c_types_prefix);
    let string_array_type = format_ident!("{}StringArray", config.c_types_prefix);

    match generated.field.ty {
        ConstType::String => quote! {
            #[unsafe(no_mangle)]
            pub extern "C" fn #function() -> *mut std::ffi::c_char {
                export_string(#crate_path::#source)
            }
        },
        ConstType::Bytes => quote! {
            #[unsafe(no_mangle)]
            pub extern "C" fn #function() -> #bytes_type {
                #bytes_type::from_vec(#crate_path::#source.to_vec())
            }
        },
        ConstType::Strings => quote! {
            #[unsafe(no_mangle)]
            pub extern "C" fn #function() -> #string_array_type {
                #string_array_type::from_strs(#crate_path::#source)
            }
        }
    }
}
