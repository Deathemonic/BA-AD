#[diplomat::bridge]
pub mod ffi {
    use crate::progress::ffi::BaadSharedProgressEvent;
    #[diplomat::opaque]
    pub struct BaadSharedObserver;

    impl BaadSharedObserver {
        pub fn set(callback: impl Fn(Box<BaadSharedProgressEvent>) + 'static) {
            unsafe {
                crate::adapter::register(move |event| {
                    callback(Box::new(BaadSharedProgressEvent(event)));
                });
            }
        }

        pub fn clear() {
            baad_shared::set_observer(std::sync::Arc::new(baad_shared::NoopObserver));
        }
    }
}
