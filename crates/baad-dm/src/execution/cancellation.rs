use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[derive(Debug, Default)]
pub struct Cancellation {
    cancelled: AtomicBool
}

pub struct CancellationGuard {
    cancellation: Arc<Cancellation>,
    armed: bool
}

impl Cancellation {
    pub fn cancel(&self) { self.cancelled.store(true, Ordering::Release); }

    pub fn check(&self) -> Result<(), Error> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(Error::Cancelled);
        }

        Ok(())
    }
}

impl CancellationGuard {
    pub const fn new(cancellation: Arc<Cancellation>) -> Self { Self { cancellation, armed: true } }

    pub const fn disarm(&mut self) { self.armed = false; }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        if self.armed {
            self.cancellation.cancel();
        }
    }
}
