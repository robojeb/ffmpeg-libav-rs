mod color_primary;
#[cfg(feature = "image-decode")]
mod image_ext;
mod pixel_format;

pub use color_primary::*;
#[cfg(feature = "image-decode")]
pub use image_ext::*;
pub use pixel_format::*;

/// Indicates the endianness of the underlying pixel format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    /// Stored in little-endian format
    Big,
    /// Stored in big-endian format
    Little,
}
