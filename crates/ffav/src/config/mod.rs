//! This module contains data structures which represent the configuration of
//! various components of an A/V Container or Stream.
//!

pub mod format;
pub mod stream;

pub use format::FormatConfig;
pub use stream::StreamConfig;

pub struct CodecConfig {}

pub struct FilterConfig {}
