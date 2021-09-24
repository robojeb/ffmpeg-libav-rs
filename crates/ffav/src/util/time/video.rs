use crate::util::marker::Video;

use super::{Rational, TimeBase, TimeBaseTicks};

/// The frame-rate of a Video stream, may be fractional

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameRate(Rational);

impl FrameRate {
    /// Standard framerate for film 24fps
    pub const FILM_STANDARD: FrameRate = FrameRate(Rational::new_raw(24, 1));
    /// Standard framerate for PAL video 25fps
    pub const PAL: FrameRate = FrameRate(Rational::new_raw(25, 1));
    /// Standard framerate for NTSC video 29.97fps
    pub const NTSC: FrameRate = FrameRate(Rational::new_raw(2997, 1000));
}

/// The index of a single frame in a Video stream

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Frame(u64);

impl super::IntoStreamTimestamp<Video> for Frame {
    fn into(self, stream: &crate::config::StreamConfig<Video>) -> super::Timestamp {
        let frame_rate = stream.parameters().avg_frame_rate();

        TimeBaseTicks::new(self.0)
            .to_timestamp(TimeBase::new(*frame_rate.0.numer(), *frame_rate.0.denom()))
    }
}
