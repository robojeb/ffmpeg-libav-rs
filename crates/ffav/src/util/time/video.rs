use crate::util::marker::Video;

use super::{Rational, TimeBase, TimeBaseTicks};

/// The frame-rate of a Video stream, may be fractional

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameRate(Rational);

/// The index of a single frame in a Video stream

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Frame(u64);

impl super::IntoStreamTimestamp<Video> for Frame {
    fn into(self, stream: &crate::config::StreamConfig<Video>) -> super::Timestamp {
        let frame_rate = stream.avg_frame_rate();

        TimeBaseTicks::new(self.0)
            .to_timestamp(TimeBase::new(*frame_rate.0.numer(), *frame_rate.0.denom()))
    }
}
