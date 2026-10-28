mod cache;
mod fetch;
mod pipeline;
mod reader;
mod source;

use fetch::fetch_segments;
pub use pipeline::download;
use reader::OrderedSegmentReader;
pub use source::{ServerSegment, network_total, progress_limits, validate_segments};
