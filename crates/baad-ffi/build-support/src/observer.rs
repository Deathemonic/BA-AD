use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::config::Config;

pub fn render_observer_uniffi() -> TokenStream {
    quote! {
        type ObserverArc<T> = std::sync::Arc<T>;

        #[uniffi::export(with_foreign)]
        pub trait ProgressObserver: Send + Sync {
            fn on_event(&self, event: ProgressEvent);
        }

        struct ForeignObserverAdapter(ObserverArc<dyn ProgressObserver>);

        impl baad_shared::ProgressObserver for ForeignObserverAdapter {
            fn on_event(&self, event: baad_shared::ProgressEvent) {
                self.0.on_event(event.into());
            }
        }

        #[uniffi::export]
        pub fn set_observer(observer: ObserverArc<dyn ProgressObserver>) {
            crate::api::observer::register_observer(ObserverArc::new(ForeignObserverAdapter(
                observer
            )));
        }

        #[uniffi::export]
        pub fn clear_observer() { crate::api::observer::unregister_observer(); }
    }
}

pub fn render_observer_dispatch() -> TokenStream {
    quote! {
        use std::sync::{Arc, RwLock};

        static FOREIGN_OBSERVER: RwLock<Option<Arc<dyn baad_shared::ProgressObserver>>> =
            RwLock::new(None);

        struct ObserverDispatcher;

        impl baad_shared::ProgressObserver for ObserverDispatcher {
            fn on_event(&self, event: baad_shared::ProgressEvent) {
                let Ok(guard) = FOREIGN_OBSERVER.read() else {
                    return;
                };

                if let Some(observer) = guard.as_ref() {
                    observer.on_event(event);
                }
            }
        }

        pub(crate) fn register_observer(observer: Arc<dyn baad_shared::ProgressObserver>) {
            baad_shared::set_observer(Arc::new(ObserverDispatcher));

            if let Ok(mut guard) = FOREIGN_OBSERVER.write() {
                *guard = Some(observer);
            }
        }

        pub(crate) fn unregister_observer() {
            if let Ok(mut guard) = FOREIGN_OBSERVER.write() {
                *guard = None;
            }
        }
    }
}

pub fn render_observer_c(config: &Config) -> TokenStream {
    let types = render_observer_c_types(config);
    let dispatch = render_observer_c_dispatch(config);
    let set_observer = format_ident!("{}_set_observer", config.c_prefix);
    let clear_observer = format_ident!("{}_clear_observer", config.c_prefix);
    let callback = format_ident!("{}ObserverCallback", config.c_types_prefix);

    quote! {
        #types
        #dispatch

        #[unsafe(no_mangle)]
        pub extern "C" fn #set_observer(callback: #callback, user_data: *mut std::ffi::c_void) {
            crate::api::observer::register_observer(std::sync::Arc::new(CallbackObserver {
                callback,
                user_data
            }));
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn #clear_observer() { crate::api::observer::unregister_observer(); }
    }
}

fn render_observer_c_types(config: &Config) -> TokenStream {
    let event_kind = format_ident!("{}EventKind", config.c_types_prefix);
    let status = format_ident!("{}ProgressStatus", config.c_types_prefix);
    let event = format_ident!("{}ProgressEvent", config.c_types_prefix);
    let callback = format_ident!("{}ObserverCallback", config.c_types_prefix);

    quote! {
        #[repr(i32)]
        #[derive(Clone, Copy)]
        pub enum #event_kind {
            Started = 0,
            Advance = 1,
            Completed = 2
        }

        #[repr(i32)]
        #[derive(Clone, Copy)]
        pub enum #status {
            NotStarted = 0,
            Success = 1,
            Skipped = 2,
            Failed = 3,
            HashMismatch = 4
        }

        #[repr(C)]
        pub struct #event {
            pub kind: i32,
            pub id: *const std::ffi::c_char,
            pub label: *const std::ffi::c_char,
            pub unit: i32,
            pub current: u64,
            pub total: u64,
            pub status: i32,
            pub reason: *const std::ffi::c_char
        }

        pub type #callback = extern "C" fn(user_data: *mut std::ffi::c_void, event: *const #event);

        struct CallbackObserver {
            callback: #callback,
            user_data: *mut std::ffi::c_void
        }

        /// SAFETY: events fire from arbitrary worker threads; registering a
        /// callback asserts `callback` and `user_data` are thread-safe.
        unsafe impl Send for CallbackObserver {}
        unsafe impl Sync for CallbackObserver {}
    }
}

fn render_observer_c_dispatch(config: &Config) -> TokenStream {
    let event_kind = format_ident!("{}EventKind", config.c_types_prefix);
    let status = format_ident!("{}ProgressStatus", config.c_types_prefix);
    let event = format_ident!("{}ProgressEvent", config.c_types_prefix);

    quote! {
        impl baad_shared::ProgressObserver for CallbackObserver {
            fn on_event(&self, event: baad_shared::ProgressEvent) {
                let (kind, id, label, unit, current, total, status, reason) = match event {
                    baad_shared::ProgressEvent::Started { id, label, unit, total } => {
                        let unit = match unit {
                            baad_shared::ProgressUnit::Bytes => 0,
                            baad_shared::ProgressUnit::Count => 1
                        };
                        (
                            #event_kind::Started,
                            id,
                            Some(label),
                            unit,
                            0,
                            total,
                            #status::NotStarted,
                            None
                        )
                    }
                    baad_shared::ProgressEvent::Advance { id, current, total } => (
                        #event_kind::Advance,
                        id,
                        None,
                        0,
                        current,
                        total,
                        #status::NotStarted,
                        None
                    ),
                    baad_shared::ProgressEvent::Completed { id, status } => {
                        let (status, reason) = match status {
                            baad_shared::ProgressStatus::NotStarted => (#status::NotStarted, None),
                            baad_shared::ProgressStatus::Success => (#status::Success, None),
                            baad_shared::ProgressStatus::Skipped(reason) => {
                                (#status::Skipped, Some(reason))
                            }
                            baad_shared::ProgressStatus::Failed(reason) => {
                                (#status::Failed, Some(reason))
                            }
                            baad_shared::ProgressStatus::HashMismatch(reason) => {
                                (#status::HashMismatch, Some(reason))
                            }
                        };
                        (#event_kind::Completed, id, None, 0, 0, 0, status, reason)
                    }
                };

                let Ok(id) = std::ffi::CString::new(id.as_ref()) else {
                    return;
                };
                let label = label.and_then(|label| std::ffi::CString::new(label).ok());
                let reason = reason.and_then(|reason| std::ffi::CString::new(reason.as_ref()).ok());

                let event = #event {
                    kind: kind as i32,
                    id: id.as_ptr(),
                    label: label.as_ref().map_or(std::ptr::null(), |label| label.as_ptr()),
                    unit,
                    current,
                    total,
                    status: status as i32,
                    reason: reason.as_ref().map_or(std::ptr::null(), |reason| reason.as_ptr())
                };
                (self.callback)(self.user_data, &event);
            }
        }
    }
}
