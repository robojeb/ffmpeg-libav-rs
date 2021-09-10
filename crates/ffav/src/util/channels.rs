use bitflags::bitflags;

use ffav_sys::{
    AV_CH_BACK_LEFT, AV_CH_BACK_RIGHT, AV_CH_FRONT_CENTER, AV_CH_FRONT_LEFT, AV_CH_FRONT_RIGHT,
    AV_CH_LAYOUT_3POINT1, AV_CH_LAYOUT_MONO, AV_CH_LAYOUT_STEREO, AV_CH_LAYOUT_SURROUND,
    AV_CH_LOW_FREQUENCY,
};

bitflags! {
    pub struct ChannelLayout: u64 {
        const FRONT_LEFT = AV_CH_FRONT_LEFT as u64;
        const FRONT_RIGHT = AV_CH_FRONT_RIGHT as u64;
        const FRONT_CENTER = AV_CH_FRONT_CENTER as u64;
        const LOW_FREQ = AV_CH_LOW_FREQUENCY as u64;
        const BACK_LEFT = AV_CH_BACK_LEFT as u64;
        const BACK_RIGHT = AV_CH_BACK_RIGHT as u64;
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
