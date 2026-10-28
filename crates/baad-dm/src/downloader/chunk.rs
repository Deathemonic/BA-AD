use std::path::Path;
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest_middleware::reqwest::StatusCode;
use reqwest_middleware::reqwest::header::RANGE;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter, SeekFrom};

use crate::client::create_range_header;
use crate::downloader::helpers::ensure_parent_dir;
use crate::downloader::progress::RangeProgress;
use crate::downloader::transfer::{Transfer, WRITE_BUFFER_SIZE};
use crate::error::Error;

struct ChunkContext<'a, 'shared> {
    transfer: &'a Transfer<'a, 'shared>,
    path: &'a Path,
    progress: RangeProgress
}

struct ChunkRange {
    index: usize,
    start: u64,
    end: u64
}

impl Transfer<'_, '_> {
    pub async fn run_chunks(&self, total_size: u64, chunk_count: usize) -> Result<(), Error> {
        ensure_parent_dir(self.request.path).await?;
        let path = self.request.path.with_added_extension("chunks");
        let file = File::create(&path).await?;
        file.set_len(total_size).await?;
        drop(file);

        let progress = RangeProgress::new(
            chunk_count,
            Arc::clone(&self.context.progress),
            self.request.progress_index
        );
        let context = ChunkContext {
            transfer: self,
            path: &path,
            progress
        };
        stream::iter(ranges(total_size, chunk_count))
            .map(Ok::<_, Error>)
            .try_for_each_concurrent(self.context.config.max_concurrent_chunks.get(), |range| {
                context.download(range)
            })
            .await?;

        fs::rename(&path, self.request.path).await?;

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
            .header(RANGE, create_range_header(range.start, Some(range.end)))
            .send()
            .await?;
        response.error_for_status_ref()?;

        if response.status() != StatusCode::PARTIAL_CONTENT {
            return Err(Error::RangeNotSupported);
        }

        let mut file = OpenOptions::new().write(true).open(self.path).await?;
        file.seek(SeekFrom::Start(range.start)).await?;
        let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, file);
        let mut stream = response.bytes_stream();
        let mut downloaded = 0_u64;
        let expected = range.end - range.start + 1;

        while let Some(chunk) = stream.next().await {
            self.transfer.context.cancellation.check()?;
            let chunk = chunk?;
            if chunk.len() as u64 > expected - downloaded {
                return Err(Error::InvalidDownload("range response exceeds requested size".into()));
            }
            writer.write_all(&chunk).await?;
            downloaded = downloaded.saturating_add(chunk.len() as u64);
            self.progress.set_position(range.index, downloaded);
        }

        writer.flush().await?;

        if downloaded != expected {
            return Err(Error::InvalidDownload(
                format!("range size mismatch: {downloaded} != {expected}").into()
            ));
        }

        Ok(())
    }
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
