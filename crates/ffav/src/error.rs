use std::ffi::CString;

use ffav_sys::{err::av_err, AVMediaType};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error creating CString for FFI: {0}")]
    FFIString(#[from] std::ffi::NulError),

    #[error("Error of unknown type: {0}")]
    Unknown(&'static str),

    /// Indicates that the requested filter could not be found
    #[error("Filter named {0:?} was not found.\nPerhaps the linked version of ffmpeg doesn't have this filter?")]
    FilterNotFound(CString),

    //
    // The following errors can be generated while working with a `FormatGraph`
    //
    #[error("The provided FilterHandle does not belong to this FilterGraph")]
    GraphDoesntOwnHandle,

    /// Indicates that the filter handle provided to the FilterGraph was invalid.
    ///
    /// This likely means a filter handle from another graph was incorrectly provided
    #[error("The filter named {name:?} doesn't exist in the FilterGraph")]
    FilterNotRegisteredWithGraph { name: String },

    /// The filter name could not be represented as a valid C-style string
    #[error("The name \"{0}\" could not be represented as a valid filter name.")]
    InvalidFilterName(String),

    #[error("The filter {name:?} of type {filter_type:?} didn't have the requested output pad #{pad_number}")]
    OutputPadDoesntExist {
        name: String,
        filter_type: CString,
        pad_number: u32,
    },

    #[error("The filter {name:?} of type {filter_type:?} didn't have the requested input pad #{pad_number}")]
    InputPadDoesntExist {
        name: String,
        filter_type: CString,
        pad_number: u32,
    },

    #[error("The requested pads do not have the same media type: src({src_type:?}) != dest({dest_type:?})")]
    PadTypeMismatch {
        src_type: AVMediaType,
        dest_type: AVMediaType,
    },

    //
    // The following errors come from the libav library
    //
    #[error("Unknown error returned from libav* while {ctx} ({ret_val})")]
    AVUnknown { ctx: &'static str, ret_val: i32 },

    #[error("Could not allocate required memory while {0}")]
    AllocationFailed(&'static str),

    #[error("Invalid arguments passed while {0}")]
    InvalidArguments(&'static str),

    #[error("Could not find requested {0}")]
    ResourceNotFound(&'static str),

    #[error("Could not get more output from codec/filter, more input required")]
    SubmitMoreInput,

    #[error("Could not accept more input, try getting output from this item")]
    CouldNotAcceptInput,

    #[error("Reached and of available input")]
    EoF,

    #[error("Could not find a relevant stream")]
    StreamNotFound,
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
        } else {
            Error::AVUnknown { ctx, ret_val: err }
        }
    }
}
