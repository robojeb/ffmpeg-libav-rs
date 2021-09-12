pub mod channels;
pub mod dict;
pub mod sampling;
pub mod time;

use ffav_sys::AVMediaType;

use crate::{
    config::stream::DecodedStreamConfig,
    raw::filter::{Filter, FilterInput},
};

/// An identification for a data stream from a Format

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamId(pub(crate) i32);

/// Represents a sampling rate in Hertz
pub struct Hz(u32);

impl Hz {
    pub fn from_hertz<H: Into<u32>>(hertz: H) -> Hz {
        Hz(hertz.into())
    }

    pub fn as_hertz_u32(&self) -> u32 {
        self.0
    }
}

/// Represents a marker type which indicates a specific media type
pub trait MediaType {
    const MEDIA_TYPE: AVMediaType;
}

/// Represents a marker type which can go through the filter graph
pub trait Filterable: Sized {
    type InputType: Filter + FilterInput;

    fn from_decoded_stream<N: Into<String>>(
        name: N,
        decoded: &DecodedStreamConfig<Self>,
    ) -> Self::InputType;
}
