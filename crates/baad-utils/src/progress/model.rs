use std::collections::HashMap;
use std::fmt::{self, Write};
use std::path::Path;
use std::sync::Arc;

use baad_shared::{ProgressEvent, ProgressObserver, ProgressUnit};
use tracing::Level;

use crate::formatter::{AlignedLine, HumanBytes, LineFormatter};
use crate::progress::view::{ProgressModel, ProgressView};

pub trait ProgressHandler: ProgressModel {
    fn handle_event(&mut self, event: ProgressEvent);
}

struct TaskState {
    label: Arc<str>,
    unit: ProgressUnit,
    current: u64,
    total: u64
}

struct UnitValue {
    unit: ProgressUnit,
    value: u64
}

impl fmt::Display for UnitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            ProgressUnit::Bytes => HumanBytes(self.value).fmt(f),
            ProgressUnit::Count => self.value.fmt(f)
        }
    }
}

#[derive(Default)]
pub struct ProgressDisplay {
    active: HashMap<Arc<str>, TaskState>,
    order: Vec<Arc<str>>,
    formatter: LineFormatter,
    scratch: String
}

impl ProgressDisplay {
    pub fn new() -> Self { Self::default() }
}

impl ProgressHandler for ProgressDisplay {
    fn handle_event(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::Started { id, label, unit, total } => {
                if !self.active.contains_key(&id) {
                    self.order.push(Arc::clone(&id));
                }
                self.active.insert(id, TaskState {
                    label,
                    unit,
                    current: 0,
                    total
                });
            }
            ProgressEvent::Advance { id, current, total } => {
                if let Some(state) = self.active.get_mut(&id) {
                    state.current = current;
                    state.total = total;
                }
            }
            ProgressEvent::Completed { id, .. } => {
                self.active.remove(&id);
                self.order.retain(|entry| entry != &id);
            }
        }
    }
}

impl ProgressModel for ProgressDisplay {
    fn render(&mut self, width: usize, height: usize, output: &mut String) {
        let reserved = (height / 3).max(5);
        let max_visible = height.saturating_sub(reserved).max(1);

        for id in self.order.iter().take(max_visible) {
            let Some(state) = self.active.get(id) else {
                continue;
            };

            let name = Path::new(id.as_ref()).file_name().and_then(|n| n.to_str()).unwrap_or(id);

            self.scratch.clear();
            if state.total > 0 {
                let _ = write!(
                    self.scratch,
                    "{} / {}",
                    UnitValue {
                        unit: state.unit,
                        value: state.current
                    },
                    UnitValue {
                        unit: state.unit,
                        value: state.total
                    }
                );
            } else if state.current > 0 {
                let _ = write!(self.scratch, "{}", UnitValue {
                    unit: state.unit,
                    value: state.current
                });
            }

            let _ = self.formatter.write_line_aligned(output, &AlignedLine {
                level: &Level::INFO,
                is_success: false,
                message: &state.label,
                value: name,
                right: &self.scratch,
                width
            });
        }

        let hidden = self.active.len().saturating_sub(max_visible);
        if hidden > 0 {
            self.scratch.clear();
            let _ = write!(self.scratch, "…and {hidden} more");
            let _ = self.formatter.write_line(output, &Level::INFO, false, &self.scratch, &[]);
        }
    }
}

pub struct ViewObserver<M: ProgressHandler> {
    view: Arc<ProgressView<M>>
}

impl<M: ProgressHandler> ViewObserver<M> {
    pub const fn new(view: Arc<ProgressView<M>>) -> Self { Self { view } }
}

impl<M: ProgressHandler + Sync> ProgressObserver for ViewObserver<M> {
    fn on_event(&self, event: ProgressEvent) {
        let _ = self.view.update(|model| model.handle_event(event));
    }
}
