//! Types and Traits for enforcing API safety through the Type system

/// Types for managing channel layouts of Audio streams
pub mod channels;
pub mod marker;
/// Types for representing Video frame pixel layouts
pub mod pixels;
/// Types for representing Audio sample formats and layouts
pub mod sampling;
pub mod time;

use crate::{
    config::stream::DecodedStreamConfig,
    error::Result,
    raw::filter::{Filter, FilterInput},
};
use ffav_sys::AVMediaType;
use std::{ffi::CString, path::Path};

/// An identification for a data stream from a Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamId(pub(crate) i32);

/// Represents a marker type which indicates a specific media type
pub trait MediaType {
    /// The expected AVMediatType enum variant
    const MEDIA_TYPE: AVMediaType;
}

/// Represents a marker type which can go through the filter graph
pub trait Filterable: Sized {
    /// The type of Filter to be initialized for an input into the filter graph
    ///
    /// This is typically going to be a BufferSrc type of some description
    type InputType: Filter + FilterInput;

    /// Create a new input to the filter graph
    fn input_from_decoded_stream<N: Into<String>>(
        name: N,
        decoded: &DecodedStreamConfig<Self>,
    ) -> Self::InputType;
}

#[cfg(unix)]
pub(crate) fn path_to_cstr(p: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(p.as_os_str().as_bytes())?)
}

#[cfg(not(unix))]
pub(crate) fn path_to_cstr(p: &Path) -> Result<CString> {
    let s = p.to_str().ok_or_else(|| Error::InvalidPath(p.to_owned()))?;
    Ok(CString::new(s)?)
}
