mod ansi;
#[cfg(feature = "observer")]
mod model;
pub mod terminal;
mod view;

#[cfg(feature = "observer")]
pub use model::{DownloadProgressHandler, DownloadProgressModel, ProgressObserver};
pub use view::{ProgressMakeWriter, ProgressModel, ProgressView, ProgressWriter};
