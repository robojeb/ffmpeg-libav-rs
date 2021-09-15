use ffav_sys::AVPixelFormat;

/// The format which the pixel information is stored in a decoded Frame of a
/// video stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelFormat {
    /// No pixel format is specified
    None,
    /// planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P,
    /// planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    NV12,
    /// as above, but U and V bytes are swapped
    NV21,
    /// planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P,
    /// planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P,
    /// planar YUV 4:1:0,  9bpp, (1 Cr & Cb sample per 4x4 Y samples)
    YUV410P,
    /// planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples)
    YUV411P,

    /// 8 bits with RGB32 palette
    PAL8,
    /// planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of YUV420P and setting color_range
    YUVJ420P,
    /// planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of YUV422P and setting color_range
    YUVJ422P,
    /// planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of YUV444P and setting color_range
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
    BGR4Byte,
    /// packed RGB 3:3:2,  8bpp, (msb)2R 3G 3B(lsb)
    RGB8,
    /// packed RGB 1:2:1 bitstream,  4bpp, (msb)1R 2G 1B(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    RGB4,
    /// packed RGB 1:2:1,  8bpp, (msb)1R 2G 1B(lsb)
    RGB4Byte,

    /// packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr
    YUYV422,
    /// packed RGB 8:8:8, 24bpp, RGBRGB...
    RGB24,
    /// packed RGB 8:8:8, 24bpp, BGRBGR...
    BGR24,

    ///        Y        ,  8bpp
    GRAY8,
    ///        Y        ,  1bpp, 0 is white, 1 is black, in each byte pixels are ordered from the msb to the lsb
    MONOWHITE,
    ///        Y        ,  1bpp, 0 is black, 1 is white, in each byte pixels are ordered from the msb to the lsb
    MONOBLACK,

    /// packed ARGB 8:8:8:8, 32bpp, ARGBARGB...
    ARGB,
    /// packed RGBA 8:8:8:8, 32bpp, RGBARGBA...
    RGBA,
    /// packed ABGR 8:8:8:8, 32bpp, ABGRABGR...
    ABGR,
    /// packed BGRA 8:8:8:8, 32bpp, BGRABGRA...
    BGRA,

    ///        Y        , 16bpp, big-endian
    GRAY16BE,
    ///        Y        , 16bpp, little-endian
    GRAY16LE,
    /// planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples)
    YUV440P,
    /// planar YUV 4:4:0 full scale (JPEG), deprecated in favor of YUV440P and setting color_range
    YUVJ440P,
    /// planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P,
    /// packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as big-endian
    RGB48BE,
    /// packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as little-endian
    RGB48LE,

    /// packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), big-endian
    RGB565BE,
    /// packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), little-endian
    RGB565LE,
    /// packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), big-endian   , X=unused/undefined
    RGB555BE,
    /// packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), little-endian, X=unused/undefined
    RGB555LE,

    /// packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), big-endian
    BGR565BE,
    /// packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), little-endian
    BGR565LE,
    /// packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), big-endian   , X=unused/undefined
    BGR555BE,
    /// packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), little-endian, X=unused/undefined
    BGR555LE,
    // TODO: VAAPI Formats
    /*
    *  Hardware acceleration through VA-API, data[3] contains a
    *  VASurfaceID.
    */

    // TODO: The annoying formats
    /*
    * The following 12 formats have the disadvantage of needing 1 format for each bit depth.
    * Notice that each 9/10 bits sample is stored in 16 bits with extra padding.
    * If you want to support multiple bit depths, then using YUV420P16* with the bpp stored separately is better.
    */

    //TODO: Hardware acceleration
    /*
     *  HW acceleration through QSV, data[3] contains a pointer to the
     *  mfxFrameSurface1 structure.
     */

     /*
     * HW acceleration though MMAL, data[3] contains a pointer to the
     * MMAL_BUFFER_HEADER_T structure.
     */

     /*
     * HW acceleration through CUDA. data[i] contain CUdeviceptr pointers
     * exactly as for system memory frames.
     */

     /*
     * Hardware surfaces for Direct3D11.
     *
     * This is preferred over the legacy AV_PIX_FMT_D3D11VA_VLD. The new D3D11
     * hwaccel API and filtering support AV_PIX_FMT_D3D11 only.
     *
     * data[0] contains a ID3D11Texture2D pointer, and data[1] contains the
     * texture array index of the frame as intptr_t if the ID3D11Texture2D is
     * an array texture (or always 0 if it's a normal texture).
     */

     /*
     * DRM-managed buffers exposed through PRIME buffer sharing.
     *
     * data[0] points to an AVDRMFrameDescriptor.
     */

    /*
     * Hardware surfaces for OpenCL.
     *
     * data[i] contain 2D image objects (typed in C as cl_mem, used
     * in OpenCL as image2d_t) for each plane of the surface.
     */

    /*
     * Vulkan hardware images.
     *
     * data[0] points to an AVVkFrame
     */
}

impl PixelFormat {
    /// Convert to an `AVPixelFormat` usable by libav
    pub fn as_av_pixel_format(self) -> AVPixelFormat {
        match self {
            PixelFormat::None => AVPixelFormat::AV_PIX_FMT_NONE,
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
            PixelFormat::BGR4Byte => AVPixelFormat::AV_PIX_FMT_BGR4_BYTE,
            PixelFormat::RGB8 => AVPixelFormat::AV_PIX_FMT_RGB8,
            PixelFormat::RGB4 => AVPixelFormat::AV_PIX_FMT_RGB4,
            PixelFormat::RGB4Byte => AVPixelFormat::AV_PIX_FMT_RGB4_BYTE,
            PixelFormat::NV12 => AVPixelFormat::AV_PIX_FMT_NV12,
            PixelFormat::NV21 => AVPixelFormat::AV_PIX_FMT_NV21,

            PixelFormat::ARGB => AVPixelFormat::AV_PIX_FMT_ARGB,
            PixelFormat::RGBA => AVPixelFormat::AV_PIX_FMT_RGBA,
            PixelFormat::ABGR => AVPixelFormat::AV_PIX_FMT_ABGR,
            PixelFormat::BGRA => AVPixelFormat::AV_PIX_FMT_BGRA,

            PixelFormat::GRAY16BE => AVPixelFormat::AV_PIX_FMT_GRAY16BE,
            PixelFormat::GRAY16LE => AVPixelFormat::AV_PIX_FMT_GRAY16LE,
            PixelFormat::YUV440P => AVPixelFormat::AV_PIX_FMT_YUV440P,
            PixelFormat::YUVJ440P => AVPixelFormat::AV_PIX_FMT_YUVJ440P,
            PixelFormat::YUVA420P => AVPixelFormat::AV_PIX_FMT_YUVA420P,
            PixelFormat::RGB48BE => AVPixelFormat::AV_PIX_FMT_RGB48BE,
            PixelFormat::RGB48LE => AVPixelFormat::AV_PIX_FMT_RGB48LE,

            PixelFormat::RGB565BE => AVPixelFormat::AV_PIX_FMT_RGB565BE,
            PixelFormat::RGB565LE => AVPixelFormat::AV_PIX_FMT_RGB565LE,
            PixelFormat::RGB555BE => AVPixelFormat::AV_PIX_FMT_RGB555BE,
            PixelFormat::RGB555LE => AVPixelFormat::AV_PIX_FMT_RGB555LE,

            PixelFormat::BGR565BE => AVPixelFormat::AV_PIX_FMT_BGR565BE,
            PixelFormat::BGR565LE => AVPixelFormat::AV_PIX_FMT_BGR565LE,
            PixelFormat::BGR555BE => AVPixelFormat::AV_PIX_FMT_BGR555BE,
            PixelFormat::BGR555LE => AVPixelFormat::AV_PIX_FMT_BGR555LE,
        }
    }
}

impl From<AVPixelFormat> for PixelFormat {
    fn from(pix: AVPixelFormat) -> Self {
        match pix {
            AVPixelFormat::AV_PIX_FMT_NONE => PixelFormat::None,
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
            AVPixelFormat::AV_PIX_FMT_BGR4_BYTE => PixelFormat::BGR4Byte,
            AVPixelFormat::AV_PIX_FMT_RGB8 => PixelFormat::RGB8,
            AVPixelFormat::AV_PIX_FMT_RGB4 => PixelFormat::RGB4,
            AVPixelFormat::AV_PIX_FMT_RGB4_BYTE => PixelFormat::RGB4Byte,
            AVPixelFormat::AV_PIX_FMT_NV12 => PixelFormat::NV12,
            AVPixelFormat::AV_PIX_FMT_NV21 => PixelFormat::NV21,

            AVPixelFormat::AV_PIX_FMT_ARGB => PixelFormat::ARGB,
            AVPixelFormat::AV_PIX_FMT_RGBA => PixelFormat::RGBA,
            AVPixelFormat::AV_PIX_FMT_ABGR => PixelFormat::ABGR,
            AVPixelFormat::AV_PIX_FMT_BGRA => PixelFormat::BGRA,

            AVPixelFormat::AV_PIX_FMT_GRAY16BE => PixelFormat::GRAY16BE,
            AVPixelFormat::AV_PIX_FMT_GRAY16LE => PixelFormat::GRAY16LE,
            AVPixelFormat::AV_PIX_FMT_YUV440P => PixelFormat::YUV440P,
            AVPixelFormat::AV_PIX_FMT_YUVJ440P => PixelFormat::YUVJ440P,
            AVPixelFormat::AV_PIX_FMT_YUVA420P => PixelFormat::YUVA420P,
            AVPixelFormat::AV_PIX_FMT_RGB48BE => PixelFormat::RGB48BE,
            AVPixelFormat::AV_PIX_FMT_RGB48LE => PixelFormat::RGB48LE,

            AVPixelFormat::AV_PIX_FMT_RGB565BE => PixelFormat::RGB565BE,
            AVPixelFormat::AV_PIX_FMT_RGB565LE => PixelFormat::RGB565LE,
            AVPixelFormat::AV_PIX_FMT_RGB555BE => PixelFormat::RGB555BE,
            AVPixelFormat::AV_PIX_FMT_RGB555LE => PixelFormat::RGB555LE,

            AVPixelFormat::AV_PIX_FMT_BGR565BE => PixelFormat::BGR565BE,
            AVPixelFormat::AV_PIX_FMT_BGR565LE => PixelFormat::BGR565LE,
            AVPixelFormat::AV_PIX_FMT_BGR555BE => PixelFormat::BGR555BE,
            AVPixelFormat::AV_PIX_FMT_BGR555LE => PixelFormat::BGR555LE,
            _ => panic!("Unimplemented or Unknown pixel format"),
        }
    }
}
