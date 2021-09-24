//! Error types and functions

use ffav_sys::AVMediaType;
use thiserror::Error;

/// Type alias for a `Result<T, crate::error::Error>`
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for all `ffav` functions
#[derive(Error, Debug)]
pub enum Error {
    /// A Rust string was passed which could not be converted into a valid string for FFI
    #[error("Error creating CString for FFI: {0}")]
    FFIString(#[from] std::ffi::NulError),

    /// Unknown or unimplmented error type.
    /// Always indicates a bug in the library
    #[error("An unknown error occurred: {0}\n Please report a bug as all errors should have useful types")]
    Unknown(&'static str),

    /// An error when converting from a `Something<Unknown>` to a `Something<AV>` of some other type.
    ///
    /// The flagged media-type of the `Something<_>` did not match the requested destination media type `AV`
    #[error(
        "Tried to create an object of tyoe {expected:?} but instead found media type {found:?}"
    )]
    MediaTypeDoesntMatch {
        /// The expected AVMediaType
        expected: AVMediaType,
        /// What AVMediaType was found
        found: AVMediaType,
    },
}
