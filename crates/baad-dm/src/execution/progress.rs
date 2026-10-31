use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use baad_shared::{ProgressEvent, ProgressObserver};
use derive_more::Debug;

const UPDATE_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub struct ProgressTracker {
    filename: Arc<str>,
    total_bytes: u64,
    limits: Vec<Option<u64>>,
    #[debug(skip)]
    state: Mutex<ProgressState>,
    #[debug(skip)]
    observer: Arc<dyn ProgressObserver>
}

pub struct RangeProgress {
    positions: Mutex<Positions>,
    parent: Arc<ProgressTracker>,
    parent_index: usize
}

struct Positions {
    values: Vec<u64>,
    total: u128
}

struct ProgressState {
    positions: Positions,
    last_position: u64,
    last_update: Option<Instant>,
    pending: Option<u64>,
    notifying: bool
}

impl ProgressTracker {
    pub fn new(
        filename: Arc<str>,
        total_bytes: u64,
        limits: Vec<Option<u64>>,
        observer: Arc<dyn ProgressObserver>
    ) -> Self {
        Self {
            filename,
            total_bytes,
            state: Mutex::new(ProgressState {
                positions: Positions::new(limits.len()),
                last_position: 0,
                last_update: None,
                pending: None,
                notifying: false
            }),
            limits,
            observer
        }
    }

    pub fn set_position(&self, index: usize, position: u64) { self.update(index, position, false); }

    pub fn complete_position(&self, index: usize, position: u64) {
        self.update(index, position, true);
    }

    fn update(&self, index: usize, position: u64, force: bool) {
        if self.prepare_update(index, position, force) {
            self.notify();
        }
    }

    fn prepare_update(&self, index: usize, position: u64, force: bool) -> bool {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);

        let position = self
            .limits
            .get(index)
            .and_then(|limit| *limit)
            .map_or(position, |limit| position.min(limit));

        let Some(changed) = state.positions.set(index, position) else {
            return false;
        };

        if !changed && !force {
            return false;
        }

        let downloaded = state.positions.total();
        let downloaded =
            if self.total_bytes > 0 { downloaded.min(self.total_bytes) } else { downloaded };
        let now = Instant::now();
        let elapsed =
            state.last_update.is_none_or(|last| now.duration_since(last) >= UPDATE_INTERVAL);
        let complete = self.total_bytes > 0 && downloaded == self.total_bytes;
        let reset = downloaded < state.last_position;

        if !force && !complete && !reset && !elapsed {
            return false;
        }

        state.last_position = downloaded;
        state.last_update = Some(now);
        state.pending = Some(downloaded);

        if state.notifying {
            return false;
        }

        state.notifying = true;

        true
    }

    fn notify(&self) {
        let _notification = scopeguard::guard_on_unwind((), |_| {
            self.state.lock().unwrap_or_else(PoisonError::into_inner).notifying = false;
        });

        loop {
            let current = {
                let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);

                let Some(current) = state.pending.take() else {
                    state.notifying = false;

                    return;
                };

                current
            };

            self.observer.on_event(ProgressEvent::Advance {
                id: Arc::clone(&self.filename),
                current,
                total: self.total_bytes
            });
        }
    }
}

impl RangeProgress {
    pub fn new(count: usize, parent: Arc<ProgressTracker>, parent_index: usize) -> Self {
        Self {
            positions: Mutex::new(Positions::new(count)),
            parent,
            parent_index
        }
    }

    pub fn set_position(&self, index: usize, position: u64) {
        let mut positions = self.positions.lock().unwrap_or_else(PoisonError::into_inner);

        if positions.set(index, position).is_none() {
            return;
        }

        let notify = self.parent.prepare_update(self.parent_index, positions.total(), false);
        drop(positions);

        if notify {
            self.parent.notify();
        }
    }
}

impl Positions {
    fn new(count: usize) -> Self {
        Self {
            values: vec![0; count],
            total: 0
        }
    }

    fn set(&mut self, index: usize, position: u64) -> Option<bool> {
        let current = self.values.get_mut(index)?;
        let changed = *current != position;
        self.total = self.total - u128::from(*current) + u128::from(position);
        *current = position;

        Some(changed)
    }

    fn total(&self) -> u64 { self.total.min(u128::from(u64::MAX)) as u64 }
}
