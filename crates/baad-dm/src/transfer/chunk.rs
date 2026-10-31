use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest_middleware::reqwest::header::{ACCEPT_ENCODING, RANGE};
use tokio::task;

use crate::error::Error;
use crate::execution::RangeProgress;
use crate::transfer::{Transfer, WRITE_BUFFER_SIZE};
use crate::{client, output};

struct ChunkContext<'a, 'shared> {
    transfer: &'a Transfer<'a, 'shared>,
    temporary: Arc<ChunkTemporary>,
    progress: RangeProgress
}

struct ChunkRange {
    index: usize,
    start: u64,
    end: u64
}

struct ChunkTemporary {
    path: PathBuf
}

struct ChunkWriter {
    writer: BufWriter<File>,
    _temporary: Arc<ChunkTemporary>
}

impl Transfer<'_, '_> {
    pub async fn run_chunks(&self, total_size: u64, chunk_count: usize) -> Result<(), Error> {
        output::ensure_parent_dir(self.request.path).await?;

        let path = output::sibling_path(self.request.path, "chunks");
        let temporary =
            task::spawn_blocking(move || ChunkTemporary::new(path, total_size)).await??;

        let progress = RangeProgress::new(
            chunk_count,
            Arc::clone(&self.context.progress),
            self.request.progress_index
        );
        let context = ChunkContext {
            transfer: self,
            temporary,
            progress
        };

        stream::iter(ranges(total_size, chunk_count))
            .map(Ok::<_, Error>)
            .try_for_each_concurrent(self.context.config.max_concurrent_chunks.get(), |range| {
                context.download(range)
            })
            .await?;

        self.context.cancellation.check()?;

        fs::rename(&context.temporary.path, self.request.path)?;

        Ok(())
    }
}

impl ChunkContext<'_, '_> {
    async fn download(&self, range: ChunkRange) -> Result<(), Error> {
        self.transfer.context.cancellation.check()?;

        let _permit = self
            .transfer
            .context
            .shared
            .requests
            .acquire()
            .await
            .map_err(|_| Error::LimiterClosed)?;

        let response = self
            .transfer
            .context
            .shared
            .client
            .get(&self.transfer.server.resolved_url)
            .header(ACCEPT_ENCODING, "identity")
            .header(RANGE, client::create_range_header(range.start, Some(range.end)))
            .send()
            .await?;
        response.error_for_status_ref()?;

        let expected = client::validate_range_response(
            &response,
            range.start,
            Some(range.end),
            self.transfer.server.content_length.or(self.transfer.request.expected_size)
        )?;

        let temporary = Arc::clone(&self.temporary);
        let mut writer =
            task::spawn_blocking(move || ChunkWriter::new(temporary, range.start)).await??;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0_u64;

        while let Some(chunk) = stream.next().await {
            self.transfer.context.cancellation.check()?;

            let chunk = chunk?;
            let chunk_size = chunk.len() as u64;

            if chunk_size > expected - downloaded {
                return Err(Error::InvalidRangeResponse);
            }

            if chunk.len() < writer.writer.capacity() - writer.writer.buffer().len() {
                writer.writer.write_all(&chunk)?;
            } else {
                writer = task::spawn_blocking(move || {
                    writer.writer.write_all(&chunk)?;

                    Ok::<_, Error>(writer)
                })
                .await??;
            }

            downloaded = downloaded.saturating_add(chunk_size);
            self.progress.set_position(range.index, downloaded);
        }

        task::spawn_blocking(move || writer.finish()).await??;

        if downloaded != expected {
            return Err(Error::InvalidDownload(
                format!("range size mismatch: {downloaded} != {expected}").into()
            ));
        }

        Ok(())
    }
}

impl ChunkTemporary {
    fn new(path: PathBuf, size: u64) -> Result<Arc<Self>, Error> {
        let file = OpenOptions::new().create_new(true).write(true).open(&path)?;
        let temporary = Self { path };
        let result = file.set_len(size);
        drop(file);

        result?;

        Ok(Arc::new(temporary))
    }
}

impl Drop for ChunkTemporary {
    fn drop(&mut self) { output::remove_temporary(&self.path); }
}

impl ChunkWriter {
    fn new(temporary: Arc<ChunkTemporary>, start: u64) -> Result<Self, Error> {
        let mut file = OpenOptions::new().write(true).open(&temporary.path)?;
        file.seek(SeekFrom::Start(start))?;

        Ok(Self {
            writer: BufWriter::with_capacity(WRITE_BUFFER_SIZE, file),
            _temporary: temporary
        })
    }

    fn finish(mut self) -> io::Result<()> { self.writer.flush() }
}

fn ranges(total_size: u64, chunk_count: usize) -> impl Iterator<Item = ChunkRange> {
    let chunk_size = total_size / chunk_count as u64;

    (0..chunk_count).map(move |index| {
        let start = index as u64 * chunk_size;
        let end = if index == chunk_count - 1 {
            total_size - 1
        } else {
            (index as u64 + 1) * chunk_size - 1
        };

        ChunkRange { index, start, end }
    })
}
