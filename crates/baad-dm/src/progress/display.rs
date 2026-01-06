use super::StyleOptions;

use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget};

pub struct ProgressDisplay {
    multi: Arc<MultiProgress>,
    main: Arc<ProgressBar>,
    style: StyleOptions,
    show_main: bool,
}

impl ProgressDisplay {
    pub fn new(style: StyleOptions, total: usize, single_file_mode: bool) -> Self {
        let multi = Arc::new(match style.is_enabled() {
            true => MultiProgress::new(),
            false => MultiProgress::with_draw_target(ProgressDrawTarget::hidden()),
        });

        let show_main = !single_file_mode || total > 1;

        let main = Arc::new(match show_main {
            true => {
                let pb = multi.add(style.main.to_progress_bar(total as u64));
                pb.tick();
                pb
            }
            false => ProgressBar::hidden(),
        });

        Self { multi, main, style, show_main }
    }

    #[inline]
    pub fn create_child(&self, size: u64, position: u64) -> ProgressBar {
        self.multi.add(self.style.child.to_progress_bar(size).with_position(position))
    }

    #[inline]
    pub fn increment_main(&self) {
        self.main.inc(1);
    }

    #[inline]
    pub fn finish_child(&self, pb: ProgressBar) {
        match self.style.child.clear {
            true => pb.finish_and_clear(),
            false => pb.finish(),
        }
    }

    pub fn finish(self) {
        if self.show_main {
            match self.style.main.clear {
                true => self.main.finish_and_clear(),
                false => self.main.finish(),
            }
        }
    }
}
