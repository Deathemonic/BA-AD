use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

use baad_core::{DownloadEvent, DownloadObserver, DownloadStatus};
use better_default::Default;
use itoa::Buffer;
use tracing::Level;

use crate::formatter::LineFormatter;
use crate::progress::view::{ProgressModel, ProgressView};

struct FileState {
    downloaded_bytes: u64,
    total_bytes: u64
}

#[derive(Default)]
#[default(formatter: LineFormatter::new().with_timestamps(false))]
pub struct DownloadProgressModel {
    active: HashMap<Arc<str>, FileState>,
    completed_count: usize,
    skipped_count: usize,
    failed_count: usize,
    total_bytes_downloaded: u64,
    formatter: LineFormatter
}

impl DownloadProgressModel {
    pub fn new() -> Self { Self::default() }

    pub fn handle_event(&mut self, event: DownloadEvent) {
        match event {
            DownloadEvent::Started { filename, total_bytes } => {
                self.active.insert(filename, FileState {
                    downloaded_bytes: 0,
                    total_bytes
                });
            }
            DownloadEvent::Progress {
                filename,
                downloaded_bytes,
                total_bytes
            } => {
                if let Some(state) = self.active.get_mut(&filename) {
                    state.downloaded_bytes = downloaded_bytes;
                    state.total_bytes = total_bytes;
                }
            }
            DownloadEvent::Completed { filename, size, status } => {
                self.active.remove(&filename);
                self.total_bytes_downloaded += size;
                match status {
                    DownloadStatus::Success => self.completed_count += 1,
                    DownloadStatus::Skipped => self.skipped_count += 1,
                    DownloadStatus::Failed(_) | DownloadStatus::HashMismatch => {
                        self.failed_count += 1;
                    }
                }
            }
        }
    }

    fn format_summary(&self, buffer: &mut Buffer) -> String {
        let active_count = self.active.len();
        let mut message = String::new();

        let _ = write!(message, "downloading {} files", buffer.format(active_count));

        let _ = write!(message, " ({} done", buffer.format(self.completed_count));

        if self.skipped_count > 0 {
            let _ = write!(message, ", {} skipped", buffer.format(self.skipped_count));
        }

        let _ = write!(message, ", {} failed)", buffer.format(self.failed_count));

        message
    }
}

impl ProgressModel for DownloadProgressModel {
    fn render(&mut self, _width: usize, output: &mut String) {
        if self.active.is_empty() {
            return;
        }

        let mut buffer = Buffer::new();
        let message = self.format_summary(&mut buffer);
        let _ = self.formatter.write_simple_message(output, &Level::INFO, false, &message);
    }

    fn final_message(&mut self, output: &mut String) {
        let mut buffer = Buffer::new();
        let total = self.completed_count + self.skipped_count + self.failed_count;

        let mut message = String::new();
        let _ = write!(message, "{} files processed", buffer.format(total));
        let _ = write!(message, " ({} done", buffer.format(self.completed_count));

        if self.skipped_count > 0 {
            let _ = write!(message, ", {} skipped", buffer.format(self.skipped_count));
        }

        let _ = write!(message, ", {} failed)", buffer.format(self.failed_count));

        let is_success = self.failed_count == 0;
        let _ = self.formatter.write_line(output, &Level::INFO, is_success, &message, &[]);
    }
}

pub struct DownloadProgressObserver {
    view: Arc<ProgressView<DownloadProgressModel>>
}

impl DownloadProgressObserver {
    pub fn new(view: Arc<ProgressView<DownloadProgressModel>>) -> Self { Self { view } }
}

impl DownloadObserver for DownloadProgressObserver {
    fn on_event(&self, event: DownloadEvent) {
        let _ = self.view.update(|model| model.handle_event(event));
    }
}
