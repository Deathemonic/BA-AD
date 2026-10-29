use futures::StreamExt;
use reqwest_middleware::reqwest::header::{ACCEPT_ENCODING, RANGE};
use reqwest_middleware::reqwest::{Response, StatusCode};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::client::{create_range_header, validate_identity, validate_range_response};
use crate::error::Error;
use crate::output::ensure_parent_dir;
use crate::transfer::{Transfer, WRITE_BUFFER_SIZE};

impl Transfer<'_, '_> {
    pub async fn run_stream(&self) -> Result<u64, Error> {
        self.context.cancellation.check()?;

        let _permit =
            self.context.shared.requests.acquire().await.map_err(|_| Error::LimiterClosed)?;

        let resumable = self.context.config.resumable && self.server.resumable;
        let mut request = self
            .context
            .shared
            .client
            .get(&self.server.resolved_url)
            .header(ACCEPT_ENCODING, "identity");

        if resumable && self.size_on_disk > 0 {
            request = request.header(RANGE, create_range_header(self.size_on_disk, None));
        }

        let response = request.send().await?;
        response.error_for_status_ref()?;

        ensure_parent_dir(self.request.path).await?;
        let append =
            resumable && self.size_on_disk > 0 && response.status() == StatusCode::PARTIAL_CONTENT;
        let total = self.server.content_length.or(self.request.expected_size);
        let expected = if append {
            Some(validate_range_response(&response, self.size_on_disk, None, total)?)
        } else {
            if response.status() != StatusCode::OK
                || response.headers().contains_key("content-range")
            {
                return Err(Error::UnexpectedDownloadResponse);
            }

            validate_identity(&response)?;

            total.or(response.content_length())
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(self.request.path)
            .await?;

        self.write_response(
            BufWriter::with_capacity(WRITE_BUFFER_SIZE, file),
            response,
            append,
            expected
        )
        .await
    }

    async fn write_response(
        &self,
        mut writer: BufWriter<File>,
        response: Response,
        append: bool,
        expected: Option<u64>
    ) -> Result<u64, Error> {
        let mut downloaded = if append { self.size_on_disk } else { 0 };
        self.context.progress.set_position(self.request.progress_index, downloaded);
        let mut stream = response.bytes_stream();
        let mut received = 0_u64;

        while let Some(chunk) = stream.next().await {
            self.context.cancellation.check()?;

            let chunk = chunk?;
            received =
                received.checked_add(chunk.len() as u64).ok_or(Error::ResponseSizeOverflow)?;

            if expected.is_some_and(|expected| received > expected) {
                return Err(Error::ResponseSizeMismatch);
            }

            writer.write_all(&chunk).await?;
            downloaded = downloaded.saturating_add(chunk.len() as u64);
            self.context.progress.set_position(self.request.progress_index, downloaded);
        }

        writer.flush().await?;

        if expected.is_some_and(|expected| received != expected) {
            return Err(Error::ResponseSizeMismatch);
        }

        Ok(downloaded)
    }
}
