use std::io::{self, Write as IoWrite};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use baad_shared::ProgressError;
use simdutf8::basic::from_utf8;
use tracing_subscriber::fmt::MakeWriter;

use crate::progress::{ansi, terminal};

pub trait ProgressModel: Send + 'static {
    fn render(&mut self, width: usize, buffer: &mut String);

    fn final_message(&mut self, _buffer: &mut String) {}
}

pub struct ProgressView<M: ProgressModel> {
    inner: Mutex<Option<InnerView<M>>>
}

struct InnerView<M: ProgressModel> {
    model: M,
    write_buffer: Vec<u8>,
    render_buffer: String,
    lines_drawn: usize,
    drawn: bool,
    last_paint: Option<Instant>,
    update_interval: Duration,
    is_terminal: bool,
    cached_width: usize,
    last_width_check: Option<Instant>
}

const WIDTH_CACHE_DURATION: Duration = Duration::from_secs(1);
const DEFAULT_WIDTH: usize = 80;

impl<M: ProgressModel> ProgressView<M> {
    pub fn new(model: M, update_interval: Duration) -> Self {
        let is_terminal = terminal::is_terminal();

        Self {
            inner: Mutex::new(Some(InnerView {
                model,
                write_buffer: Vec::with_capacity(4096),
                render_buffer: String::with_capacity(2048),
                lines_drawn: 0,
                drawn: false,
                last_paint: None,
                update_interval,
                is_terminal,
                cached_width: terminal::width().unwrap_or(DEFAULT_WIDTH),
                last_width_check: Some(Instant::now())
            }))
        }
    }

    fn with_inner<F, R>(&self, function: F) -> Result<R, ProgressError>
    where
        F: FnOnce(&mut InnerView<M>) -> R
    {
        let mut guard = self.inner.lock().map_err(|_| ProgressError::MutexPoisoned)?;
        let inner = guard.as_mut().ok_or(ProgressError::AlreadyFinished)?;
        Ok(function(inner))
    }

    pub fn update<F, R>(&self, update_function: F) -> Result<R, ProgressError>
    where
        F: FnOnce(&mut M) -> R
    {
        self.with_inner(|inner| {
            let result = update_function(&mut inner.model);
            inner.paint();
            result
        })
    }

    pub fn message(&self, message: &str) -> Result<(), ProgressError> {
        self.with_inner(|inner| inner.write_message_and_repaint(message))
    }

    pub fn clear(&self) -> Result<(), ProgressError> { self.with_inner(|inner| inner.erase()) }

    pub fn finish(self) -> Result<M, ProgressError> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| ProgressError::MutexPoisoned)?
            .take()
            .ok_or(ProgressError::AlreadyFinished)?;

        inner.erase();
        inner.write_final_message();

        Ok(inner.model)
    }
}

impl<M: ProgressModel> Drop for ProgressView<M> {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.inner.try_lock()
            && let Some(inner) = guard.as_mut()
        {
            inner.erase();
            inner.write_final_message();
        }
    }
}

impl<M: ProgressModel> io::Write for &ProgressView<M> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut guard =
            self.inner.lock().map_err(|_| io::Error::other(ProgressError::MutexPoisoned))?;
        let inner =
            guard.as_mut().ok_or_else(|| io::Error::other(ProgressError::AlreadyFinished))?;
        let message = from_utf8(buf).map_err(|_| io::Error::other(ProgressError::InvalidUtf8))?;

        inner.write_message_and_repaint(message);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl<M: ProgressModel> InnerView<M> {
    fn terminal_width(&mut self) -> usize {
        let should_refresh =
            self.last_width_check.is_none_or(|last| last.elapsed() >= WIDTH_CACHE_DURATION);

        if should_refresh {
            self.cached_width = terminal::width().unwrap_or(DEFAULT_WIDTH);
            self.last_width_check = Some(Instant::now());
        }

        self.cached_width
    }

    fn should_paint(&self) -> bool {
        if !self.is_terminal {
            return false;
        }

        self.last_paint.is_none_or(|last| last.elapsed() >= self.update_interval)
    }

    fn render_to_buffer(&mut self) {
        let width = self.terminal_width();
        self.render_buffer.clear();
        self.model.render(width, &mut self.render_buffer);
    }

    fn write_rendered_lines(&mut self) {
        self.write_buffer.extend_from_slice(ansi::DISABLE_LINE_WRAP);

        let mut line_count = 0;
        for (index, line) in self.render_buffer.lines().enumerate() {
            if index > 0 {
                self.write_buffer.push(b'\n');
            }
            self.write_buffer.extend_from_slice(line.as_bytes());
            self.write_buffer.extend_from_slice(ansi::CLEAR_TO_END_OF_LINE);
            line_count = index;
        }

        self.write_buffer.extend_from_slice(ansi::CLEAR_TO_END_OF_SCREEN);
        self.write_buffer.extend_from_slice(ansi::ENABLE_LINE_WRAP);

        self.flush_buffer();
        self.lines_drawn = line_count;
        self.drawn = !self.render_buffer.is_empty();
        self.last_paint = Some(Instant::now());
    }

    fn paint(&mut self) {
        if !self.should_paint() {
            return;
        }

        self.render_to_buffer();

        if self.render_buffer.is_empty() && !self.drawn {
            return;
        }

        self.write_buffer.clear();

        if self.drawn {
            ansi::cursor_up_and_home(self.lines_drawn, &mut self.write_buffer);
        }

        self.write_rendered_lines();
    }

    fn erase(&mut self) {
        if !self.drawn {
            return;
        }

        self.write_buffer.clear();
        ansi::cursor_up_and_home(self.lines_drawn, &mut self.write_buffer);
        self.write_buffer.extend_from_slice(ansi::CLEAR_TO_END_OF_SCREEN);
        self.write_buffer.extend_from_slice(ansi::ENABLE_LINE_WRAP);
        self.flush_buffer();
        self.lines_drawn = 0;
        self.drawn = false;
    }

    fn write_message_and_repaint(&mut self, message: &str) {
        if !self.is_terminal || !self.drawn {
            self.write_raw(message.as_bytes());
            return;
        }

        self.render_to_buffer();

        self.write_buffer.clear();
        ansi::cursor_up_and_home(self.lines_drawn, &mut self.write_buffer);

        for byte in message.bytes() {
            if byte == b'\n' {
                self.write_buffer.extend_from_slice(ansi::CLEAR_TO_END_OF_LINE);
            }
            self.write_buffer.push(byte);
        }
        if !message.ends_with('\n') {
            self.write_buffer.extend_from_slice(ansi::CLEAR_TO_END_OF_LINE);
        }

        self.write_rendered_lines();
    }

    fn write_raw(&mut self, bytes: &[u8]) {
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(bytes);
        let _ = stderr.flush();
    }

    fn flush_buffer(&mut self) {
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(&self.write_buffer);
        let _ = stderr.flush();
    }

    fn write_final_message(&mut self) {
        self.render_buffer.clear();
        self.model.final_message(&mut self.render_buffer);

        if self.render_buffer.is_empty() {
            return;
        }

        self.write_buffer.clear();
        self.write_buffer.extend_from_slice(self.render_buffer.as_bytes());
        if !self.render_buffer.ends_with('\n') {
            self.write_buffer.push(b'\n');
        }
        self.flush_buffer();
    }
}

pub struct ProgressMakeWriter<M: ProgressModel> {
    view: Arc<ProgressView<M>>
}

impl<M: ProgressModel> ProgressMakeWriter<M> {
    pub fn new(view: Arc<ProgressView<M>>) -> Self { Self { view } }
}

impl<M: ProgressModel> Clone for ProgressMakeWriter<M> {
    fn clone(&self) -> Self {
        Self {
            view: Arc::clone(&self.view)
        }
    }
}

pub struct ProgressWriter<M: ProgressModel> {
    view: Arc<ProgressView<M>>
}

impl<M: ProgressModel> io::Write for ProgressWriter<M> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { (&*self.view).write(buf) }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl<M: ProgressModel + Send + Sync + 'static> MakeWriter<'_> for ProgressMakeWriter<M> {
    type Writer = ProgressWriter<M>;

    fn make_writer(&self) -> Self::Writer {
        ProgressWriter {
            view: Arc::clone(&self.view)
        }
    }
}
