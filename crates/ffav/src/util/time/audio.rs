use crate::util::marker::Audio;

use super::{TimeBase, TimeBaseTicks};

/// The sampling rate of an audio stream as an integer number of Hz
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleRate(u32);

impl SampleRate {
    /// Create a new SampleRate from a u32 number of Hertz
    pub fn new(hz: u32) -> Self {
        SampleRate(hz)
    }

    /// Get the sampling rate as a u32 number of Hertz
    pub fn as_hz(self) -> u32 {
        self.0
    }
}

/// The sample number of a single sample from an Audio stream

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sample(u64);

impl super::IntoStreamTimestamp<Audio> for Sample {
    fn into(self, stream: &crate::config::StreamConfig<Audio>) -> super::Timestamp {
        TimeBaseTicks::new(self.0)
            .to_timestamp(TimeBase::new(1, stream.parameters().sample_rate().0 as u64))
    }
}
