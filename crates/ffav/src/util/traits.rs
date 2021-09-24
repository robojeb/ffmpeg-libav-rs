//! Traits which help enforce safety at the type level

mod filter;

use ffav_sys::AVMediaType;

pub use filter::*;

/// A Trait for marker types which represent a type of Media stream
pub trait MediaMarker {
    /// The expected AVMediaType for items with this marker
    const MEDIA_TYPE: AVMediaType;
}
