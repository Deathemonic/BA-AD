use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;

use baad_shared::{DownloadEvent, DownloadObserver, DownloadStatus};
use better_default::Default;
use tracing::Level;

use crate::formatter::{HumanBytes, LineFormatter};
use crate::progress::view::{ProgressModel, ProgressView};

pub trait DownloadProgressHandler: ProgressModel {
    fn handle_event(&mut self, event: DownloadEvent);
}

struct FileState {
    downloaded_bytes: u64,
    total_bytes: u64
}

#[derive(Default)]
pub struct DownloadProgressModel {
    active: HashMap<Arc<str>, FileState>,
    completed: Vec<Arc<str>>,
    formatter: LineFormatter,
    scratch: String
}

impl DownloadProgressModel {
    pub fn new() -> Self { Self::default() }
}

impl DownloadProgressHandler for DownloadProgressModel {
    fn handle_event(&mut self, event: DownloadEvent) {
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
            DownloadEvent::Completed { filename, size: _, status } => {
                self.active.remove(&filename);
                if matches!(status, DownloadStatus::Success) {
                    self.completed.push(filename);
                }
            }
        }
    }
}

impl ProgressModel for DownloadProgressModel {
    fn render(&mut self, width: usize, height: usize, output: &mut String) {
        let reserved = (height / 3).max(5);
        let max_visible = height.saturating_sub(reserved).max(1);

        for (filename, state) in self.active.iter().take(max_visible) {
            let name = Path::new(filename.as_ref())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(filename);

            self.scratch.clear();
            if state.total_bytes > 0 {
                let _ = write!(
                    self.scratch,
                    "{} / {}",
                    HumanBytes(state.downloaded_bytes),
                    HumanBytes(state.total_bytes)
                );
            } else if state.downloaded_bytes > 0 {
                let _ = write!(self.scratch, "{}", HumanBytes(state.downloaded_bytes));
            }

            let _ = self.formatter.write_line_aligned(
                output,
                &Level::INFO,
                false,
                "Downloading",
                name,
                &self.scratch,
                width
            );
        }

        let hidden = self.active.len().saturating_sub(max_visible);
        if hidden > 0 {
            self.scratch.clear();
            let _ = write!(self.scratch, "…and {hidden} more");
            let _ = self.formatter.write_line(output, &Level::INFO, false, &self.scratch, &[]);
        }
    }

    fn final_message(&mut self, output: &mut String) {
        for filename in &self.completed {
            let _ = self
                .formatter
                .write_line(output, &Level::INFO, true, "Downloaded", &[("file", filename)]);
        }
    }
}

pub struct ProgressObserver<M: DownloadProgressHandler> {
    view: Arc<ProgressView<M>>
}

impl<M: DownloadProgressHandler> ProgressObserver<M> {
    pub const fn new(view: Arc<ProgressView<M>>) -> Self { Self { view } }
}

impl<M: DownloadProgressHandler + Sync> DownloadObserver for ProgressObserver<M> {
    fn on_event(&self, event: DownloadEvent) {
        let _ = self.view.update(|model| model.handle_event(event));
    }
}
