//! Utility structs and functions for dealing with time

use super::{audio::*, video::*, Rational};
use ffav_sys::{AVRational, AV_TIME_BASE};
use std::{fmt, ops::Rem, time::Duration};

/// The minimum temporal resolution for timestamps of a Stream or Format
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeBase(Rational);

impl TimeBase {
    /// The default time base used by ffmpeg for Format timestamps
    pub const DEFAULT: TimeBase = TimeBase(Rational::new_raw(1, AV_TIME_BASE as u64));
    /// A TimeBase in milliseconds
    pub const MILLISECONDS: TimeBase = TimeBase(Rational::new_raw(1, 1000));

    /// Create a time-base from an `AVRational` type
    pub fn from_av_rational(rational: &AVRational) -> Self {
        TimeBase(Rational::new(rational.num as u64, rational.den as u64))
    }

    /// Create a new TimeBase
    pub fn new(numer: u64, denom: u64) -> Self {
        TimeBase(Rational::new(numer, denom))
    }
}

impl fmt::Debug for TimeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TimeBase({}/{})", self.0.numer(), self.0.denom())
    }
}

impl fmt::Display for TimeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0.numer(), self.0.denom())
    }
}

/// A timestamp or duration in an unknown TimeBase, recorded as a count of
/// TimeBase units.
///
/// Without an associated TimeBase this does not provide any useful
/// information and two instances cannot be compared
#[derive(Debug, Clone, Copy, Hash)]
pub struct TimeBaseTicks(u64);

impl TimeBaseTicks {
    /// Create aa new count of Ticks against an unknown TimeBase
    pub fn new(ticks: u64) -> TimeBaseTicks {
        TimeBaseTicks(ticks)
    }

    /// Associate this TimeBaseTicks with a TimeBase to get a true Timestamp
    pub fn to_timestamp(self, base: TimeBase) -> Timestamp {
        Timestamp { ticks: self, base }
    }

    /// Get the count of ticks as i64 for use with most libav functions
    pub fn as_av_timestamp(self) -> i64 {
        self.0 as i64
    }
}

/// A single point in time in a stream
///
/// The resolution of this Timestamp is dependant on the associated TimeBase.
#[derive(Debug, Clone, Copy, Hash)]
pub struct Timestamp {
    ticks: TimeBaseTicks,
    base: TimeBase,
}

impl Timestamp {
    /// Convert this `Timestamp` to using a different time base
    ///
    /// This may result in a loss of precision depending on the new TimeBase.
    /// The new Timestamp should be the nearest valid time in the new TimeBase
    /// prior to the old Timestamp
    pub fn with_new_timebase(self, other: TimeBase) -> Self {
        if self.base == other {
            self
        } else {
            unimplemented!()
        }
    }

    /// Convert this `Timestamp` to a `std::time::Duration`
    ///
    /// The provided duration will have millisecond resolution
    pub fn as_duration(self) -> Duration {
        let millis = self.with_new_timebase(TimeBase::MILLISECONDS);

        Duration::from_millis(millis.get_ticks().0)
    }

    /// Get the underlying tick count of this timestamp in its current time-base
    pub fn get_ticks(self) -> TimeBaseTicks {
        self.ticks
    }

    /// Convert this `Timestamp` into a sample number given a known sample rate
    pub fn sample(self, _sample_rate: SampleRate) -> Sample {
        unimplemented!()
    }

    /// Convert this timestamp into the index of the corresponding frame
    ///
    /// The frame number returned will be the nearest frame before the `Timestamp`
    /// NOTE: Because the `frame_rate` is typically an average, this may not always
    /// return the exact frame number.
    pub fn frame(self, _frame_rate: ()) -> Frame {
        unimplemented!()
    }
}

impl std::convert::From<Duration> for Timestamp {
    fn from(d: Duration) -> Self {
        Timestamp {
            ticks: TimeBaseTicks::new(d.as_millis() as u64),
            base: TimeBase::MILLISECONDS,
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = self.as_duration();
        let mut total_millis = duration.as_millis();

        let millis = total_millis.rem(1000);
        total_millis = (total_millis - millis) / 1000;

        let secs = total_millis.rem(60);
        total_millis = (total_millis - secs) / 60;

        let mins = total_millis.rem(60);
        total_millis = (total_millis - mins) / 60;

        let hours = total_millis;

        write!(f, "{:02}:{:02}:{02}.{03}", hours, mins, secs, millis)
    }
}

// /// Convert a string from the format `HH+:MM:SS.mmm` to a duration
// impl<'a> TryFrom<&'a str> for Timestamp {
//     type Error = Error;

//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         let slices = value.splitn(n, pat)
//     }
// }

// impl TryFrom<String> for Timestamp {
//     type Error = Error;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         value.as_str().try_into()
//     }
// }
