#![allow(missing_docs)]

//! Common error type
//!
//! All functions which can fail in this library will return a `Result<T, Error>`
//! and in the error case will attempt to provide a useful specific error.
//! In some cases it isn't possible to determine the exact reason why a libav*
//! function returned an error, in that case the generic `AVUnknown` will be
//! returned with some context about what operation was being attempted at the
//! time of the failure.

use std::{ffi::CString, path::PathBuf};

use ffav_sys::{err::av_err, AVMediaType};
use thiserror::Error;

/// Common alias for a Result with `crate::error::Error`
pub type Result<T> = std::result::Result<T, Error>;

/// The common Error type for ffav
#[derive(Error, Debug)]
pub enum Error {
    /// A Rust string was passed which could not be converted into a valid string for FFI
    #[error("Error creating CString for FFI: {0}")]
    FFIString(#[from] std::ffi::NulError),

    /// An unknown erorr occured
    ///
    /// NOTE: Any instance of this error indicates a portion of the library under development
    /// check the github issues, or file a new issue for this error
    #[error("Error of unknown type: {0}")]
    Unknown(&'static str),

    /// A Path could not be used in an FFI context
    ///
    /// Typically this will occur on Windows systems
    #[error("The supplied path could not be validated for FFI: {0:?}")]
    InvalidPath(PathBuf),

    /// Indicates that the requested filter could not be found
    #[error("Filter named {0:?} was not found.\nPerhaps the linked version of ffmpeg doesn't have this filter?")]
    FilterNotFound(CString),

    //
    // The following errors can be generated while working with a `FilterGraph`
    //
    /// The FormatHandle being passed as an argument does not belong to the Filter Graph
    #[error("The provided FilterHandle does not belong to this FilterGraph")]
    GraphDoesntOwnHandle,

    /// Indicates that the filter handle provided to the FilterGraph was invalid.
    ///
    /// This likely means a filter handle from another graph was incorrectly provided
    #[error("The filter named {name:?} doesn't exist in the FilterGraph")]
    FilterNotRegisteredWithGraph {
        /// Name of the filter in the provided FilterHandle
        name: String,
    },

    /// The filter name could not be represented as a valid C-style string
    #[error("The name \"{0}\" could not be represented as a valid filter name.")]
    InvalidFilterName(String),

    /// The requested Filter output pad doesn't exist
    #[error("The filter {name:?} of type {filter_type:?} didn't have the requested output pad #{pad_number}")]
    OutputPadDoesntExist {
        name: String,
        filter_type: CString,
        pad_number: u32,
    },

    /// The requested Filter Input pad doesn't exist
    #[error("The filter {name:?} of type {filter_type:?} didn't have the requested input pad #{pad_number}")]
    InputPadDoesntExist {
        name: String,
        filter_type: CString,
        pad_number: u32,
    },

    /// The two pads requested to be linked don't have the same media type
    #[error("The requested pads do not have the same media type: src({src_type:?}) != dest({dest_type:?})")]
    PadTypeMismatch {
        src_type: AVMediaType,
        dest_type: AVMediaType,
    },

    /// The provided packet was not for the stream that configured this Codec
    #[error("The supplied packet was not for the stream which configured this Codec")]
    PacketFromInvalidStream,

    //
    // The following errors come from the libav library
    //
    /// An unknown error was returned by libav
    #[error("Unknown error returned from libav* while {ctx} ({ret_val})")]
    AVUnknown { ctx: &'static str, ret_val: i32 },

    /// Libav failed to allocate some required memory
    #[error("Could not allocate required memory while {0}")]
    AllocationFailed(&'static str),

    /// Invalid arguments were passed to a libav function
    #[error("Invalid arguments passed while {0}")]
    InvalidArguments(&'static str),

    /// Could not find the requested item
    #[error("Could not find requested {0}")]
    ResourceNotFound(&'static str),

    /// The requested operation cannot complete without more input being sumitted
    #[error("Could not get more output from codec/filter, more input required")]
    SubmitMoreInput,

    /// The requested operation could not complete because more input could not be accepted
    #[error("Could not accept more input, try getting output from this item")]
    CouldNotAcceptInput,

    /// Reached the end of available data in a Format
    #[error("Reached and of available input")]
    EoF,

    /// The requested stream was not found
    #[error("Could not find a relevant stream")]
    StreamNotFound,

    /// Incorrect or invalid data was passed to a Codec
    #[error("Format stream contained invalid data")]
    InvalidData,
}

impl Error {
    pub(crate) fn from_av_err(ctx: &'static str, err: i32) -> Error {
        if err == av_err(ffav_sys::err::ENOMEM) {
            Error::AllocationFailed(ctx)
        } else if err == av_err(ffav_sys::err::EINVAL) {
            Error::InvalidArguments(ctx)
        } else if err == ffav_sys::err::AVERROR_EOF {
            Error::EoF
        } else if err == ffav_sys::err::AVERROR_STREAM_NOT_FOUND {
            Error::StreamNotFound
        } else if err == ffav_sys::err::AVERROR_INVALIDDATA {
            Error::InvalidData
        } else {
            Error::AVUnknown { ctx, ret_val: err }
        }
    }
}
