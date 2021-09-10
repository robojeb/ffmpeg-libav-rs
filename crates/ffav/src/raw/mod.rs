//! This module provides "raw" bindings to the ffmpeg libav* libraries.
//!
//! The provided wrappers are intended to be safe, but not fool-proof.
//! Very little protection is provided for accidentally passing configuration
//! information or decode data from incorrect Formats. It shouldn't be possible
//! to SegFault the system, but you can quite easily get errors passing the
//! wrong data to the wrong function.

pub mod codec;
pub mod filter;
pub mod format;
pub mod frame;
pub mod packet;
pub mod stream;
