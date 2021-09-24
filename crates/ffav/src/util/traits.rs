//! Traits which help enforce safety at the type level

use ffav_sys::AVMediaType;

/// A Trait for marker types which represent a type of Media stream
pub trait MediaMarker {
    /// The expected AVMediaType for items with this marker
    const MEDIA_TYPE: AVMediaType;
}
