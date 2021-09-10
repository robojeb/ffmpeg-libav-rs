//! Utility structs and functions for dealing with time

use ffav_sys::{AVRational, AV_TIME_BASE};
use num_rational::Rational64;
use std::{fmt, ops::Rem, time::Duration};

/// The default time base used by ffmpeg for Format timestamps
pub const DEFAULT_TIME_BASE: TimeBase = TimeBase(Rational64::new_raw(1, AV_TIME_BASE as i64));

/// The minimum temporal resolution for timestamps of a Stream of Format
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeBase(Rational64);

impl TimeBase {
    /// Create a time-base from an `AVRational` type
    pub(crate) fn from_av_rational(rational: &AVRational) -> Self {
        TimeBase(Rational64::new(rational.num as i64, rational.den as i64))
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
/// TimeBase units
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeBaseTicks(u64);

impl TimeBaseTicks {
    pub fn new(ticks: u64) -> TimeBaseTicks {
        TimeBaseTicks(ticks)
    }

    pub fn to_timestamp(&self, base: TimeBase) -> Timestamp {
        Timestamp { ticks: *self, base }
    }
}

/// A true timestamp with associated TimeBase
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    ticks: TimeBaseTicks,
    base: TimeBase,
}

impl Timestamp {
    /// Convert the timestamp into a `std::time::Duration` type indicating the
    /// duration from the start of the Format or Stream
    pub fn as_duration(&self) -> Duration {
        let millis = (self.base.0 * self.ticks.0 as i64) * 1000;

        Duration::from_millis(millis.to_integer() as u64)
    }
}

impl std::convert::From<Duration> for Timestamp {
    fn from(d: Duration) -> Self {
        Timestamp {
            ticks: TimeBaseTicks::new(d.as_millis() as u64),
            base: TimeBase(Rational64::new(1, 1000)),
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
