//! Types for handling Streams of media data from a Container
use ffav_sys::AVStream;

use crate::{
    config::StreamConfig,
    error::{Error, Result},
    util::{marker::Unknown, traits::MediaMarker, StreamID},
};
use std::marker::PhantomData;

/// A Reference to a Stream of media data owned by a Container
pub struct StreamRef<'container, AV: MediaMarker> {
    stream: *mut AVStream,
    _lav: PhantomData<&'container AV>,
}

impl<'a, AV: MediaMarker> StreamRef<'a, AV> {
    /// Create a StreamRef from a provided AVStream
    ///
    /// The media-type of the Stream will be checked
    ///
    /// # Safety
    /// The `*mut AVStream` must be valid and the lifetime should be not exceed
    /// the lifetime of the Container which owns the stream.
    pub unsafe fn from_av_stream<'container>(
        other: *mut AVStream,
    ) -> Result<StreamRef<'container, AV>> {
        StreamRef::from_av_stream_unchecked(other).as_other_type()
    }
}

impl<'a> StreamRef<'a, Unknown> {
    /// Create a StreamRef from a provided AVStream
    ///
    /// # Safety
    /// The `*mut AVStream` must be valid and the lifetime should be not exceed
    /// the lifetime of the Container which owns the stream.
    pub unsafe fn from_av_stream_unchecked<'container>(
        other: *mut AVStream,
    ) -> StreamRef<'container, Unknown> {
        StreamRef {
            stream: other,
            _lav: PhantomData,
        }
    }

    /// Try to convert an Unknown stream to a Stream of a known Type
    pub fn as_other_type<T: MediaMarker>(self) -> Result<StreamRef<'a, T>> {
        unsafe {
            if (*(*self.stream).codecpar).codec_type != T::MEDIA_TYPE {
                return Err(Error::MediaTypeDoesntMatch {
                    expected: T::MEDIA_TYPE,
                    found: (*(*self.stream).codecpar).codec_type,
                });
            }

            Ok(std::mem::transmute(self))
        }
    }
}

/// A handle to a Stream of media within a Container
pub struct StreamHandle<AV: MediaMarker> {
    id: StreamID,
    index: usize,
    config: StreamConfig<AV>,
    _av: PhantomData<AV>,
}

impl<AV: MediaMarker> StreamHandle<AV> {
    /// Get the format specific ID of this Stream
    pub fn id(&self) -> StreamID {
        self.id
    }

    /// Get the index this stream is stored within its Container
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the configuration informatino for this stream
    pub fn config(&self) -> &StreamConfig<AV> {
        &self.config
    }
}

impl<'a, AV: MediaMarker> From<StreamRef<'a, AV>> for StreamHandle<AV> {
    fn from(_: StreamRef<'a, AV>) -> Self {
        unimplemented!()
    }
}
