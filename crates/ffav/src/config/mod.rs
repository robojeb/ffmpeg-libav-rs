//! This module contains data structures which store or modify the configuration
//! of various `ffav` structures.
//!

mod dict;
pub mod format;
pub mod stream;

pub use dict::*;
pub use format::FormatConfig;
pub use stream::StreamConfig;
