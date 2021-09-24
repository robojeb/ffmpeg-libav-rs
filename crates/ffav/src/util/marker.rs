//! Marker types to indicate the state of a structure at compile time

use super::traits::MediaMarker;

/// Marker struct indicating that the associated item is related to Audio
#[derive(Debug, Clone, Copy)]
pub struct Audio;

impl MediaMarker for Audio {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_AUDIO;
}

/// Marker struct indicating that the associated item is related to Video
#[derive(Debug, Clone, Copy)]
pub struct Video;

impl MediaMarker for Video {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_VIDEO;
}

/// Marker struct indicating that the associated item is related to raw Data
#[derive(Debug, Clone, Copy)]
pub struct Data;

impl MediaMarker for Data {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_DATA;
}

/// Marker struct indicating that the associated item is related to Subtitles
#[derive(Debug, Clone, Copy)]
pub struct Subtitles;

impl MediaMarker for Subtitles {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_SUBTITLE;
}

/// Marker struct indicating that the associated item is related to file Attachments
#[derive(Debug, Clone, Copy)]
pub struct Attachment;

impl MediaMarker for Attachment {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_ATTACHMENT;
}

/// Marker struct indicating that the associated item is unknown
#[derive(Debug, Clone, Copy)]
pub struct Unknown;

impl MediaMarker for Unknown {
    const MEDIA_TYPE: ffav_sys::AVMediaType = ffav_sys::AVMediaType::AVMEDIA_TYPE_UNKNOWN;
}

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
