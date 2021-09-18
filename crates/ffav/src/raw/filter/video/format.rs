use std::ffi::CString;

use crate::{
    config::Dictionary,
    raw::filter::{Filter, HasInputPads, HasOutputPads},
    util::color::PixelFormat,
};

pub struct VFormat {
    name: String,
    pixel_format: Option<PixelFormat>,
}

impl VFormat {
    pub fn new(name: impl Into<String>, pixel_format: Option<PixelFormat>) -> Self {
        VFormat {
            name: name.into(),
            pixel_format,
        }
    }
}

impl HasInputPads for VFormat {}
impl HasOutputPads for VFormat {}

impl Filter for VFormat {
    fn filter_name(&self) -> &String {
        &self.name
    }

    fn filter_type_name() -> CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("format").expect("Critical Error")
    }

    fn config_parameters_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        if let Some(sample_rate) = self.pixel_format {
            dict.add("pix_fmts", dbg!(format!("{}", sample_rate)));
        }

        dict
    }
}
