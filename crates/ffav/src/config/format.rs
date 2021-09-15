//! Configuration information for the root Format container

use ffav_sys::AVFormatContext;

use crate::util::{
    marker::Input,
    time::{TimeBase, TimeBaseTicks, Timestamp},
};
use std::{marker::PhantomData, num::NonZeroI64};

/// Holds informationa bout the configuration of a Format object
///
/// The marker type `IO` can be one of `crate::tags::{Input, Output}` depending
/// on if the
#[derive(Debug)]
pub struct FormatConfig<IO> {
    //
    // The following members are only set during demuxing, and thus are only
    // available for `FormatConfig<Input>` types.
    //
    start_time: TimeBaseTicks,
    duration: TimeBaseTicks,
    bit_rate: Option<NonZeroI64>,

    //
    // The following are relevant to both input and output Formats
    //
    num_streams: usize,
    num_chapters: usize,

    // Indicate if this is an input or output type
    _io: PhantomData<IO>,
}

impl FormatConfig<Input> {
    pub(crate) unsafe fn from_input_fmt_ctx(ctx: *mut AVFormatContext) -> Self {
        FormatConfig {
            start_time: TimeBaseTicks::new((*ctx).start_time as u64),
            duration: TimeBaseTicks::new((*ctx).duration as u64),
            num_streams: (*ctx).nb_streams as usize,

            bit_rate: NonZeroI64::new((*ctx).bit_rate),
            num_chapters: (*ctx).nb_chapters as usize,

            _io: PhantomData,
        }
    }

    /// The time of the first frame of the Format
    pub fn start_time(&self) -> Timestamp {
        self.start_time.to_timestamp(TimeBase::DEFAULT)
    }

    /// The duration of the open Format
    pub fn duration(&self) -> Timestamp {
        self.duration.to_timestamp(TimeBase::DEFAULT)
    }

    /// The bit-rate of the open Format
    ///
    /// Will return None if the value is not known
    pub fn bit_rate(&self) -> Option<NonZeroI64> {
        self.bit_rate
    }
}

impl<T> FormatConfig<T> {
    /// How many streams are in the open Format
    pub fn num_streams(&self) -> usize {
        self.num_streams
    }

    /// How many chapters are in the open Format
    pub fn num_chapters(&self) -> usize {
        self.num_chapters
    }
}
