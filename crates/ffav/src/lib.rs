#![warn(missing_docs)]
//! This crate provides safe wrappers around ffmpeg's libav* set of libraries.
//!
//! # Goals
//! There are three goals of this library:
//!    1. Be Memory Safe
//!    1. Prevent programmer errors
//!    1. Be efficient for critical operations
//!
//! As the first goal it should be impossible for any use of this API to lead
//! to a segementation fault, or other memory issue. As with most Rust programs
//! memory leaks are not considered safety issues, but they are avoided when
//! possible.
//!
//! For the second goal, this library attempts to encode much of the useful
//! or critical information into the Type system. For example a `StreamConfig<AV>`
//! is parameterized by its stream media type `<AV>`, a stream of type `Audio`
//! will only present information that is relevant to an Audio stream like `sampling_rate()`
//! but not irrelevant information like `pixel_format()`.
//! In cases where it isn't possible or would be incredibly cumbersome to encode
//! the appropriate invariants in the type-system this Library will attempt to
//! check early and return useful messages through the `Error` type.
//!
//! As the final goal, this library attempts to have little overhead above the
//! libav* C-API. To this end, checks are optimized in the required fast path
//! when encoding/decoding media. In non-performance critical places like
//! configuration this library may take extra time to check invariants.
//!
//! # Usage
//! This library provides two classes of interface, "raw" and "high-level".
//!
//! The "raw" interface is provided in the `raw` module. This interface is
//! mostly a direct port of the libav* APIs. The goals of the `raw` module are
//! to provide memory safety, and some minimal type level API safety. Through
//! programmer errors, it is possible to get a runtime failure using this API.
//!
//! The "high-level" API is split across three modules `decode`, `encode`, and
//! `transcode`. The goal here is to prevent all runtime erors that are not
//! related to, resource exhaustion, input corruption, or hardware failure.
//! This means what the APIs can do is limited to a small subset of what libav
//! can do.
//!
//! ## High level API
//! TODO
//!
//! ## Raw API
//! TODO

#[cfg(target_pointer_width = "16")]
std::compile_error! {"Targets with 16-bit pointer width are not supported at this time"}

pub mod config;
pub mod error;
pub mod raw;
pub mod util;
