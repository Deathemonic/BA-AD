use std::io::{self, ErrorKind, Read, Write};
use std::sync::Arc;

use crate::error::Error;
use crate::execution::Cancellation;

pub struct CheckedWriter<W> {
    inner: W,
    pub expected: Option<u64>,
    written: u64,
    pub exceeded: bool,
    cancellation: Arc<Cancellation>
}

pub struct CheckedReader<'a, R> {
    pub inner: &'a mut R,
    pub cancellation: &'a Cancellation
}

impl<W> CheckedWriter<W> {
    pub const fn new(inner: W, expected: Option<u64>, cancellation: Arc<Cancellation>) -> Self {
        Self {
            inner,
            expected,
            written: 0,
            exceeded: false,
            cancellation
        }
    }

    pub const fn validate_size(&self) -> Result<(), Error> {
        if let Some(expected) = self.expected
            && self.written != expected
        {
            return Err(Error::OutputSizeMismatch {
                expected,
                actual: self.written
            });
        }

        Ok(())
    }
}

impl<R: Read> Read for CheckedReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.cancellation.check().map_err(|_| io::Error::from(ErrorKind::Other))?;

        self.inner.read(buffer)
    }
}

impl<W: Write> Write for CheckedWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.cancellation.check().map_err(|_| io::Error::from(ErrorKind::Other))?;

        if let Some(expected) = self.expected
            && self.written.saturating_add(buffer.len() as u64) > expected
        {
            self.exceeded = true;

            return Err(ErrorKind::InvalidData.into());
        }

        let written = self.inner.write(buffer)?;
        self.written += written as u64;

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> { self.inner.flush() }
}
