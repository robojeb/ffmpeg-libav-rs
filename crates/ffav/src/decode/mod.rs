//! Helper functions and structures to decode A/V streams

#[cfg(feature = "image-decode")]
mod image_decode;
mod simple;

#[cfg(feature = "image-decode")]
pub use image_decode::ImageDecoder;
pub use simple::SimpleDecoder;
