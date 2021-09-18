#![allow(non_camel_case_types)]
use super::Endian;
use ffav_sys::{av_get_pix_fmt_name, AVPixelFormat};
use std::{borrow::Cow, ffi::CStr, fmt};

pub enum ColorEncoding {
    /// YUV and YUVA encodings
    YUV(),
    /// RGB Encodings
    RGB(),
    /// XYZ Encodings
    XYZ(),
    /// Grayscale encodings
    Gray(),
    /// Indexed color encodings
    Indexed(),
    /// Hardware specific encodings
    Hardware(),
}

fflib_version::ffversion! {

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    NONE,
    /// planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P,
    /// packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr
    YUYV422,
    /// packed RGB 8:8:8, 24bpp, RGBRGB...
    RGB24,
    /// packed RGB 8:8:8, 24bpp, BGRBGR...
    BGR24,
    /// planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P,
    /// planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P,
    /// planar YUV 4:1:0,  9bpp, (1 Cr & Cb sample per 4x4 Y samples)
    YUV410P,
    /// planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples)
    YUV411P,
    ///        Y        ,  8bpp
    GRAY8,
    ///        Y        ,  1bpp, 0 is white, 1 is black, in each byte pixels are ordered from the msb to the lsb
    MONOWHITE,
    ///        Y        ,  1bpp, 0 is black, 1 is white, in each byte pixels are ordered from the msb to the lsb
    MONOBLACK,
    /// 8 bits with AV_PIX_FMT_RGB32 palette
    PAL8,
    /// planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV420P and setting color_range
    YUVJ420P,
    /// planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV422P and setting color_range
    YUVJ422P,
    /// planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV444P and setting color_range
    YUVJ444P,
    /// packed YUV 4:2:2, 16bpp, Cb Y0 Cr Y1
    UYVY422,
    /// packed YUV 4:1:1, 12bpp, Cb Y0 Y1 Cr Y2 Y3
    UYYVYY411,
    /// packed RGB 3:3:2,  8bpp, (msb)2B 3G 3R(lsb)
    BGR8,
    /// packed RGB 1:2:1 bitstream,  4bpp, (msb)1B 2G 1R(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    BGR4,
    /// packed RGB 1:2:1,  8bpp, (msb)1B 2G 1R(lsb)
    BGR4_BYTE,
    /// packed RGB 3:3:2,  8bpp, (msb)2R 3G 3B(lsb)
    RGB8,
    /// packed RGB 1:2:1 bitstream,  4bpp, (msb)1R 2G 1B(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    RGB4,
    /// packed RGB 1:2:1,  8bpp, (msb)1R 2G 1B(lsb)
    RGB4_BYTE,
    /// planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    NV12,
    /// planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte V and the following byte U)
    NV21,
    /// packed ARGB 8:8:8:8, 32bpp, ARGBARGB...
    ARGB,
    /// packed RGBA 8:8:8:8, 32bpp, RGBARGBA...
    RGBA,
    /// packed ABGR 8:8:8:8, 32bpp, ABGRABGR...
    ABGR,
    /// packed BGRA 8:8:8:8, 32bpp, BGRABGRA...
    BGRA,
    ///        Y        , 16bpp
    GRAY16(Endian),
    /// planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples)
    YUV440P,
    /// planar YUV 4:4:0 full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV440P and setting color_range
    YUVJ440P,
    /// planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P,
    /// packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored in the specified `Endian`
    RGB48(Endian),
    /// packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb)
    RGB565(Endian),
    /// packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb)   , X=unused/undefined
    RGB555(Endian),
    /// packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb)
    BGR565(Endian),
    /// packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb)   , X=unused/undefined
    BGR555(Endian),
    /// HW acceleration through VA API at motion compensation entry-point, Picture.data[3] contains a vaapi_render_state struct which contains macroblocks as well as various fields extracted from headers.
    VAAPI_MOCO,
    /// HW acceleration through VA API at IDCT entry-point, Picture.data[3] contains a vaapi_render_state struct which contains fields extracted from headers.
    VAAPI_IDCT,
    /// HW decoding through VA API, Picture.data[3] contains a VASurfaceID.
    VAAPI_VLD,
    /// planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P16(Endian),
    /// planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P16(Endian),
    /// planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P16(Endian),
    /// HW decoding through DXVA2, Picture.data[3] contains a LPDIRECT3DSURFACE9 pointer
    DXVA2_VLD,
    /// packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), X=unused/undefined
    RGB444(Endian),
    /// packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), X=unused/undefined
    BGR444(Endian),
    /// 8 bits gray, 8 bits alpha
    YA8,
    /// packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored in the specified `Endian`
    BGR48(Endian),
    /// planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P9(Endian),
    /// planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P10(Endian),
    /// planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P10(Endian),
    /// planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P9(Endian),
    /// planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P10(Endian),
    /// planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P9(Endian),
    /// planar GBR 4:4:4 24bpp
    GBRP,
    /// planar GBR 4:4:4 27bpp
    GBRP9(Endian),
    /// planar GBR 4:4:4 30bpp
    GBRP10(Endian),
    /// planar GBR 4:4:4 48bpp
    GBRP16(Endian),
    /// planar YUV 4:2:2 24bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    YUVA422P,
    /// planar YUV 4:4:4 32bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    YUVA444P,
    /// planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P9(Endian),
    /// planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    YUVA422P9(Endian),
    /// planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    YUVA444P9(Endian),
    /// planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P10(Endian),
    /// planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    YUVA422P10(Endian),
    /// planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    YUVA444P10(Endian),
    /// planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P16(Endian),
    /// planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    YUVA422P16(Endian),
    /// planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    YUVA444P16(Endian),
    /// HW acceleration through VDPAU, Picture.data[3] contains a VdpVideoSurface
    VDPAU,
    /// packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored in the specified `Endian`, the 4 lower bits are set to 0
    XYZ12(Endian),
    /// interleaved chroma YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    NV16,
    /// interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples)
    NV20(Endian),
    /// packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored in the specified `Endian`
    RGBA64(Endian),
    /// packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored in the specified `Endian`
    BGRA64(Endian),
    /// packed YUV 4:2:2, 16bpp, Y0 Cr Y1 Cb
    YVYU422,
    /// 16 bits gray, 16 bits alpha
    YA16(Endian),
    /// planar GBRA 4:4:4:4 32bpp
    GBRAP,
    /// planar GBRA 4:4:4:4 64bpp
    GBRAP16(Endian),
    /**
     *  HW acceleration through QSV, data[3] contains a pointer to the
     *  mfxFrameSurface1 structure.
     */
    QSV,
    /**
     * HW acceleration though MMAL, data[3] contains a pointer to the
     * MMAL_BUFFER_HEADER_T structure.
     */
    MMAL,
    /// HW decoding through Direct3D11 via old API, Picture.data[3] contains a ID3D11VideoDecoderOutputView pointer
    D3D11VA_VLD,
    /**
     * HW acceleration through CUDA. data[i] contain CUdeviceptr pointers
     * exactly as for system memory frames.
     */
    CUDA,
    /// packed RGB 8:8:8, 32bpp, XRGBXRGB...   X=unused/undefined
    XRGB,
    /// packed RGB 8:8:8, 32bpp, RGBXRGBX...   X=unused/undefined
    RGBX,
    /// packed BGR 8:8:8, 32bpp, XBGRXBGR...   X=unused/undefined
    XBGR,
    /// packed BGR 8:8:8, 32bpp, BGRXBGRX...   X=unused/undefined
    BGRX,
    /// planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P12(Endian),
    /// planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P14(Endian),
    /// planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P12(Endian),
    /// planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P14(Endian),
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P12(Endian),
    /// planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P14(Endian),
    /// planar GBR 4:4:4 36bpp
    GBRP12(Endian),
    /// planar GBR 4:4:4 42bpp
    GBRP14(Endian),
    /// planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples) full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV411P and setting color_range
    YUVJ411P,
    /// bayer, BGBG..(odd line), GRGR..(even line), 8-bit samples
    BAYER_BGGR8,
    /// bayer, RGRG..(odd line), GBGB..(even line), 8-bit samples
    BAYER_RGGB8,
    /// bayer, GBGB..(odd line), RGRG..(even line), 8-bit samples
    BAYER_GBRG8,
    /// bayer, GRGR..(odd line), BGBG..(even line), 8-bit samples
    BAYER_GRBG8,
    /// bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples
    BAYER_BGGR16(Endian),
    /// bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples
    BAYER_RGGB16(Endian),
    /// bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples
    BAYER_GBRG16(Endian),
    /// bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples
    BAYER_GRBG16(Endian),
    /// XVideo Motion Acceleration via common packet passing
    XVMC,
    /// planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples)
    YUV440P10(Endian),
    /// planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples)
    YUV440P12(Endian),
    /// packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples)
    AYUV64(Endian),
    /// hardware decoding through Videotoolbox
    VIDEOTOOLBOX,
    /// like NV12, with 10bpp per component, data in the high bits, zeros in the low bits
    P010(Endian),
    /// planar GBR 4:4:4:4 48bpp
    GBRAP12(Endian),
    /// planar GBR 4:4:4:4 40bpp
    GBRAP10(Endian),
    /// hardware decoding through MediaCodec
    MEDIACODEC,
    ///        Y        , 12bpp
    GRAY12(Endian),
    ///        Y        , 10bpp
    GRAY10(Endian),
    /// like NV12, with 16bpp per component
    P016(Endian),
    /**
     * Hardware surfaces for Direct3D11.
     *
     * This is preferred over the legacy AV_PIX_FMT_D3D11VA_VLD. The new D3D11
     * hwaccel API and filtering support AV_PIX_FMT_D3D11 only.
     *
     * data[0] contains a ID3D11Texture2D pointer, and data[1] contains the
     * texture array index of the frame as intptr_t if the ID3D11Texture2D is
     * an array texture (or always 0 if it's a normal texture).
     */
    D3D11,
    ///        Y        , 9bpp
    Gray9(Endian),
    /// IEEE-754 single precision planar GBR 4:4:4,     96bpp
    GBRPF32(Endian),
    /// IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp
    GBRAPF32(Endian),
    /**
     * DRM-managed buffers exposed through PRIME buffer sharing.
     *
     * data[0] points to an AVDRMFrameDescriptor.
     */
    DrmPrime,
    /**
     * Hardware surfaces for OpenCL.
     *
     * data[i] contain 2D image objects (typed in C as cl_mem, used
     * in OpenCL as image2d_t) for each plane of the surface.
     */
    OpenCL,
    ///        Y        , 14bpp
    GRAY14(Endian),
    /// IEEE-754 single precision Y, 32bpp
    GRAYF32(Endian),
    /// planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha
    YUVA422P12(Endian),
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha
    YUVA444P12(Endian),
    /// planar YUV 4:4:4, 24bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    NV24,
    /// planar YUV 4:4:4, 24bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte V and the following byte U)
    NV42,
    #[libavformat(since(58.76))] {
    /**
     * Vulkan hardware images.
     *
     * data[0] points to an AVVkFrame
     */
    Vulkan,
    /// packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits
    Y210(Endian),
    /// packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), X=unused/undefined
    X2RGB10(Endian),
    }
}

impl PixelFormat {
    /// Get the stringified name of this pixel format
    pub fn format_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(av_get_pix_fmt_name(self.into())).to_string_lossy() }
    }
}

impl From<&PixelFormat> for AVPixelFormat {
    fn from(pix: &PixelFormat) -> Self {
        (*pix).into()
    }
}

impl From<PixelFormat> for AVPixelFormat {
    fn from(pix: PixelFormat) -> AVPixelFormat {
        match pix {
            PixelFormat::NONE => AVPixelFormat::AV_PIX_FMT_NONE,
            PixelFormat::YUV420P => AVPixelFormat::AV_PIX_FMT_YUV420P,
            PixelFormat::YUYV422 => AVPixelFormat::AV_PIX_FMT_YUYV422,
            PixelFormat::RGB24 => AVPixelFormat::AV_PIX_FMT_RGB24,
            PixelFormat::BGR24 => AVPixelFormat::AV_PIX_FMT_BGR24,
            PixelFormat::YUV422P => AVPixelFormat::AV_PIX_FMT_YUV422P,
            PixelFormat::YUV444P => AVPixelFormat::AV_PIX_FMT_YUV444P,
            PixelFormat::YUV410P => AVPixelFormat::AV_PIX_FMT_YUV410P,
            PixelFormat::YUV411P => AVPixelFormat::AV_PIX_FMT_YUV411P,
            PixelFormat::GRAY8 => AVPixelFormat::AV_PIX_FMT_GRAY8,
            PixelFormat::MONOWHITE => AVPixelFormat::AV_PIX_FMT_MONOWHITE,
            PixelFormat::MONOBLACK => AVPixelFormat::AV_PIX_FMT_MONOBLACK,
            PixelFormat::PAL8 => AVPixelFormat::AV_PIX_FMT_PAL8,
            PixelFormat::YUVJ420P => AVPixelFormat::AV_PIX_FMT_YUVJ420P,
            PixelFormat::YUVJ422P => AVPixelFormat::AV_PIX_FMT_YUVJ422P,
            PixelFormat::YUVJ444P => AVPixelFormat::AV_PIX_FMT_YUVJ444P,
            PixelFormat::UYVY422 => AVPixelFormat::AV_PIX_FMT_UYVY422,
            PixelFormat::UYYVYY411 => AVPixelFormat::AV_PIX_FMT_UYYVYY411,
            PixelFormat::BGR8 => AVPixelFormat::AV_PIX_FMT_BGR8,
            PixelFormat::BGR4 => AVPixelFormat::AV_PIX_FMT_BGR4,
            PixelFormat::BGR4_BYTE => AVPixelFormat::AV_PIX_FMT_BGR4_BYTE,
            PixelFormat::RGB8 => AVPixelFormat::AV_PIX_FMT_RGB8,
            PixelFormat::RGB4 => AVPixelFormat::AV_PIX_FMT_RGB4,
            PixelFormat::RGB4_BYTE => AVPixelFormat::AV_PIX_FMT_RGB4_BYTE,
            PixelFormat::NV12 => AVPixelFormat::AV_PIX_FMT_NV12,
            PixelFormat::NV21 => AVPixelFormat::AV_PIX_FMT_NV21,
            PixelFormat::ARGB => AVPixelFormat::AV_PIX_FMT_ARGB,
            PixelFormat::RGBA => AVPixelFormat::AV_PIX_FMT_RGBA,
            PixelFormat::ABGR => AVPixelFormat::AV_PIX_FMT_ABGR,
            PixelFormat::BGRA => AVPixelFormat::AV_PIX_FMT_BGRA,
            PixelFormat::GRAY16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAY16BE,
            PixelFormat::GRAY16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAY16LE,
            PixelFormat::YUV440P => AVPixelFormat::AV_PIX_FMT_YUV440P,
            PixelFormat::YUVJ440P => AVPixelFormat::AV_PIX_FMT_YUVJ440P,
            PixelFormat::YUVA420P => AVPixelFormat::AV_PIX_FMT_YUVA420P,
            PixelFormat::RGB48(Endian::Big) => AVPixelFormat::AV_PIX_FMT_RGB48BE,
            PixelFormat::RGB48(Endian::Little) => AVPixelFormat::AV_PIX_FMT_RGB48LE,
            PixelFormat::RGB565(Endian::Big) => AVPixelFormat::AV_PIX_FMT_RGB565BE,
            PixelFormat::RGB565(Endian::Little) => AVPixelFormat::AV_PIX_FMT_RGB565LE,
            PixelFormat::RGB555(Endian::Big) => AVPixelFormat::AV_PIX_FMT_RGB555BE,
            PixelFormat::RGB555(Endian::Little) => AVPixelFormat::AV_PIX_FMT_RGB555LE,
            PixelFormat::BGR565(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BGR565BE,
            PixelFormat::BGR565(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BGR565LE,
            PixelFormat::BGR555(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BGR555BE,
            PixelFormat::BGR555(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BGR555LE,
            PixelFormat::VAAPI_MOCO => AVPixelFormat::AV_PIX_FMT_VAAPI_MOCO,
            PixelFormat::VAAPI_IDCT => AVPixelFormat::AV_PIX_FMT_VAAPI_IDCT,
            PixelFormat::VAAPI_VLD => AVPixelFormat::AV_PIX_FMT_VAAPI_VLD,
            PixelFormat::YUV420P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV420P16LE,
            PixelFormat::YUV420P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV420P16BE,
            PixelFormat::YUV422P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV422P16LE,
            PixelFormat::YUV422P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV422P16BE,
            PixelFormat::YUV444P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV444P16LE,
            PixelFormat::YUV444P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV444P16BE,
            PixelFormat::DXVA2_VLD => AVPixelFormat::AV_PIX_FMT_DXVA2_VLD,
            PixelFormat::RGB444(Endian::Little) => AVPixelFormat::AV_PIX_FMT_RGB444LE,
            PixelFormat::RGB444(Endian::Big) => AVPixelFormat::AV_PIX_FMT_RGB444BE,
            PixelFormat::BGR444(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BGR444LE,
            PixelFormat::BGR444(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BGR444BE,
            PixelFormat::YA8 => AVPixelFormat::AV_PIX_FMT_YA8,
            PixelFormat::BGR48(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BGR48BE,
            PixelFormat::BGR48(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BGR48LE,
            PixelFormat::YUV420P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV420P9BE,
            PixelFormat::YUV420P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV420P9LE,
            PixelFormat::YUV420P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV420P10BE,
            PixelFormat::YUV420P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV420P10LE,
            PixelFormat::YUV422P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV422P10BE,
            PixelFormat::YUV422P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV422P10LE,
            PixelFormat::YUV444P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV444P9BE,
            PixelFormat::YUV444P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV444P9LE,
            PixelFormat::YUV444P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV444P10BE,
            PixelFormat::YUV444P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV444P10LE,
            PixelFormat::YUV422P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV422P9BE,
            PixelFormat::YUV422P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV422P9LE,
            PixelFormat::GBRP => AVPixelFormat::AV_PIX_FMT_GBRP,
            PixelFormat::GBRP9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRP9BE,
            PixelFormat::GBRP9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRP9LE,
            PixelFormat::GBRP10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRP10BE,
            PixelFormat::GBRP10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRP10LE,
            PixelFormat::GBRP16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRP16BE,
            PixelFormat::GBRP16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRP16LE,
            PixelFormat::YUVA422P => AVPixelFormat::AV_PIX_FMT_YUVA422P,
            PixelFormat::YUVA444P => AVPixelFormat::AV_PIX_FMT_YUVA444P,
            PixelFormat::YUVA420P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA420P9BE,
            PixelFormat::YUVA420P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA420P9LE,
            PixelFormat::YUVA422P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA422P9BE,
            PixelFormat::YUVA422P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA422P9LE,
            PixelFormat::YUVA444P9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA444P9BE,
            PixelFormat::YUVA444P9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA444P9LE,
            PixelFormat::YUVA420P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA420P10BE,
            PixelFormat::YUVA420P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA420P10LE,
            PixelFormat::YUVA422P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA422P10BE,
            PixelFormat::YUVA422P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA422P10LE,
            PixelFormat::YUVA444P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA444P10BE,
            PixelFormat::YUVA444P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA444P10LE,
            PixelFormat::YUVA420P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA420P16BE,
            PixelFormat::YUVA420P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA420P16LE,
            PixelFormat::YUVA422P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA422P16BE,
            PixelFormat::YUVA422P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA422P16LE,
            PixelFormat::YUVA444P16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA444P16BE,
            PixelFormat::YUVA444P16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA444P16LE,
            PixelFormat::VDPAU => AVPixelFormat::AV_PIX_FMT_VDPAU,
            PixelFormat::XYZ12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_XYZ12LE,
            PixelFormat::XYZ12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_XYZ12BE,
            PixelFormat::NV16 => AVPixelFormat::AV_PIX_FMT_NV16,
            PixelFormat::NV20(Endian::Little) => AVPixelFormat::AV_PIX_FMT_NV20LE,
            PixelFormat::NV20(Endian::Big) => AVPixelFormat::AV_PIX_FMT_NV20BE,
            PixelFormat::RGBA64(Endian::Big) => AVPixelFormat::AV_PIX_FMT_RGBA64BE,
            PixelFormat::RGBA64(Endian::Little) => AVPixelFormat::AV_PIX_FMT_RGBA64LE,
            PixelFormat::BGRA64(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BGRA64BE,
            PixelFormat::BGRA64(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BGRA64LE,
            PixelFormat::YVYU422 => AVPixelFormat::AV_PIX_FMT_YVYU422,
            PixelFormat::YA16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YA16BE,
            PixelFormat::YA16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YA16LE,
            PixelFormat::GBRAP => AVPixelFormat::AV_PIX_FMT_GBRAP,
            PixelFormat::GBRAP16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRAP16BE,
            PixelFormat::GBRAP16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRAP16LE,
            PixelFormat::QSV => AVPixelFormat::AV_PIX_FMT_QSV,
            PixelFormat::MMAL => AVPixelFormat::AV_PIX_FMT_MMAL,
            PixelFormat::D3D11VA_VLD => AVPixelFormat::AV_PIX_FMT_D3D11VA_VLD,
            PixelFormat::CUDA => AVPixelFormat::AV_PIX_FMT_CUDA,
            PixelFormat::XRGB => AVPixelFormat::AV_PIX_FMT_0RGB,
            PixelFormat::RGBX => AVPixelFormat::AV_PIX_FMT_RGB0,
            PixelFormat::XBGR => AVPixelFormat::AV_PIX_FMT_0BGR,
            PixelFormat::BGRX => AVPixelFormat::AV_PIX_FMT_BGR0,
            PixelFormat::YUV420P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV420P12BE,
            PixelFormat::YUV420P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV420P12LE,
            PixelFormat::YUV420P14(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV420P14BE,
            PixelFormat::YUV420P14(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV420P14LE,
            PixelFormat::YUV422P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV422P12BE,
            PixelFormat::YUV422P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV422P12LE,
            PixelFormat::YUV422P14(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV422P14BE,
            PixelFormat::YUV422P14(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV422P14LE,
            PixelFormat::YUV444P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV444P12BE,
            PixelFormat::YUV444P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV444P12LE,
            PixelFormat::YUV444P14(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV444P14BE,
            PixelFormat::YUV444P14(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV444P14LE,
            PixelFormat::GBRP12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRP12BE,
            PixelFormat::GBRP12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRP12LE,
            PixelFormat::GBRP14(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRP14BE,
            PixelFormat::GBRP14(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRP14LE,
            PixelFormat::YUVJ411P => AVPixelFormat::AV_PIX_FMT_YUVJ411P,
            PixelFormat::BAYER_BGGR8 => AVPixelFormat::AV_PIX_FMT_BAYER_BGGR8,
            PixelFormat::BAYER_RGGB8 => AVPixelFormat::AV_PIX_FMT_BAYER_RGGB8,
            PixelFormat::BAYER_GBRG8 => AVPixelFormat::AV_PIX_FMT_BAYER_GBRG8,
            PixelFormat::BAYER_GRBG8 => AVPixelFormat::AV_PIX_FMT_BAYER_GRBG8,
            PixelFormat::BAYER_BGGR16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BAYER_BGGR16LE,
            PixelFormat::BAYER_BGGR16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BAYER_BGGR16BE,
            PixelFormat::BAYER_RGGB16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BAYER_RGGB16LE,
            PixelFormat::BAYER_RGGB16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BAYER_RGGB16BE,
            PixelFormat::BAYER_GBRG16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BAYER_GBRG16LE,
            PixelFormat::BAYER_GBRG16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BAYER_GBRG16BE,
            PixelFormat::BAYER_GRBG16(Endian::Little) => AVPixelFormat::AV_PIX_FMT_BAYER_GRBG16LE,
            PixelFormat::BAYER_GRBG16(Endian::Big) => AVPixelFormat::AV_PIX_FMT_BAYER_GRBG16BE,
            PixelFormat::XVMC => AVPixelFormat::AV_PIX_FMT_XVMC,
            PixelFormat::YUV440P10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV440P10LE,
            PixelFormat::YUV440P10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV440P10BE,
            PixelFormat::YUV440P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUV440P12LE,
            PixelFormat::YUV440P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUV440P12BE,
            PixelFormat::AYUV64(Endian::Little) => AVPixelFormat::AV_PIX_FMT_AYUV64LE,
            PixelFormat::AYUV64(Endian::Big) => AVPixelFormat::AV_PIX_FMT_AYUV64BE,
            PixelFormat::VIDEOTOOLBOX => AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX,
            PixelFormat::P010(Endian::Little) => AVPixelFormat::AV_PIX_FMT_P010LE,
            PixelFormat::P010(Endian::Big) => AVPixelFormat::AV_PIX_FMT_P010BE,
            PixelFormat::GBRAP12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRAP12BE,
            PixelFormat::GBRAP12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRAP12LE,
            PixelFormat::GBRAP10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRAP10BE,
            PixelFormat::GBRAP10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRAP10LE,
            PixelFormat::MEDIACODEC => AVPixelFormat::AV_PIX_FMT_MEDIACODEC,
            PixelFormat::GRAY12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAY12BE,
            PixelFormat::GRAY12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAY12LE,
            PixelFormat::GRAY10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAY10BE,
            PixelFormat::GRAY10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAY10LE,
            PixelFormat::P016(Endian::Little) => AVPixelFormat::AV_PIX_FMT_P016LE,
            PixelFormat::P016(Endian::Big) => AVPixelFormat::AV_PIX_FMT_P016BE,
            PixelFormat::D3D11 => AVPixelFormat::AV_PIX_FMT_D3D11,
            PixelFormat::Gray9(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAY9BE,
            PixelFormat::Gray9(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAY9LE,
            PixelFormat::GBRPF32(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRPF32BE,
            PixelFormat::GBRPF32(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRPF32LE,
            PixelFormat::GBRAPF32(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GBRAPF32BE,
            PixelFormat::GBRAPF32(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GBRAPF32LE,
            PixelFormat::DrmPrime => AVPixelFormat::AV_PIX_FMT_DRM_PRIME,
            PixelFormat::OpenCL => AVPixelFormat::AV_PIX_FMT_OPENCL,
            PixelFormat::GRAY14(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAY14BE,
            PixelFormat::GRAY14(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAY14LE,
            PixelFormat::GRAYF32(Endian::Big) => AVPixelFormat::AV_PIX_FMT_GRAYF32BE,
            PixelFormat::GRAYF32(Endian::Little) => AVPixelFormat::AV_PIX_FMT_GRAYF32LE,
            PixelFormat::YUVA422P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA422P12BE,
            PixelFormat::YUVA422P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA422P12LE,
            PixelFormat::YUVA444P12(Endian::Big) => AVPixelFormat::AV_PIX_FMT_YUVA444P12BE,
            PixelFormat::YUVA444P12(Endian::Little) => AVPixelFormat::AV_PIX_FMT_YUVA444P12LE,
            PixelFormat::NV24 => AVPixelFormat::AV_PIX_FMT_NV24,
            PixelFormat::NV42 => AVPixelFormat::AV_PIX_FMT_NV42,

    #[libavformat(since(58.76))] {
            PixelFormat::Vulkan => AVPixelFormat::AV_PIX_FMT_VULKAN,
            PixelFormat::Y210(Endian::Big) => AVPixelFormat::AV_PIX_FMT_Y210BE,
            PixelFormat::Y210(Endian::Little) => AVPixelFormat::AV_PIX_FMT_Y210LE,
            PixelFormat::X2RGB10(Endian::Little) => AVPixelFormat::AV_PIX_FMT_X2RGB10LE,
            PixelFormat::X2RGB10(Endian::Big) => AVPixelFormat::AV_PIX_FMT_X2RGB10BE,
    }
        }
    }
}

impl From<AVPixelFormat> for PixelFormat {
    fn from(pix: AVPixelFormat) -> Self {
        match pix {
            AVPixelFormat::AV_PIX_FMT_NONE => PixelFormat::NONE,
            AVPixelFormat::AV_PIX_FMT_YUV420P => PixelFormat::YUV420P,
            AVPixelFormat::AV_PIX_FMT_YUYV422 => PixelFormat::YUYV422,
            AVPixelFormat::AV_PIX_FMT_RGB24 => PixelFormat::RGB24,
            AVPixelFormat::AV_PIX_FMT_BGR24 => PixelFormat::BGR24,
            AVPixelFormat::AV_PIX_FMT_YUV422P => PixelFormat::YUV422P,
            AVPixelFormat::AV_PIX_FMT_YUV444P => PixelFormat::YUV444P,
            AVPixelFormat::AV_PIX_FMT_YUV410P => PixelFormat::YUV410P,
            AVPixelFormat::AV_PIX_FMT_YUV411P => PixelFormat::YUV411P,
            AVPixelFormat::AV_PIX_FMT_GRAY8 => PixelFormat::GRAY8,
            AVPixelFormat::AV_PIX_FMT_MONOWHITE => PixelFormat::MONOWHITE,
            AVPixelFormat::AV_PIX_FMT_MONOBLACK => PixelFormat::MONOBLACK,
            AVPixelFormat::AV_PIX_FMT_PAL8 => PixelFormat::PAL8,
            AVPixelFormat::AV_PIX_FMT_YUVJ420P => PixelFormat::YUVJ420P,
            AVPixelFormat::AV_PIX_FMT_YUVJ422P => PixelFormat::YUVJ422P,
            AVPixelFormat::AV_PIX_FMT_YUVJ444P => PixelFormat::YUVJ444P,
            AVPixelFormat::AV_PIX_FMT_UYVY422 => PixelFormat::UYVY422,
            AVPixelFormat::AV_PIX_FMT_UYYVYY411 => PixelFormat::UYYVYY411,
            AVPixelFormat::AV_PIX_FMT_BGR8 => PixelFormat::BGR8,
            AVPixelFormat::AV_PIX_FMT_BGR4 => PixelFormat::BGR4,
            AVPixelFormat::AV_PIX_FMT_BGR4_BYTE => PixelFormat::BGR4_BYTE,
            AVPixelFormat::AV_PIX_FMT_RGB8 => PixelFormat::RGB8,
            AVPixelFormat::AV_PIX_FMT_RGB4 => PixelFormat::RGB4,
            AVPixelFormat::AV_PIX_FMT_RGB4_BYTE => PixelFormat::RGB4_BYTE,
            AVPixelFormat::AV_PIX_FMT_NV12 => PixelFormat::NV12,
            AVPixelFormat::AV_PIX_FMT_NV21 => PixelFormat::NV21,
            AVPixelFormat::AV_PIX_FMT_ARGB => PixelFormat::ARGB,
            AVPixelFormat::AV_PIX_FMT_RGBA => PixelFormat::RGBA,
            AVPixelFormat::AV_PIX_FMT_ABGR => PixelFormat::ABGR,
            AVPixelFormat::AV_PIX_FMT_BGRA => PixelFormat::BGRA,
            AVPixelFormat::AV_PIX_FMT_GRAY16BE => PixelFormat::GRAY16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAY16LE => PixelFormat::GRAY16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV440P => PixelFormat::YUV440P,
            AVPixelFormat::AV_PIX_FMT_YUVJ440P => PixelFormat::YUVJ440P,
            AVPixelFormat::AV_PIX_FMT_YUVA420P => PixelFormat::YUVA420P,
            AVPixelFormat::AV_PIX_FMT_RGB48BE => PixelFormat::RGB48(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_RGB48LE => PixelFormat::RGB48(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_RGB565BE => PixelFormat::RGB565(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_RGB565LE => PixelFormat::RGB565(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_RGB555BE => PixelFormat::RGB555(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_RGB555LE => PixelFormat::RGB555(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BGR565BE => PixelFormat::BGR565(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BGR565LE => PixelFormat::BGR565(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BGR555BE => PixelFormat::BGR555(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BGR555LE => PixelFormat::BGR555(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_VAAPI_MOCO => PixelFormat::VAAPI_MOCO,
            AVPixelFormat::AV_PIX_FMT_VAAPI_IDCT => PixelFormat::VAAPI_IDCT,
            AVPixelFormat::AV_PIX_FMT_VAAPI_VLD => PixelFormat::VAAPI_VLD,
            AVPixelFormat::AV_PIX_FMT_YUV420P16LE => PixelFormat::YUV420P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV420P16BE => PixelFormat::YUV420P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV422P16LE => PixelFormat::YUV422P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV422P16BE => PixelFormat::YUV422P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV444P16LE => PixelFormat::YUV444P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV444P16BE => PixelFormat::YUV444P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_DXVA2_VLD => PixelFormat::DXVA2_VLD,
            AVPixelFormat::AV_PIX_FMT_RGB444LE => PixelFormat::RGB444(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_RGB444BE => PixelFormat::RGB444(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BGR444LE => PixelFormat::BGR444(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BGR444BE => PixelFormat::BGR444(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YA8 => PixelFormat::YA8,
            AVPixelFormat::AV_PIX_FMT_BGR48BE => PixelFormat::BGR48(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BGR48LE => PixelFormat::BGR48(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV420P9BE => PixelFormat::YUV420P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV420P9LE => PixelFormat::YUV420P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV420P10BE => PixelFormat::YUV420P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV420P10LE => PixelFormat::YUV420P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV422P10BE => PixelFormat::YUV422P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV422P10LE => PixelFormat::YUV422P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV444P9BE => PixelFormat::YUV444P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV444P9LE => PixelFormat::YUV444P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV444P10BE => PixelFormat::YUV444P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV444P10LE => PixelFormat::YUV444P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV422P9BE => PixelFormat::YUV422P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV422P9LE => PixelFormat::YUV422P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRP => PixelFormat::GBRP,
            AVPixelFormat::AV_PIX_FMT_GBRP9BE => PixelFormat::GBRP9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRP9LE => PixelFormat::GBRP9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRP10BE => PixelFormat::GBRP10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRP10LE => PixelFormat::GBRP10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRP16BE => PixelFormat::GBRP16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRP16LE => PixelFormat::GBRP16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA422P => PixelFormat::YUVA422P,
            AVPixelFormat::AV_PIX_FMT_YUVA444P => PixelFormat::YUVA444P,
            AVPixelFormat::AV_PIX_FMT_YUVA420P9BE => PixelFormat::YUVA420P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA420P9LE => PixelFormat::YUVA420P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA422P9BE => PixelFormat::YUVA422P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA422P9LE => PixelFormat::YUVA422P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA444P9BE => PixelFormat::YUVA444P9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA444P9LE => PixelFormat::YUVA444P9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA420P10BE => PixelFormat::YUVA420P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA420P10LE => PixelFormat::YUVA420P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA422P10BE => PixelFormat::YUVA422P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA422P10LE => PixelFormat::YUVA422P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA444P10BE => PixelFormat::YUVA444P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA444P10LE => PixelFormat::YUVA444P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA420P16BE => PixelFormat::YUVA420P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA420P16LE => PixelFormat::YUVA420P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA422P16BE => PixelFormat::YUVA422P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA422P16LE => PixelFormat::YUVA422P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA444P16BE => PixelFormat::YUVA444P16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA444P16LE => PixelFormat::YUVA444P16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_VDPAU => PixelFormat::VDPAU,
            AVPixelFormat::AV_PIX_FMT_XYZ12LE => PixelFormat::XYZ12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_XYZ12BE => PixelFormat::XYZ12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_NV16 => PixelFormat::NV16,
            AVPixelFormat::AV_PIX_FMT_NV20LE => PixelFormat::NV20(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_NV20BE => PixelFormat::NV20(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_RGBA64BE => PixelFormat::RGBA64(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_RGBA64LE => PixelFormat::RGBA64(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BGRA64BE => PixelFormat::BGRA64(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BGRA64LE => PixelFormat::BGRA64(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YVYU422 => PixelFormat::YVYU422,
            AVPixelFormat::AV_PIX_FMT_YA16BE => PixelFormat::YA16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YA16LE => PixelFormat::YA16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRAP => PixelFormat::GBRAP,
            AVPixelFormat::AV_PIX_FMT_GBRAP16BE => PixelFormat::GBRAP16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRAP16LE => PixelFormat::GBRAP16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_QSV => PixelFormat::QSV,
            AVPixelFormat::AV_PIX_FMT_MMAL => PixelFormat::MMAL,
            AVPixelFormat::AV_PIX_FMT_D3D11VA_VLD => PixelFormat::D3D11VA_VLD,
            AVPixelFormat::AV_PIX_FMT_CUDA => PixelFormat::CUDA,
            AVPixelFormat::AV_PIX_FMT_0RGB => PixelFormat::XRGB,
            AVPixelFormat::AV_PIX_FMT_RGB0 => PixelFormat::RGBX,
            AVPixelFormat::AV_PIX_FMT_0BGR => PixelFormat::XBGR,
            AVPixelFormat::AV_PIX_FMT_BGR0 => PixelFormat::BGRX,
            AVPixelFormat::AV_PIX_FMT_YUV420P12BE => PixelFormat::YUV420P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV420P12LE => PixelFormat::YUV420P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV420P14BE => PixelFormat::YUV420P14(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV420P14LE => PixelFormat::YUV420P14(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV422P12BE => PixelFormat::YUV422P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV422P12LE => PixelFormat::YUV422P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV422P14BE => PixelFormat::YUV422P14(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV422P14LE => PixelFormat::YUV422P14(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV444P12BE => PixelFormat::YUV444P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV444P12LE => PixelFormat::YUV444P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV444P14BE => PixelFormat::YUV444P14(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV444P14LE => PixelFormat::YUV444P14(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRP12BE => PixelFormat::GBRP12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRP12LE => PixelFormat::GBRP12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRP14BE => PixelFormat::GBRP14(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRP14LE => PixelFormat::GBRP14(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVJ411P => PixelFormat::YUVJ411P,
            AVPixelFormat::AV_PIX_FMT_BAYER_BGGR8 => PixelFormat::BAYER_BGGR8,
            AVPixelFormat::AV_PIX_FMT_BAYER_RGGB8 => PixelFormat::BAYER_RGGB8,
            AVPixelFormat::AV_PIX_FMT_BAYER_GBRG8 => PixelFormat::BAYER_GBRG8,
            AVPixelFormat::AV_PIX_FMT_BAYER_GRBG8 => PixelFormat::BAYER_GRBG8,
            AVPixelFormat::AV_PIX_FMT_BAYER_BGGR16LE => PixelFormat::BAYER_BGGR16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BAYER_BGGR16BE => PixelFormat::BAYER_BGGR16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BAYER_RGGB16LE => PixelFormat::BAYER_RGGB16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BAYER_RGGB16BE => PixelFormat::BAYER_RGGB16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BAYER_GBRG16LE => PixelFormat::BAYER_GBRG16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BAYER_GBRG16BE => PixelFormat::BAYER_GBRG16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_BAYER_GRBG16LE => PixelFormat::BAYER_GRBG16(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_BAYER_GRBG16BE => PixelFormat::BAYER_GRBG16(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_XVMC => PixelFormat::XVMC,
            AVPixelFormat::AV_PIX_FMT_YUV440P10LE => PixelFormat::YUV440P10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV440P10BE => PixelFormat::YUV440P10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUV440P12LE => PixelFormat::YUV440P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUV440P12BE => PixelFormat::YUV440P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_AYUV64LE => PixelFormat::AYUV64(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_AYUV64BE => PixelFormat::AYUV64(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX => PixelFormat::VIDEOTOOLBOX,
            AVPixelFormat::AV_PIX_FMT_P010LE => PixelFormat::P010(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_P010BE => PixelFormat::P010(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRAP12BE => PixelFormat::GBRAP12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRAP12LE => PixelFormat::GBRAP12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRAP10BE => PixelFormat::GBRAP10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRAP10LE => PixelFormat::GBRAP10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_MEDIACODEC => PixelFormat::MEDIACODEC,
            AVPixelFormat::AV_PIX_FMT_GRAY12BE => PixelFormat::GRAY12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAY12LE => PixelFormat::GRAY12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GRAY10BE => PixelFormat::GRAY10(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAY10LE => PixelFormat::GRAY10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_P016LE => PixelFormat::P016(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_P016BE => PixelFormat::P016(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_D3D11 => PixelFormat::D3D11,
            AVPixelFormat::AV_PIX_FMT_GRAY9BE => PixelFormat::Gray9(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAY9LE => PixelFormat::Gray9(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRPF32BE => PixelFormat::GBRPF32(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRPF32LE => PixelFormat::GBRPF32(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GBRAPF32BE => PixelFormat::GBRAPF32(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GBRAPF32LE => PixelFormat::GBRAPF32(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_DRM_PRIME => PixelFormat::DrmPrime,
            AVPixelFormat::AV_PIX_FMT_OPENCL => PixelFormat::OpenCL,
            AVPixelFormat::AV_PIX_FMT_GRAY14BE => PixelFormat::GRAY14(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAY14LE => PixelFormat::GRAY14(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_GRAYF32BE => PixelFormat::GRAYF32(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_GRAYF32LE => PixelFormat::GRAYF32(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA422P12BE => PixelFormat::YUVA422P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA422P12LE => PixelFormat::YUVA422P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_YUVA444P12BE => PixelFormat::YUVA444P12(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_YUVA444P12LE => PixelFormat::YUVA444P12(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_NV24 => PixelFormat::NV24,
            AVPixelFormat::AV_PIX_FMT_NV42 => PixelFormat::NV42,

    #[libavformat(since(58.76))] {
            AVPixelFormat::AV_PIX_FMT_VULKAN => PixelFormat::Vulkan,
            AVPixelFormat::AV_PIX_FMT_Y210BE => PixelFormat::Y210(Endian::Big),
            AVPixelFormat::AV_PIX_FMT_Y210LE => PixelFormat::Y210(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_X2RGB10LE => PixelFormat::X2RGB10(Endian::Little),
            AVPixelFormat::AV_PIX_FMT_X2RGB10BE => PixelFormat::X2RGB10(Endian::Big),
    }
            _ => panic!("Unracognized pixel format"),
        }
    }
}
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_name())
    }
}
