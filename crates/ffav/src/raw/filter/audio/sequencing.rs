use std::ffi::CString;

use crate::{
    config::Dictionary,
    raw::filter::{Filter, HasInputPads, HasOutputPads},
    util::time::Timestamp,
};

pub struct ATrim {
    name: String,
    start: Timestamp,
    end: Option<Timestamp>,
    duration: Option<Timestamp>,
}

impl ATrim {
    pub fn new_with_duration(
        name: impl Into<String>,
        start: impl Into<Timestamp>,
        duration: impl Into<Timestamp>,
    ) -> Self {
        ATrim {
            name: name.into(),
            start: start.into(),
            end: None,
            duration: Some(duration.into()),
        }
    }

    pub fn new_with_end(
        name: impl Into<String>,
        start: impl Into<Timestamp>,
        end: impl Into<Timestamp>,
    ) -> Self {
        ATrim {
            name: name.into(),
            start: start.into(),
            end: Some(end.into()),
            duration: None,
        }
    }
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
    duration: Timestamp,
    overlap: bool,
}

impl ACrosssfade {
    pub fn no_overlap(name: impl Into<String>, duration: impl Into<Timestamp>) -> Self {
        ACrosssfade {
            name: name.into(),
            duration: duration.into(),
            overlap: false,
        }
    }
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

        dict.add("duration", format!("{}", self.duration));
        dict.add("overlap", format!("{}", if self.overlap { 1 } else { 0 }));

        dict
    }
}
