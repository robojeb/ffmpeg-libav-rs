use ffav_sys::AVColorPrimaries;

fflib_version::ffversion! {

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorPrimary {
    /// Unspecified color primary
    Unspecified,
    /// also ITU-R BT1361 / IEC 61966-2-4 / SMPTE RP177 Annex B
    BT709,
    /// also FCC Title 47 Code of Federal Regulations 73.682 (a)(20)
    BT470M,
    /// also ITU-R BT601-6 625 / ITU-R BT1358 625 / ITU-R BT1700 625 PAL & SECAM
    BT470BG,
    /// also ITU-R BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC
    SMPTE170M,
    /// Functionally the same as `ColorPrimary:::SMPTE170M`
    SMPTE240M,
    /// colour filters using Illuminant C
    FILM,
    /// ITU-R BT2020
    BT2020,
    /// SMPTE ST 428-1 (CIE 1931 XYZ)
    SMPTE428,
    /// SMPTE ST 431-2 (2011) / DCI P3
    SMPTE431,
    /// SMPTE ST 432-1 (2010) / P3 D65 / Display P3
    SMPTE432,

    #[libavformat(since(58.76))] {
    /// EBU Tech. 3213-E (nothing there) / one of JEDEC P22 group phosphors
    EBU3213,
    }

    #[libavformat(before(58.76))] {
    ///JEDEC P22 group phosphors
    JEDECP22,
    }
}

impl From<AVColorPrimaries> for ColorPrimary {
    fn from(pri: AVColorPrimaries) -> Self {
        match pri {
            AVColorPrimaries::AVCOL_PRI_BT709 => ColorPrimary::BT709,
            AVColorPrimaries::AVCOL_PRI_UNSPECIFIED => ColorPrimary::Unspecified,
            AVColorPrimaries::AVCOL_PRI_BT470M => ColorPrimary::BT470M,
            AVColorPrimaries::AVCOL_PRI_BT470BG => ColorPrimary::BT470BG,
            AVColorPrimaries::AVCOL_PRI_SMPTE170M => ColorPrimary::SMPTE170M,
            AVColorPrimaries::AVCOL_PRI_SMPTE240M => ColorPrimary::SMPTE240M,
            AVColorPrimaries::AVCOL_PRI_FILM => ColorPrimary::FILM,
            AVColorPrimaries::AVCOL_PRI_BT2020 => ColorPrimary::BT2020,
            AVColorPrimaries::AVCOL_PRI_SMPTE428 => ColorPrimary::SMPTE428,
            AVColorPrimaries::AVCOL_PRI_SMPTE431 => ColorPrimary::SMPTE431,
            AVColorPrimaries::AVCOL_PRI_SMPTE432 => ColorPrimary::SMPTE432,
            #[libavformat(before(58.76))] {
            AVColorPrimaries::AVCOL_PRI_JEDEC_P22 => ColorPrimary::JEDECP22,
            }
            #[libavformat(since(58.76))] {
            AVColorPrimaries::AVCOL_PRI_EBU3213 => ColorPrimary::EBU3213,
            }
            _ => panic!("Unknown or Reserved color primary was provided"),
        }
    }
}

impl From<ColorPrimary> for AVColorPrimaries {
    fn from(pri: ColorPrimary) -> Self {
        match pri {
            ColorPrimary::BT709 => AVColorPrimaries::AVCOL_PRI_BT709,
            ColorPrimary::Unspecified => AVColorPrimaries::AVCOL_PRI_UNSPECIFIED,
            ColorPrimary::BT470M => AVColorPrimaries::AVCOL_PRI_BT470M,
            ColorPrimary::BT470BG => AVColorPrimaries::AVCOL_PRI_BT470BG,
            ColorPrimary::SMPTE170M => AVColorPrimaries::AVCOL_PRI_SMPTE170M,
            ColorPrimary::SMPTE240M => AVColorPrimaries::AVCOL_PRI_SMPTE240M,
            ColorPrimary::FILM => AVColorPrimaries::AVCOL_PRI_FILM,
            ColorPrimary::BT2020 => AVColorPrimaries::AVCOL_PRI_BT2020,
            ColorPrimary::SMPTE428 => AVColorPrimaries::AVCOL_PRI_SMPTE428,
            ColorPrimary::SMPTE431 => AVColorPrimaries::AVCOL_PRI_SMPTE431,
            ColorPrimary::SMPTE432 => AVColorPrimaries::AVCOL_PRI_SMPTE432,
            #[libavformat(since(58.76))] {
            ColorPrimary::EBU3213 => AVColorPrimaries::AVCOL_PRI_EBU3213,
            }
            #[libavformat(before(58.76))] {
            ColorPrimary::JEDECP22 => AVColorPrimaries::AVCOL_PRI_JEDEC_P22,
            }
            _ => unimplemented!(),
        }
    }
}

}
