use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use baad_utils::config::{LoggingConfig, init_logging};
use baad_utils::formatter::LineFormatter;
use baad_utils::progress::{ProgressModel, ProgressView};
use rand::Rng;
use tracing::Level;

const TOTAL_TASKS: usize = 15;
const MAX_PARALLEL: usize = 5;

struct DownloadModel {
    active: Vec<bool>,
    completed: Vec<String>,
    formatter: LineFormatter
}

impl DownloadModel {
    fn start(&mut self, task_id: usize) { self.active[task_id] = true; }

    fn complete(&mut self, task_id: usize) {
        self.active[task_id] = false;

        let task_id_str = task_id.to_string();
        let mut line = String::new();
        self.formatter
            .write_line(&mut line, &Level::INFO, true, "complete", &[("task", &task_id_str)])
            .unwrap();
        self.completed.push(line);
    }
}

impl ProgressModel for DownloadModel {
    fn render(&mut self, _width: usize, output: &mut String) {
        for line in &self.completed {
            output.push_str(line);
        }

        for (index, &active) in self.active.iter().enumerate() {
            if active {
                let task_id = index.to_string();
                self.formatter
                    .write_line(output, &Level::INFO, false, "downloading", &[("task", &task_id)])
                    .unwrap();
            }
        }
    }

    fn final_message(&mut self, output: &mut String) {
        for line in &self.completed {
            output.push_str(line);
        }
    }
}

struct Semaphore {
    state: Mutex<usize>,
    available: Condvar,
    max: usize
}

impl Semaphore {
    fn new(max: usize) -> Self {
        Self {
            state: Mutex::new(0),
            available: Condvar::new(),
            max
        }
    }

    fn acquire(&self) {
        let mut count = self.state.lock().unwrap();
        while *count >= self.max {
            count = self.available.wait(count).unwrap();
        }
        *count += 1;
    }

    fn release(&self) {
        let mut count = self.state.lock().unwrap();
        *count -= 1;
        self.available.notify_one();
    }
}

fn simulate_download(task_id: usize, view: &ProgressView<DownloadModel>, semaphore: &Semaphore) {
    semaphore.acquire();
    let _ = view.update(|model| model.start(task_id));

    let mut rng = rand::rng();
    let steps = rng.random_range(5..20);
    for _ in 0..steps {
        thread::sleep(Duration::from_millis(rng.random_range(100..600)));
    }

    let _ = view.update(|model| model.complete(task_id));
    semaphore.release();
}

fn main() {
    let model = DownloadModel {
        active: vec![false; TOTAL_TASKS],
        completed: Vec::with_capacity(TOTAL_TASKS),
        formatter: LineFormatter::new().with_timestamps(true)
    };
    let view = Arc::new(ProgressView::new(model, Duration::ZERO));

    init_logging(LoggingConfig::default(), Arc::clone(&view)).unwrap();

    let semaphore = Arc::new(Semaphore::new(MAX_PARALLEL));

    let handles: Vec<_> = (0..TOTAL_TASKS)
        .map(|task_id| {
            let view = Arc::clone(&view);
            let semaphore = Arc::clone(&semaphore);
            thread::spawn(move || simulate_download(task_id, &view, &semaphore))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
