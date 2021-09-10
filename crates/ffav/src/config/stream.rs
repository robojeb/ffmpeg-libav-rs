use std::{marker::PhantomData, ops::Deref};

use ffav_sys::{AVCodecParameters, AVMediaType, AVSampleFormat, AVStream};

use crate::{
    tags::{Audio, Unknown},
    util::{
        channels::ChannelLayout,
        sampling::SampleFormat,
        time::{TimeBase, TimeBaseTicks},
        MediaType,
    },
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

    pub fn as_unknown(self) -> StreamConfig<Unknown> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn try_as_type<T: MediaType>(self) -> Option<StreamConfig<T>> {
        if self.codec_params.media_type == T::MEDIA_TYPE {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }

    pub fn time_base(&self) -> &TimeBase {
        &self.time_base
    }
}

/// Holds information about a stream that has been decoded.
///
/// This may hold additional stream type dependent information like
/// sampling rate, sample type, or frame format.
/// Filters will update the decoded stream configuration information based
/// on their expected operation.
#[derive(Debug, Clone)]
pub struct DecodedStreamConfig<AV> {
    base: StreamConfig<AV>,
    sample_format: SampleFormat,
    channel_layout: ChannelLayout,
    sample_rate: u32,
}

impl<AV> Deref for DecodedStreamConfig<AV> {
    type Target = StreamConfig<AV>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<AV> DecodedStreamConfig<AV> {
    pub fn new(
        cfg: StreamConfig<AV>,
        sample_fmt: AVSampleFormat,
        channel_layout: u64,
        sample_rate: u32,
    ) -> DecodedStreamConfig<AV> {
        DecodedStreamConfig {
            base: cfg,
            sample_format: sample_fmt.into(),
            channel_layout: ChannelLayout::from_bits_truncate(channel_layout),
            sample_rate,
        }
    }
}

impl DecodedStreamConfig<Audio> {
    pub fn sample_format(&self) -> &SampleFormat {
        &self.sample_format
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        self.channel_layout
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodecParameters {
    media_type: AVMediaType,
}

impl CodecParameters {
    unsafe fn from_av_params(param: *mut AVCodecParameters) -> CodecParameters {
        CodecParameters {
            media_type: (*param).codec_type,
        }
    }
}
