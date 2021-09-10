//! This crate provides safe wrappers around ffmpeg's libav* set of libraries.
//!

pub mod config;
pub mod decode;
pub mod error;
pub mod raw;
pub mod tags;
pub mod util;

pub use raw::{frame::Frame, packet::Packet};
