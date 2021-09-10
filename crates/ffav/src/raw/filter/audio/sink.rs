use crate::{
    error::{Error, Result},
    raw::filter::{Filter, FilterOutput, HasInputPads},
    tags::Audio,
    util::dict::Dictionary,
};
use ffav_sys::av_buffersink_get_frame;
use std::ffi::CString;

pub struct ABufferSink {
    name: String,
}

impl ABufferSink {
    pub fn new(name: impl Into<String>) -> Self {
        ABufferSink { name: name.into() }
    }
}

impl Filter for ABufferSink {
    fn filter_type_name() -> CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("abuffersink").expect("Critical Error")
    }

    fn filter_name(&self) -> &String {
        &self.name
    }

    fn config_parameters_dict(&self) -> Dictionary {
        Dictionary::new()
    }
}

impl HasInputPads for ABufferSink {}

impl FilterOutput for ABufferSink {
    type StreamType = Audio;

    unsafe fn get_frame_into(
        filter: *mut ffav_sys::AVFilterContext,
        frame: &mut crate::Frame<Self::StreamType>,
    ) -> Result<()> {
        let err = av_buffersink_get_frame(filter, frame.as_raw());

        if err < 0 {
            return Err(Error::from_av_err("getting frame from FilterGraph", err));
        }

        Ok(())
    }
}

pub struct ANullSink {
    name: String,
}

impl Filter for ANullSink {
    fn filter_type_name() -> CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("anullsink").expect("Critical Error")
    }

    fn filter_name(&self) -> &String {
        &self.name
    }

    fn config_parameters_dict(&self) -> Dictionary {
        Dictionary::new()
    }
}

impl HasInputPads for ANullSink {}
