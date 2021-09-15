//! Configuration information for streams contained in Formats

use std::{marker::PhantomData, ops::Deref};

use ffav_sys::{
    AVCodecContext, AVCodecID, AVCodecParameters, AVMediaType, AVPixelFormat, AVSampleFormat,
    AVStream,
};

use crate::util::{
    channels::ChannelLayout,
    marker::{Audio, Unknown, Video},
    pixels::PixelFormat,
    sampling::SampleFormat,
    time::{FrameRate, SampleRate, TimeBase, TimeBaseTicks},
    MediaType,
};

/// Holds information about the static configuration of a stream object
///
/// The marker type `T` can be one of `crate::tags::{Audio, Video, Data, Subtitle, Attachment, Unknown}`
/// The structure will only provide data relevant to current type of stream.
/// Conversion from an `Unknown` stream to a stream of a specific type can be accomplished
/// by one of the `as_X_stream()` types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamConfig<AV> {
    id: i32,
    index: usize,
    time_base: TimeBase,
    start_time: TimeBaseTicks,
    duration: TimeBaseTicks,
    num_frames: u64,
    codec_params: CodecParameters,
    _type: PhantomData<AV>,
}

impl<AV> StreamConfig<AV> {
    pub(crate) fn from_av_stream(stream: *mut AVStream) -> StreamConfig<Unknown> {
        debug_assert!(
            !stream.is_null(),
            "NULL Stream paassed to config struct constructor"
        );
        unsafe {
            StreamConfig {
                id: (*stream).id,
                index: {
                    debug_assert!((*stream).index >= 0);
                    (*stream).index as usize
                },
                time_base: TimeBase::from_av_rational(&(*stream).time_base),
                start_time: TimeBaseTicks::new((*stream).start_time as u64),
                duration: TimeBaseTicks::new((*stream).duration as u64),
                num_frames: (*stream).nb_frames as u64,
                codec_params: CodecParameters::from_av_params((*stream).codecpar),
                _type: PhantomData,
            }
        }
    }

    /// Convert a stream config pack to "Unknown" type
    ///
    /// This is most useful if you are storing a bunch of streams of various types
    /// in a homogeneous collection.
    pub fn as_unknown(self) -> StreamConfig<Unknown> {
        unsafe { std::mem::transmute(self) }
    }

    /// Get the time-base used for this stream
    pub fn time_base(&self) -> TimeBase {
        self.time_base
    }

    /// Get the index of this stream
    pub fn stream_index(&self) -> usize {
        self.index
    }
}

impl StreamConfig<Unknown> {
    /// Try to convert a stream of unknown type to another stream type
    pub fn try_as_type<T: MediaType>(self) -> Option<StreamConfig<T>> {
        if self.codec_params.media_type == T::MEDIA_TYPE {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }
}

impl StreamConfig<Audio> {
    /// How many channels does this stream contain
    pub fn num_channels(&self) -> u32 {
        self.codec_params.channel_layout.bits().count_ones()
    }

    /// What is the channel layout of this stream
    pub fn channel_layout(&self) -> ChannelLayout {
        self.codec_params.channel_layout
    }

    /// What is the sample rate of this stream
    pub fn sample_rate(&self) -> SampleRate {
        self.codec_params.sample_rate
    }

    /// What format are samples stored in this stream
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
        let av_fmt: AVSampleFormat = unsafe { std::mem::transmute(self.codec_params.format) };

        SampleFormat::from(av_fmt)
    }
}

impl StreamConfig<Video> {
    /// The width of a single frame of video
    pub fn width(&self) -> u32 {
        self.codec_params.width
    }

    /// The height of a single frame of vidoe
    pub fn height(&self) -> u32 {
        self.codec_params.height
    }

    /// The pixel format of a decoded frame in this stream
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
        let av_fmt: AVPixelFormat = unsafe { std::mem::transmute(self.codec_params.format) };

        PixelFormat::from(av_fmt)
    }

    /// Get the average frame-rate of this stream
    pub fn avg_frame_rate(&self) -> FrameRate {
        unimplemented!()
    }
}

/// Holds information about a stream that has been decoded.
///
/// This information may differ from the base stream information depending
/// on additional options passed to the codec during setup.
#[derive(Debug, Clone)]
pub struct DecodedStreamConfig<AV> {
    base: StreamConfig<AV>,
}

impl<AV> Deref for DecodedStreamConfig<AV> {
    type Target = StreamConfig<AV>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<AV> DecodedStreamConfig<AV> {
    // TODO: Determine what extra parameters a DecodedStream can/should have
    // its known that the values can differ, so we might need to re-read some of
    // those values from the Codec
    /// Create a configuration for a stream after the codec has be initialized
    ///
    /// # Safety
    /// The `_codec_ctx` must be a valid `AVCodecContext` that was
    /// initialized for the stream passed in `cfg`
    pub unsafe fn new(
        cfg: StreamConfig<AV>,
        _codec_ctx: *mut AVCodecContext,
    ) -> DecodedStreamConfig<AV> {
        DecodedStreamConfig { base: cfg }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodecParameters {
    // Parameters useful for setting up a codec
    media_type: AVMediaType,
    codec_id: AVCodecID,
    codec_tag: u32,
    profile: i32,
    level: i32,

    // Informational parameters
    bit_rate: u64,
    bits_per_coded_sample: u32,
    bits_per_raw_sample: u32,

    // Stupidly mixed audio/video information
    // Have to convert to the appropriate format type depending on the `media_type`
    format: i32,

    // Audio specific parameters
    channel_layout: ChannelLayout,
    sample_rate: SampleRate,

    // Video specific parameters
    width: u32,
    height: u32,
    video_delay: i32,
}

impl CodecParameters {
    unsafe fn from_av_params(param: *mut AVCodecParameters) -> CodecParameters {
        let channel_layout = ChannelLayout::from_bits_truncate((*param).channel_layout);

        // Sanity check that the parameters we are being given make sense
        // In the future we will return the channel count exclusivly by
        // counting the bits in the channel_layout, so just make sense of it
        // here
        assert_eq!(
            channel_layout.bits().count_ones(),
            (*param).channels as u32,
            "Channel layout and channel count do not match"
        );

        CodecParameters {
            media_type: (*param).codec_type,
            codec_id: (*param).codec_id,
            codec_tag: (*param).codec_tag,
            profile: (*param).profile,
            level: (*param).level,

            bit_rate: (*param).bit_rate as u64,
            bits_per_coded_sample: (*param).bits_per_coded_sample as u32,
            bits_per_raw_sample: (*param).bits_per_raw_sample as u32,

            format: (*param).format,

            channel_layout,
            sample_rate: SampleRate::new((*param).sample_rate as u32),

            width: (*param).width as u32,
            height: (*param).height as u32,
            video_delay: (*param).video_delay,
        }
    }
}
