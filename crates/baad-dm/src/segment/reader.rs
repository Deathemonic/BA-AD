use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

const IO_BUFFER_SIZE: usize = 256 * 1024;

pub struct OrderedSegmentReader {
    paths: Vec<PathBuf>,
    index: usize,
    current: Option<BufReader<File>>
}

impl OrderedSegmentReader {
    pub const fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            index: 0,
            current: None
        }
    }
}

impl Read for OrderedSegmentReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        loop {
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
