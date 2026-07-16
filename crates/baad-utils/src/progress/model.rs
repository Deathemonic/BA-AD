use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use baad_shared::{DownloadEvent, DownloadObserver, DownloadStatus};
use better_default::Default;
use tracing::Level;

use crate::formatter::LineFormatter;
use crate::progress::view::{ProgressModel, ProgressView};

struct FileState {
    downloaded_bytes: u64,
    total_bytes: u64
}

#[derive(Default)]
pub struct DownloadProgressModel {
    active: HashMap<Arc<str>, FileState>,
    completed: Vec<Arc<str>>,
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
    fn render(&mut self, _width: usize, output: &mut String) {
        for filename in self.active.keys() {
            let name = Path::new(filename.as_ref())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(filename);
            let _ = self
                .formatter
                .write_line(output, &Level::INFO, false, "Downloading", &[("file", name)]);
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

pub struct DownloadProgressObserver {
    view: Arc<ProgressView<DownloadProgressModel>>
}

impl DownloadProgressObserver {
    pub const fn new(view: Arc<ProgressView<DownloadProgressModel>>) -> Self { Self { view } }
}

impl DownloadObserver for DownloadProgressObserver {
    fn on_event(&self, event: DownloadEvent) {
        let _ = self.view.update(|model| model.handle_event(event));
    }
}
