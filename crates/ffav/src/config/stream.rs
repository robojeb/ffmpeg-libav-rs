use ffav_sys::AVStream;

use super::CodecParameters;
use crate::{
    error::Result,
    util::{marker::Unknown, time::TimeBase, traits::MediaMarker},
};
use std::marker::PhantomData;

/// The configuration of a media Stream
pub struct StreamConfig<AV: MediaMarker> {
    codec_parameters: CodecParameters<AV>,
    _av: PhantomData<AV>,
}

impl<AV: MediaMarker> StreamConfig<AV> {
    /// Create a StreamConfiguration from an existing AVStream object'
    ///
    /// If the MediaType of the stream configuration doesn't match an `Err` will
    /// be returned.
    ///
    /// # Safety
    /// The AVStream pointer must be valid
    ///
    /// # Panics
    /// If the space for CodecParameters cannot be allocated
    pub unsafe fn from_av_stream(other: *const AVStream) -> Result<StreamConfig<AV>> {
        Ok(StreamConfig {
            codec_parameters: CodecParameters::from_av_codec_parameters((*other).codecpar)?,
            // NOTE: We don't have to do further checking of the AV type match
            // because the `CodecParameters` will do it for us
            _av: PhantomData,
        })
    }

    /// Get the TimeBase used for this Stream
    pub fn time_base(&self) -> TimeBase {
        unimplemented!()
    }

    /// Get the CodecParameters used for this stream
    pub fn parameters(&self) -> &CodecParameters<AV> {
        &self.codec_parameters
    }
}

impl StreamConfig<Unknown> {
    /// Create a StreamConfiguration from an existing AVStream
    ///
    /// This does not check the media type of the stream
    ///
    /// # Safety
    /// The AVStream pointer must be valid
    ///
    /// # Panics
    /// If the space for the CodecParameters cannot be allocated
    pub unsafe fn from_av_stream_unchecked(other: *const AVStream) -> StreamConfig<Unknown> {
        StreamConfig {
            codec_parameters: CodecParameters::from_av_codec_parameters_unchecked(
                (*other).codecpar,
            ),
            _av: PhantomData,
        }
    }

    /// Try to convert a stream of unknown type into a known type
    pub fn as_other_type<T: MediaMarker>(self) -> Result<StreamConfig<T>> {
        Ok(StreamConfig {
            codec_parameters: self.codec_parameters.as_other_type()?,
            _av: PhantomData,
        })
    }
}
