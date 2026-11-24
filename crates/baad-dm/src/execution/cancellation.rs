use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[derive(Debug, Default)]
pub struct Cancellation {
    cancelled: AtomicBool
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
