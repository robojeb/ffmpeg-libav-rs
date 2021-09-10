use std::ffi::CString;

use crate::{
    raw::filter::{Filter, HasInputPads, HasOutputPads},
    util::{dict::Dictionary, time::Timestamp},
};

pub struct ATrim {
    name: String,
    start: Timestamp,
    end: Option<Timestamp>,
    duration: Option<Timestamp>,
}

impl HasInputPads for ATrim {}
impl HasOutputPads for ATrim {}

impl Filter for ATrim {
    fn filter_name(&self) -> &String {
        &self.name
    }

    fn filter_type_name() -> std::ffi::CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("atrim").expect("Critical Error")
    }

    fn config_parameters_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        dict.add("start", format!("{}", self.start));

        if let Some(end) = self.end {
            dict.add("end", format!("{}", end));
        } else if let Some(dur) = self.duration {
            dict.add("duration", format!("{}", dur));
        } else {
            panic!("Duration or End not set");
        }

        dict
    }
}

pub struct ACrosssfade {
    name: String,
    nb_samples: usize,
    overlap: bool,
}

impl HasInputPads for ACrosssfade {}
impl HasOutputPads for ACrosssfade {}

impl Filter for ACrosssfade {
    fn filter_name(&self) -> &String {
        &self.name
    }

    fn filter_type_name() -> std::ffi::CString {
        // NOTE: This unwrap should never fail because I know this string to be
        // a valid C-style string. If this fails something horrible has gone wrong
        // and a bug should probably be filed.
        CString::new("acrossfade").expect("Critical Error")
    }

    fn config_parameters_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        dict.add("nb_samples", format!("{}", self.nb_samples));
        dict.add("overlap", format!("{}", if self.overlap { 1 } else { 0 }));

        dict
    }
}
