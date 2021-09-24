//! Types and traits for temporal indexing of media
//!
//! There are four primary ways to index a stream:
//!   1. Generic index in TimeBase ticks
//!   1. Realtime (eg std::time::Duration from start of media)
//!   1. Sample number (for Audio only)
//!   1. Frame number (for Video only)
//!
//! This library has chosen to use the `TimeBase` + `TimeBaseTicks` generic method
//! as its primary method of recording timestamps. This is represented in the
//! type `TimeStamp`.
//!
//! With appropriate context all the other temporal indexing methods can be converted
//! to/from a `Timestamp`. Please refer to the specific documentation for each
//! method for details.

mod audio;
mod generic;
mod video;

pub use audio::*;
pub use generic::*;
pub use video::*;

use crate::config::StreamConfig;
use num_rational::Ratio;

use super::traits::MediaMarker;

/// Internal rational type for storing TimeBase and Framerate
///
/// Where AVRational is a signed 32bit type, we are using an unsigned 64-bit
/// type because TimeBase and FrameRate should never be negative. 64-bits also
/// allows more precision.
type Rational = Ratio<u64>;

/// A type which can be converted into a timestamp using the context of a Stream
pub trait IntoStreamTimestamp<AV: MediaMarker> {
    /// Convert to a timestamp
    fn into(self, stream: &StreamConfig<AV>) -> Timestamp;
}

/// Anything which can be directly converted to a timestamp can be converted
/// to stream specific timestamp
impl<T, AV: MediaMarker> IntoStreamTimestamp<AV> for T
where
    T: Into<Timestamp>,
{
    fn into(self, stream: &StreamConfig<AV>) -> Timestamp {
        let raw = self.into();

        raw.with_new_timebase(stream.time_base())
    }
}
