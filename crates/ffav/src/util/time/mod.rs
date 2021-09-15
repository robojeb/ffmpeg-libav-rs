//! Types and traits for temporal indexing of media
//!
//!

mod audio;
mod generic;
mod video;

pub use audio::*;
pub use generic::*;
pub use video::*;

use num_rational::Ratio;

use crate::config::StreamConfig;

type Rational = Ratio<u64>;

/// A type which can be converted into a timestamp using the context of a Stream
pub trait IntoStreamTimestamp<AV> {
    /// Convert to a timestamp
    fn into(self, stream: &StreamConfig<AV>) -> Timestamp;
}

/// Anything which can be directly converted to a timestamp can be converted
/// to stream specific timestamp
impl<T, AV> IntoStreamTimestamp<AV> for T
where
    T: Into<Timestamp>,
{
    fn into(self, stream: &StreamConfig<AV>) -> Timestamp {
        let raw = self.into();

        raw.with_new_timebase(stream.time_base())
    }
}
