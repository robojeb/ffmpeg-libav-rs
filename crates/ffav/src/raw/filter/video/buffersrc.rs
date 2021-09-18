use crate::{
    config::{stream::DecodedStreamConfig, Dictionary},
    error::Error,
    raw::filter::{Filter, FilterInput, HasOutputPads},
    util::marker::Video,
};
use ffav_sys::{av_buffersrc_add_frame_flags, AVFilterContext, AV_BUFFERSRC_FLAG_KEEP_REF};
use std::ffi::CString;

pub struct VBufferSource {
    name: String,
    incoming_stream: DecodedStreamConfig<Video>,
}

impl VBufferSource {
    pub fn from_decoded_stream(
        name: impl Into<String>,
        dec_stream: &DecodedStreamConfig<Video>,
    ) -> Self {
        VBufferSource {
            name: name.into(),
            incoming_stream: dec_stream.clone(),
        }
    }
}

impl Filter for VBufferSource {
    fn filter_type_name() -> CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("buffer").expect("Critical Error")
    }

    fn filter_name(&self) -> &String {
        &self.name
    }

    fn config_parameters_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        dict.add("time_base", format!("{}", self.incoming_stream.time_base()));
        dict.add("width", dbg!(format!("{}", self.incoming_stream.width())));
        dict.add("height", dbg!(format!("{}", self.incoming_stream.height())));
        dict.add(
            "pix_fmt",
            format!("{}", self.incoming_stream.pixel_format()),
        );

        dict
    }
}

impl HasOutputPads for VBufferSource {}

impl FilterInput for VBufferSource {
    type StreamType = Video;

    unsafe fn submit_frame(
        filter: *mut AVFilterContext,
        frame: &mut crate::Frame<Self::StreamType>,
    ) -> crate::error::Result<()> {
        let err =
            av_buffersrc_add_frame_flags(filter, frame.as_raw(), AV_BUFFERSRC_FLAG_KEEP_REF as i32);

        if err < 0 {
            return Err(Error::from_av_err("submitting frame to FrameGraph", err));
        }

        Ok(())
    }
}
