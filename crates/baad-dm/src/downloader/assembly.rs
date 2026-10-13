use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::mem;
use std::path::{Path, PathBuf};
use std::process::id as process_id;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tracing::warn;

use crate::download::OutputTransform;
use crate::downloader::helpers::{Cancellation, verify_file};
use crate::error::Error;

const IO_BUFFER_SIZE: usize = 256 * 1024;
static TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

pub struct OutputJob {
    pub paths: Vec<PathBuf>,
    pub destination: PathBuf,
    pub transform: Option<Arc<dyn OutputTransform>>,
    pub expected_size: Option<u64>,
    pub expected_hash: Option<String>,
    pub cancellation: Arc<Cancellation>
}

pub struct OrderedSegmentReader {
    paths: Vec<PathBuf>,
    index: usize,
    current: Option<BufReader<File>>,
    cancellation: Arc<Cancellation>
}

struct CheckedWriter<W> {
    inner: W,
    expected: Option<u64>,
    written: u64,
    cancellation: Arc<Cancellation>
}

impl OutputJob {
    pub fn run(mut self) -> Result<u64, Error> {
        self.cancellation.check()?;

        let temporary = sibling_path(&self.destination, "baad-tmp");
        let result = self.write_temporary(&temporary);

        if let Err(error) = result {
            remove_temporary(&temporary);
            return Err(error);
        }

        if !verify_file(&temporary, self.expected_size, self.expected_hash.as_ref())? {
            remove_temporary(&temporary);
            return Err(Error::HashMismatch {
                expected: self
                    .expected_hash
                    .unwrap_or_else(|| "expected output metadata".into())
                    .into(),
                actual: "output did not match".into()
            });
        }

        self.publish(&temporary)
    }

    pub fn publish(&self, temporary: &Path) -> Result<u64, Error> {
        self.cancellation.check()?;

        publish_output(temporary, &self.destination)?;
        Ok(fs::metadata(&self.destination)?.len())
    }

    fn write_temporary(&mut self, temporary: &Path) -> Result<(), Error> {
        let file = OpenOptions::new().create_new(true).write(true).open(temporary)?;
        let writer = BufWriter::with_capacity(IO_BUFFER_SIZE, file);
        let mut output =
            CheckedWriter::new(writer, self.expected_size, Arc::clone(&self.cancellation));
        let paths = mem::take(&mut self.paths);
        let mut input = OrderedSegmentReader::new(paths, Arc::clone(&self.cancellation));

        match &self.transform {
            Some(transform) => {
                transform.transform(&mut input, &mut output).map_err(Error::Transform)?;
            }
            None => {
                io::copy(&mut input, &mut output)?;
            }
        }

        output.flush()?;

        output.validate_size()
    }
}

impl OrderedSegmentReader {
    pub const fn new(paths: Vec<PathBuf>, cancellation: Arc<Cancellation>) -> Self {
        Self {
            paths,
            index: 0,
            current: None,
            cancellation
        }
    }
}

impl Read for OrderedSegmentReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        loop {
            self.cancellation.check().map_err(io::Error::other)?;
            if self.current.is_none() {
                let Some(path) = self.paths.get(self.index) else {
                    return Ok(0);
                };
                self.current = Some(BufReader::with_capacity(IO_BUFFER_SIZE, File::open(path)?));
            }

            let Some(current) = self.current.as_mut() else {
                continue;
            };
            let read = current.read(buffer)?;

            if read > 0 {
                return Ok(read);
            }

            self.current = None;
            self.index += 1;
        }
    }
}

impl<W> CheckedWriter<W> {
    const fn new(inner: W, expected: Option<u64>, cancellation: Arc<Cancellation>) -> Self {
        Self {
            inner,
            expected,
            written: 0,
            cancellation
        }
    }

    fn validate_size(&self) -> Result<(), Error> {
        if let Some(expected) = self.expected
            && self.written != expected
        {
            return Err(Error::InvalidDownload(
                format!("output size mismatch: {} != {expected}", self.written).into()
            ));
        }
        Ok(())
    }
}

impl<W: Write> Write for CheckedWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.cancellation.check().map_err(io::Error::other)?;
        if let Some(expected) = self.expected
            && self.written.saturating_add(buffer.len() as u64) > expected
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "output exceeds expected size"));
        }
        let written = self.inner.write(buffer)?;
        self.written += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> { self.inner.flush() }
}

fn publish_output(temporary: &Path, destination: &Path) -> Result<(), Error> {
    match fs::rename(temporary, destination) {
        Ok(()) => return Ok(()),
        Err(error) if !destination.exists() => return Err(error.into()),
        Err(_) => {}
    }

    let backup = sibling_path(destination, "baad-backup");
    fs::rename(destination, &backup)?;
    match fs::rename(temporary, destination) {
        Ok(()) => {
            if let Err(error) = fs::remove_file(&backup) {
                warn!(path = %backup.display(), cause = %error, "Failed to remove replaced output backup");
            }
            Ok(())
        }
        Err(error) => {
            if let Err(restore_error) = fs::rename(&backup, destination) {
                return Err(Error::InvalidDownload(
                    format!("publish failed: {error}; restore failed: {restore_error}").into()
                ));
            }
            Err(error.into())
        }
    }
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
