use ffav_sys::{AVMediaType, AVStream};
use std::marker::PhantomData;

use crate::{
    config::StreamConfig,
    util::{marker::Unknown, MediaType},
};

use super::packet::Packet;

/// A stream contained in a format
///
/// This is a simple wrapper around the underlying libav struct and is mainly
/// used as an identification when asking for operations to be set up for streams.
///
/// NOTE: A stream is infinitely copyable because no modification of the underlying
/// data can be done through this handle.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stream<'ctx, AV> {
    stream: *mut AVStream,
    _av: PhantomData<&'ctx AV>,
}

impl<'ctx, AV> Stream<'ctx, AV> {
    /// Get the raw pointer to the Packet
    ///
    /// Intended as an escape hatch if something is impossible with the abstraction
    /// layer.
    ///
    /// # Safety
    /// The pointer should not be held longer than the life of the `Format` which owns this stream.
    /// While using the raw pointer it should be considered that the `Stream` and `Format
    /// are mutably borrowed.
    pub unsafe fn as_raw(&self) -> *mut AVStream {
        self.stream
    }

    /// Check if the provided packet belongs to this data stream
    pub fn is_packet_for_stream(&self, packet: &Packet) -> bool {
        unsafe { (*self.stream).index == (*packet.as_raw()).stream_index }
    }
}

impl<'ctx, AV: MediaType> Stream<'ctx, AV> {
    /// Generate the configuration object associated with this stream
    ///
    /// NOTE: This function builds the stream confing when it is requested and
    /// should not be called repeatedly or there may be a performance impact
    pub fn config(&self) -> StreamConfig<AV> {
        let sc = StreamConfig::from_av_stream(self.stream);
        match sc.try_as_type::<AV>() {
            Some(c) => c,
            // SAFETY: We know that this is going to be type AV because the
            // stream we are deriving it from is of that type
            None => unreachable!(),
        }
    }
}

impl<'ctx> Stream<'ctx, Unknown> {
    /// Check if this is an audio stream
    pub fn is_audio(&self) -> bool {
        unsafe { (*(*self.stream).codecpar).codec_type == AVMediaType::AVMEDIA_TYPE_AUDIO }
    }

    /// Check if this is a video stream
    pub fn is_video(&self) -> bool {
        unsafe { (*(*self.stream).codecpar).codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO }
    }

    /// Try to cast the `Unknown` stream to type `T`
    pub fn try_as_type<T: MediaType>(self) -> Option<Stream<'ctx, T>> {
        unsafe {
            if (*(*self.stream).codecpar).codec_type == T::MEDIA_TYPE {
                Some(std::mem::transmute(self))
            } else {
                None
            }
        }
    }
}

// TODO: Useful information getters

// NOTE: No drop impl is required for the `Stream` object because its lifetime
// is constrained by the Format which created it.
