use std::cell::Cell;
thread_local! { static CALLBACK_DEPTH: Cell<usize> = const { Cell::new(0) }; }

pub fn in_callback() -> bool { CALLBACK_DEPTH.with(|depth| depth.get() != 0) }

pub(crate) struct CallbackScope;
impl CallbackScope {
    pub(crate) fn enter() -> Self {
        CALLBACK_DEPTH.with(|depth| depth.set(depth.get() + 1));
        Self
    }
}
impl Drop for CallbackScope {
    fn drop(&mut self) { CALLBACK_DEPTH.with(|depth| depth.set(depth.get() - 1)); }
}

use std::sync::OnceLock;

pub fn block_on<T>(future: impl std::future::Future<Output = T>) -> Result<T, String> {
    if in_callback() || tokio::runtime::Handle::try_current().is_ok() {
        return Err(
            "Blocking FFI calls cannot run inside a Tokio runtime or progress callback".into()
        );
    }
    static RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();
    let runtime = RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread().enable_all().build().map_err(|e| e.to_string())
    });
    runtime.as_ref().map(|runtime| runtime.block_on(future)).map_err(Clone::clone)
}
