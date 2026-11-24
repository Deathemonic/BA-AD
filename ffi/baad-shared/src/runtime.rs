use std::cell::Cell;
use std::future::Future;
use std::mem::DropGuard;
use std::sync::OnceLock;

use tokio::runtime::{Builder, Handle, Runtime};
thread_local! {
    static CALLBACK_DEPTH : Cell < usize > = const { Cell::new(0) };
}

pub fn in_callback() -> bool { CALLBACK_DEPTH.with(|depth| depth.get() != 0) }

pub(crate) fn enter_callback() -> DropGuard<(), impl FnOnce(())> {
    CALLBACK_DEPTH.with(|depth| depth.set(depth.get() + 1));
    DropGuard::new((), |()| {
        CALLBACK_DEPTH.with(|depth| depth.set(depth.get() - 1));
    })
}

pub fn block_on<T>(future: impl Future<Output = T>) -> Result<T, String> {
    if in_callback() || Handle::try_current().is_ok() {
        return Err(
            "Blocking FFI calls cannot run inside a Tokio runtime or progress callback".into()
        );
    }
    static RUNTIME: OnceLock<Result<Runtime, String>> = OnceLock::new();
    let runtime = RUNTIME.get_or_init(|| {
        Builder::new_multi_thread().enable_all().build().map_err(|e| e.to_string())
    });
    runtime.as_ref().map(|runtime| runtime.block_on(future)).map_err(Clone::clone)
}
