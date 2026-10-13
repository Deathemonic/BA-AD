use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use baad_shared::{ProgressEvent, ProgressObserver};
use derive_more::Debug;

const UPDATE_INTERVAL: Duration = Duration::from_millis(50);

struct ProgressState {
    positions: Vec<u64>,
    last_position: u64,
    last_update: Option<Instant>
}

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
    positions: Mutex<Vec<u64>>,
    parent: Arc<ProgressTracker>,
    parent_index: usize
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
                positions: vec![0; limits.len()],
                last_position: 0,
                last_update: None
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
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(current) = state.positions.get_mut(index) else {
            return;
        };
        let position = self
            .limits
            .get(index)
            .and_then(|limit| *limit)
            .map_or(position, |limit| position.min(limit));
        if *current == position && !force {
            return;
        }
        *current = position;
        let downloaded = state.positions.iter().copied().fold(0_u64, u64::saturating_add);
        let downloaded =
            if self.total_bytes > 0 { downloaded.min(self.total_bytes) } else { downloaded };
        let now = Instant::now();
        let elapsed =
            state.last_update.is_none_or(|last| now.duration_since(last) >= UPDATE_INTERVAL);
        let complete = self.total_bytes > 0 && downloaded == self.total_bytes;
        let reset = downloaded < state.last_position;
        if !force && !complete && !reset && !elapsed {
            return;
        }

        state.last_position = downloaded;
        state.last_update = Some(now);
        self.observer.on_event(ProgressEvent::Advance {
            id: Arc::clone(&self.filename),
            current: downloaded,
            total: self.total_bytes
        });
    }
}

impl RangeProgress {
    pub fn new(count: usize, parent: Arc<ProgressTracker>, parent_index: usize) -> Self {
        Self {
            positions: Mutex::new(vec![0; count]),
            parent,
            parent_index
        }
    }

    pub fn set_position(&self, index: usize, position: u64) {
        let mut positions = self.positions.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(current) = positions.get_mut(index) else {
            return;
        };
        *current = position;
        let position = positions.iter().copied().fold(0_u64, u64::saturating_add);
        self.parent.set_position(self.parent_index, position);
    }
}
