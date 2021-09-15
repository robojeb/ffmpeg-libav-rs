#![allow(missing_docs)]
use bitflags::bitflags;

use ffav_sys::{
    AV_CH_BACK_CENTER, AV_CH_BACK_LEFT, AV_CH_BACK_RIGHT, AV_CH_FRONT_CENTER, AV_CH_FRONT_LEFT,
    AV_CH_FRONT_LEFT_OF_CENTER, AV_CH_FRONT_RIGHT, AV_CH_FRONT_RIGHT_OF_CENTER,
    AV_CH_LAYOUT_3POINT1, AV_CH_LAYOUT_MONO, AV_CH_LAYOUT_STEREO, AV_CH_LAYOUT_SURROUND,
    AV_CH_LOW_FREQUENCY, AV_CH_LOW_FREQUENCY_2, AV_CH_SIDE_LEFT, AV_CH_SIDE_RIGHT,
    AV_CH_STEREO_LEFT, AV_CH_STEREO_RIGHT, AV_CH_SURROUND_DIRECT_LEFT, AV_CH_SURROUND_DIRECT_RIGHT,
    AV_CH_TOP_BACK_CENTER, AV_CH_TOP_BACK_LEFT, AV_CH_TOP_BACK_RIGHT, AV_CH_TOP_CENTER,
    AV_CH_TOP_FRONT_CENTER, AV_CH_TOP_FRONT_LEFT, AV_CH_TOP_FRONT_RIGHT, AV_CH_WIDE_LEFT,
    AV_CH_WIDE_RIGHT,
};

bitflags! {
    /// The Layout of channels in an Audio stream
    pub struct ChannelLayout: u64 {
        /// The Front left speaker
        const FRONT_LEFT = AV_CH_FRONT_LEFT as u64;
        /// The Front right speaker
        const FRONT_RIGHT = AV_CH_FRONT_RIGHT as u64;
        /// The Front center spoeaker
        const FRONT_CENTER = AV_CH_FRONT_CENTER as u64;
        /// A low frequency channel (eg. subwoofer)
        const LOW_FREQ = AV_CH_LOW_FREQUENCY as u64;
        const BACK_LEFT = AV_CH_BACK_LEFT as u64;
        const BACK_RIGHT = AV_CH_BACK_RIGHT as u64;
        const FRONT_LEFT_OF_CENTER = AV_CH_FRONT_LEFT_OF_CENTER as u64;
        const FRONT_RIGHT_OF_CENTER = AV_CH_FRONT_RIGHT_OF_CENTER as u64;
        const BACK_CENTER = AV_CH_BACK_CENTER as u64;
        const SIDE_LEFT = AV_CH_SIDE_LEFT as u64;
        const SIDE_RIGHT = AV_CH_SIDE_RIGHT as u64;
        const TOP_CENTER = AV_CH_TOP_CENTER as u64;
        const TOP_FRONT_LEFT = AV_CH_TOP_FRONT_LEFT as u64;
        const TOP_FRONT_RIGHT = AV_CH_TOP_FRONT_RIGHT as u64;
        const TOP_FRONT_CENTER = AV_CH_TOP_FRONT_CENTER as u64;
        const TOP_BACK_CENTER = AV_CH_TOP_BACK_CENTER as u64;
        const TOP_BACK_LEFT = AV_CH_TOP_BACK_LEFT as u64;
        const TOP_BACK_RIGHT = AV_CH_TOP_BACK_RIGHT as u64;
        const STEREO_LEFT = AV_CH_STEREO_LEFT as u64;
        const STEREO_RIGHT = AV_CH_STEREO_RIGHT as u64;
        const WIDE_LEFT = AV_CH_WIDE_LEFT as u64;
        const WIDE_RIGHT = AV_CH_WIDE_RIGHT as u64;
        const SURROUND_DIRECT_LEFT = AV_CH_SURROUND_DIRECT_LEFT as u64;
        const SURROUND_DIRECT_RIGHT = AV_CH_SURROUND_DIRECT_RIGHT as u64;
        const LOW_FREQ_2 = AV_CH_LOW_FREQUENCY_2 as u64;
    }
}

impl ChannelLayout {
    /// Single central channel
    pub const LAYOUT_MONO: ChannelLayout =
        ChannelLayout::from_bits_truncate(AV_CH_LAYOUT_MONO as u64);
    /// Left and right channels
    pub const LAYOUT_STEREO: ChannelLayout =
        ChannelLayout::from_bits_truncate(AV_CH_LAYOUT_STEREO as u64);
    /// Front, Left and Right channels
    pub const LAYOUT_SURROUND: ChannelLayout =
        ChannelLayout::from_bits_truncate(AV_CH_LAYOUT_SURROUND as u64);
    /// Surround with subwoofer
    pub const LAYOUT_3_POINT_1: ChannelLayout =
        ChannelLayout::from_bits_truncate(AV_CH_LAYOUT_3POINT1 as u64);
}
