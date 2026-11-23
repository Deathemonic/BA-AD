#[diplomat::bridge]
pub mod ffi {
    use std::sync::Arc;

    use crate::adapter::register;
    use crate::progress::ffi::BaadSharedProgressEvent;

    #[diplomat::opaque]
    pub struct BaadSharedObserver;

    impl BaadSharedObserver {
        pub fn set(callback: impl Fn(Box<BaadSharedProgressEvent>) + 'static) {
            unsafe {
                register(move |event| {
                    callback(Box::new(BaadSharedProgressEvent(event)));
                });
            }
        }

        pub fn clear() { baad_shared::set_observer(Arc::new(baad_shared::NoopObserver)); }
    }
}
