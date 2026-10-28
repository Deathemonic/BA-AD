use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::id as process_id;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use bytes::Bytes;
use tokio::task::spawn_blocking;
use tracing::warn;

use crate::download::{Download, OutputTransform};
use crate::downloader::helpers::{Cancellation, SharedExecution, verify_file};
use crate::error::Error;

const IO_BUFFER_SIZE: usize = 256 * 1024;
static TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

pub struct OutputJob {
    pub destination: PathBuf,
    pub transform: Option<Arc<dyn OutputTransform>>,
    pub expected_size: Option<u64>,
    pub expected_hash: Option<String>,
    pub cancellation: Arc<Cancellation>
}

struct CheckedWriter<W> {
    inner: W,
    expected: Option<u64>,
    written: u64,
    exceeded: bool,
    cancellation: Arc<Cancellation>
}

struct CheckedReader<'a, R> {
    inner: &'a mut R,
    cancellation: &'a Cancellation
}

impl OutputJob {
    pub fn new(download: &Download, destination: PathBuf, cancellation: Arc<Cancellation>) -> Self {
        Self {
            destination,
            transform: download.transform.clone(),
            expected_size: download.size,
            expected_hash: download.hash.clone(),
            cancellation
        }
    }

    pub async fn finish_download(
        self,
        shared: &SharedExecution,
        path: PathBuf,
        lock: Arc<File>
    ) -> Result<u64, Error> {
        let limiter = if self.transform.is_some() { &shared.transforms } else { &shared.blocking };
        let permit = limiter.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;

        spawn_blocking(move || {
            let _permit = permit;
            let _lock = lock;
            if self.transform.is_none() {
                return self.publish(&path);
            }

            let mut input = BufReader::with_capacity(IO_BUFFER_SIZE, File::open(&path)?);
            let size = self.run(&mut input)?;
            drop(input);
            remove_temporary(&path);
            Ok(size)
        })
        .await?
    }

    pub async fn finish_bytes(self, shared: &SharedExecution, data: Bytes) -> Result<u64, Error> {
        let permit =
            shared.blocking.clone().acquire_owned().await.map_err(|_| Error::LimiterClosed)?;
        spawn_blocking(move || {
            let _permit = permit;
            self.run(&mut data.as_ref())
        })
        .await?
    }

    pub fn run(self, input: &mut impl Read) -> Result<u64, Error> {
        self.cancellation.check()?;

        let temporary = sibling_path(&self.destination, "baad-tmp");
        let file = OpenOptions::new().create_new(true).write(true).open(&temporary)?;
        let result = (|| {
            self.write_temporary(file, input)?;
            self.verify(&temporary)?;
            self.publish(&temporary)
        })();

        if result.is_err() {
            remove_temporary(&temporary);
        }

        result
    }

    pub fn publish(&self, temporary: &Path) -> Result<u64, Error> {
        self.cancellation.check()?;

        let size = fs::metadata(temporary)?.len();
        fs::rename(temporary, &self.destination)?;
        Ok(size)
    }

    fn verify(&self, path: &Path) -> Result<(), Error> {
        if !verify_file(path, self.expected_size, self.expected_hash.as_ref())? {
            return Err(Error::OutputVerificationFailed {
                expected_size: self.expected_size,
                expected_hash: self.expected_hash.as_deref().map(Into::into)
            });
        }
        Ok(())
    }

    fn write_temporary(&self, file: File, input: &mut impl Read) -> Result<(), Error> {
        let writer = BufWriter::with_capacity(IO_BUFFER_SIZE, file);
        let mut output =
            CheckedWriter::new(writer, self.expected_size, Arc::clone(&self.cancellation));
        let mut input = CheckedReader {
            inner: input,
            cancellation: &self.cancellation
        };

        let result = match &self.transform {
            Some(transform) => {
                transform.transform(&mut input, &mut output).map_err(Error::Transform)
            }
            None => io::copy(&mut input, &mut output).map(|_| ()).map_err(Error::Io)
        };

        self.cancellation.check()?;
        if output.exceeded
            && let Some(expected) = output.expected
        {
            return Err(Error::OutputSizeExceeded { expected });
        }
        result?;

        output.flush()?;
        output.validate_size()
    }
}

impl<W> CheckedWriter<W> {
    const fn new(inner: W, expected: Option<u64>, cancellation: Arc<Cancellation>) -> Self {
        Self {
            inner,
            expected,
            written: 0,
            exceeded: false,
            cancellation
        }
    }

    const fn validate_size(&self) -> Result<(), Error> {
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

fn remove_temporary(path: &Path) {
    if let Err(error) = fs::remove_file(path)
        && error.kind() != io::ErrorKind::NotFound
    {
        warn!(path = %path.display(), cause = %error, "Failed to remove temporary output");
    }
}

fn sibling_path(destination: &Path, marker: &str) -> PathBuf {
    let id = TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
    let name = destination.file_name().and_then(|name| name.to_str()).unwrap_or("download");
    destination.with_file_name(format!(".{name}.{marker}-{}-{id}", process_id()))
}
