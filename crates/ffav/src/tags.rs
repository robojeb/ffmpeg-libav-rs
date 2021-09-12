//! Contains marker structs used to enforce type safety for components

use crate::{
    raw::filter::audio::ABufferSource,
    util::{Filterable, MediaType},
};
use ffav_sys::AVMediaType;

/// Marker struct indicating that the associated item is related to Audio
#[derive(Debug, Clone, Copy)]
pub struct Audio;

/// Marker struct indicating that the associated item is related to Video
#[derive(Debug, Clone, Copy)]
pub struct Video;

/// Marker struct indicating that the associated item is related to raw Data
#[derive(Debug, Clone, Copy)]
pub struct Data;

/// Marker struct indicating that the associated item is related to Subtitles
#[derive(Debug, Clone, Copy)]
pub struct Subtitles;

/// Marker struct indicating that the associated item is related to file Attachments
#[derive(Debug, Clone, Copy)]
pub struct Attachment;

/// Marker struct indicating that the associated item is unknown
#[derive(Debug, Clone, Copy)]
pub struct Unknown;

/// Marker struct for Input related items
#[derive(Debug, Clone, Copy)]
pub struct Input;

/// Marker struct for Ouput related items
#[derive(Debug, Clone, Copy)]
pub struct Output;

/// Marker struct for Decoding configured codecs
#[derive(Debug, Clone, Copy)]
pub struct Decode;

/// Marker struct for Encoding configured codecs
#[derive(Debug, Clone, Copy)]
pub struct Encode;

/// Marker struct for a fully configured FilterGraph
#[derive(Debug, Clone, Copy)]
pub struct Configured;

/// Marker struct for a FilterGraph that is still being configured
#[derive(Debug, Clone, Copy)]
pub struct Unconfigured;

impl MediaType for Audio {
    const MEDIA_TYPE: AVMediaType = AVMediaType::AVMEDIA_TYPE_AUDIO;
}

impl MediaType for Video {
    const MEDIA_TYPE: AVMediaType = AVMediaType::AVMEDIA_TYPE_VIDEO;
}

impl MediaType for Subtitles {
    const MEDIA_TYPE: AVMediaType = AVMediaType::AVMEDIA_TYPE_SUBTITLE;
}

impl MediaType for Data {
    const MEDIA_TYPE: AVMediaType = AVMediaType::AVMEDIA_TYPE_DATA;
}

impl MediaType for Attachment {
    const MEDIA_TYPE: AVMediaType = AVMediaType::AVMEDIA_TYPE_ATTACHMENT;
}

impl Filterable for Audio {
    type InputType = ABufferSource;

    fn from_decoded_stream<N: Into<String>>(
        name: N,
        decoded: &crate::config::stream::DecodedStreamConfig<Self>,
    ) -> Self::InputType {
        ABufferSource::from_decoded_stream(name, decoded)
    }
}
