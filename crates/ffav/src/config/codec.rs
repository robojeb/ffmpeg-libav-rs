use ffav_sys::{
    avcodec_parameters_alloc, avcodec_parameters_copy, avcodec_parameters_free, AVCodecParameters,
    AVPixelFormat, AVSampleFormat,
};

use crate::{
    error::{Error, Result},
    util::{
        color::{ColorPrimary, PixelFormat},
        marker::{Audio, Unknown, Video},
        sampling::SampleFormat,
        time::{FrameRate, SampleRate},
        traits::MediaMarker,
    },
};
use std::marker::PhantomData;

/// Parameters for configuring a codec object to Encode or Decode a stream
pub struct CodecParameters<AV: MediaMarker> {
    params: *mut AVCodecParameters,
    _av: PhantomData<AV>,
}

impl<AV: MediaMarker> CodecParameters<AV> {
    /// Create a new set of CodecParameters from an existing raw AVCodecParameters pointer
    ///
    /// The type of the output must match or an `Err` will be returned at runtime
    ///
    /// # Safety
    /// The `other` AVCodecParameters pointer must be valid and have been allocated with
    /// `ffavsys::avcodec_paramters_alloc()` or any other `ffav_sys` function.
    pub unsafe fn from_av_codec_parameters(
        other: *const AVCodecParameters,
    ) -> Result<CodecParameters<AV>> {
        CodecParameters::from_av_codec_parameters_unchecked(other).as_other_type()
    }
}

impl CodecParameters<Unknown> {
    /// Create a new set of CodecParameters from an existing raw AVCodecParameters pointer
    ///
    /// The media type of the passed AVCodecParameters is not checked.
    /// Conversion to other types can be performed
    ///
    /// # Safety
    /// The `other` AVCodecParameters pointer must be valid and have been allocated with
    /// `ffavsys::avcodec_paramters_alloc()` or any other `ffav_sys` function.
    ///
    /// # Panics
    /// Panics if space could not be allocated to hold the information required
    /// for CodecParameters
    pub unsafe fn from_av_codec_parameters_unchecked(
        other: *const AVCodecParameters,
    ) -> CodecParameters<Unknown> {
        let params = avcodec_parameters_alloc();

        if params.is_null() {
            panic!("Could not allocate space for AVCodecParameter")
        }

        let err = avcodec_parameters_copy(params, other);
        if err < 0 {
            panic!("Could not allocate space for the extra data while copying");
        }

        CodecParameters {
            params,
            _av: PhantomData,
        }
    }

    /// Try to convert codec for an Unknown media stream type into one of type `T`
    ///
    /// If the underlying stream is not of type `T` this will return an `Error::MediaTypeDoesntMatch` error
    pub fn as_other_type<T: MediaMarker>(self) -> Result<CodecParameters<T>> {
        unsafe {
            if (*self.params).codec_type != T::MEDIA_TYPE {
                Err(Error::MediaTypeDoesntMatch {
                    expected: T::MEDIA_TYPE,
                    found: (*self.params).codec_type,
                })
            } else {
                Ok(std::mem::transmute(self))
            }
        }
    }
}

impl CodecParameters<Audio> {
    /// How many channels does this stream contain
    #[inline]
    pub fn num_channels(&self) -> u32 {
        unsafe { (*self.params).channels as u32 }
    }

    // /// What is the channel layout of this stream
    // #[inline]
    // pub fn channel_layout(&self) -> ChannelLayout {
    //     unsafe { ChannelLayout::from_bits_truncate((*self.params).channel_layout) }
    // }

    /// What is the sample rate of this stream
    #[inline]
    pub fn sample_rate(&self) -> SampleRate {
        unsafe { SampleRate::new((*self.params).sample_rate as u32) }
    }

    /// What format are samples stored in this stream
    #[inline]
    pub fn sample_format(&self) -> SampleFormat {
        // Safety: The `format` field from an `AVCodecParameters` struct can be one of two
        // types cast ot an i32 depending on the underlying media type.
        // Per the documentation:
        //
        //  video: the pixel format, the value corresponds to enum AVPixelFormat.
        //  audio: the sample format, the value corresponds to enum AVSampleFormat.
        //
        // It is thus safe to transmute to the unerlying `AVSampleFormat` because
        // we know that this stream is `Audio` type. Then we can convert to the
        // High level `SampleFormat` type which may not be bit identical to the
        // C enum.
        let av_fmt: AVSampleFormat = unsafe { std::mem::transmute((*self.params).format) };

        SampleFormat::from(av_fmt)
    }
}

impl CodecParameters<Video> {
    /// The width of a single frame of video
    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { (*self.params).width as u32 }
    }

    /// The height of a single frame of vidoe
    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { (*self.params).height as u32 }
    }

    /// The pixel format of a decoded frame in this stream
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        // Safety: The `format` field from an `AVCodecParameters` struct can be one of two
        // types cast ot an i32 depending on the underlying media type.
        // Per the documentation:
        //
        //  video: the pixel format, the value corresponds to enum AVPixelFormat.
        //  audio: the sample format, the value corresponds to enum AVSampleFormat.
        //
        // It is thus safe to transmute to the unerlying `AVPixelFormat` because
        // we know that this stream is `Audio` type. Then we can convert to the
        // High level `SampleFormat` type which may not be bit identical to the
        // C enum.
        let av_fmt: AVPixelFormat = unsafe { std::mem::transmute((*self.params).format) };

        PixelFormat::from(av_fmt)
    }

    /// Get the color primary used for this video stream
    #[inline]
    pub fn color_primary(&self) -> ColorPrimary {
        unsafe { ColorPrimary::from((*self.params).color_primaries) }
    }

    /// Get the average frame-rate of this stream
    #[inline]
    pub fn avg_frame_rate(&self) -> FrameRate {
        unimplemented!()
    }
}

impl<AV: MediaMarker> std::ops::Drop for CodecParameters<AV> {
    fn drop(&mut self) {
        // Safety: `avcodec_parameters_free()` doesn't care if the pointer is
        // already NULL, we also know that every `AVCodecParameter` contained in
        // this structure was copied using `avcodec_parameters_copy` so we are
        // responsible for freeing it
        unsafe {
            avcodec_parameters_free(&mut self.params);
            self.params = std::ptr::null_mut();
        }
    }
}
