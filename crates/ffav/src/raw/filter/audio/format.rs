use std::ffi::CString;

use crate::{
    raw::filter::{Filter, HasInputPads, HasOutputPads},
    util::{channels::ChannelLayout, dict::Dictionary, sampling::SampleFormat},
};

pub struct AFormat {
    name: String,
    sample_rate: Option<u32>,
    sample_format: Option<SampleFormat>,
    channel_layout: Option<ChannelLayout>,
}

impl AFormat {
    pub fn new(
        name: impl Into<String>,
        sample_rate: Option<u32>,
        sample_fmt: Option<SampleFormat>,
        channel_layout: Option<ChannelLayout>,
    ) -> Self {
        AFormat {
            name: name.into(),
            sample_rate,
            sample_format: sample_fmt,
            channel_layout,
        }
    }
}

impl HasInputPads for AFormat {}
impl HasOutputPads for AFormat {}

impl Filter for AFormat {
    fn filter_name(&self) -> &String {
        &self.name
    }

    fn filter_type_name() -> CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("aformat").expect("Critical Error")
    }

    fn config_parameters_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        if let Some(sample_rate) = self.sample_rate {
            dict.add("sample_rates", format!("{}", sample_rate));
        }

        if let Some(sample_format) = self.sample_format {
            dict.add("sample_fmts", dbg!(format!("{}", sample_format)));
        }

        if let Some(channel_layout) = self.channel_layout {
            dict.add("channel_layouts", format!("{:x}", channel_layout));
        }

        dict
    }
}
