use core::fmt;
use std::{borrow::Cow, ffi::CStr};

use ffav_sys::{av_get_sample_fmt_name, AVSampleFormat};

/// Information about the native sample format type of an audio stream
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    /// Planar u8 samples, each channel is its own data plane
    PlanarU8,
    /// Planar i16 samples, each channel is its own data plane
    PlanarI16,
    /// Planar i32 samples, each channel is its own data plane
    PlanarI32,
    /// Planar i64 samples, each channel is its own data plane
    PlanarI64,
    /// Planar f32 samples, each channel is its own data plane
    PlanarF32,
    /// Planar f64 samples, each channel is its own data plane
    PlanarF64,
    /// Packed u8 samples, each channel is stored in the same data plane sequentially
    PackedU8,
    /// Packed i16 samples, each channel is stored in the same data plane sequentially
    PackedI16,
    /// Packed i32 samples, each channel is stored in the same data plane sequentially
    PackedI32,
    /// Packed i64 samples, each channel is stored in the same data plane sequentially
    PackedI64,
    /// Packed f32 samples, each channel is stored in the same data plane sequentially
    PackedF32,
    /// Packed f64 samples, each channel is stored in the same data plane sequentially
    PackedF64,
}

impl SampleFormat {
    /// Convert this sample format to an `AVSampleFormat` usable by libav
    pub fn to_av_sample(self) -> AVSampleFormat {
        match self {
            Self::PackedU8 => AVSampleFormat::AV_SAMPLE_FMT_U8,
            Self::PackedI16 => AVSampleFormat::AV_SAMPLE_FMT_S16,
            Self::PackedI32 => AVSampleFormat::AV_SAMPLE_FMT_S32,
            Self::PackedI64 => AVSampleFormat::AV_SAMPLE_FMT_S64,
            Self::PackedF32 => AVSampleFormat::AV_SAMPLE_FMT_FLT,
            Self::PackedF64 => AVSampleFormat::AV_SAMPLE_FMT_DBL,

            Self::PlanarU8 => AVSampleFormat::AV_SAMPLE_FMT_U8P,
            Self::PlanarI16 => AVSampleFormat::AV_SAMPLE_FMT_S16P,
            Self::PlanarI32 => AVSampleFormat::AV_SAMPLE_FMT_S32P,
            Self::PlanarI64 => AVSampleFormat::AV_SAMPLE_FMT_S64P,
            Self::PlanarF32 => AVSampleFormat::AV_SAMPLE_FMT_FLTP,
            Self::PlanarF64 => AVSampleFormat::AV_SAMPLE_FMT_DBLP,
        }
    }

    /// Get the stringified name of this pixel format
    pub fn format_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(av_get_sample_fmt_name(self.to_av_sample())).to_string_lossy() }
    }
}

impl From<AVSampleFormat> for SampleFormat {
    fn from(other: AVSampleFormat) -> Self {
        match other {
            AVSampleFormat::AV_SAMPLE_FMT_U8 => Self::PackedU8,
            AVSampleFormat::AV_SAMPLE_FMT_S16 => Self::PackedI16,
            AVSampleFormat::AV_SAMPLE_FMT_S32 => Self::PackedI32,
            AVSampleFormat::AV_SAMPLE_FMT_S64 => Self::PackedI64,
            AVSampleFormat::AV_SAMPLE_FMT_FLT => Self::PackedF32,
            AVSampleFormat::AV_SAMPLE_FMT_DBL => Self::PackedF64,

            AVSampleFormat::AV_SAMPLE_FMT_U8P => Self::PlanarU8,
            AVSampleFormat::AV_SAMPLE_FMT_S16P => Self::PlanarI16,
            AVSampleFormat::AV_SAMPLE_FMT_S32P => Self::PlanarI32,
            AVSampleFormat::AV_SAMPLE_FMT_S64P => Self::PlanarI64,
            AVSampleFormat::AV_SAMPLE_FMT_FLTP => Self::PlanarF32,
            AVSampleFormat::AV_SAMPLE_FMT_DBLP => Self::PlanarF64,

            _ => panic!("Unknown sample format, or AV_SMAPLE_FMT_NB was returned"),
        }
    }
}

impl std::fmt::Display for SampleFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_name())
    }
}
