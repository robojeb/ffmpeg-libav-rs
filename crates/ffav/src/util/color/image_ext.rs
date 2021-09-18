use crate::{
    raw::frame::Frame,
    util::{color::PixelFormat, marker::Video},
};
use image::RgbImage;

pub trait ImageFormat: Sized {
    const FORMAT: PixelFormat;

    fn from_frame(frame: &Frame<Video>) -> Self;
}

impl ImageFormat for RgbImage {
    const FORMAT: PixelFormat = PixelFormat::RGB24;

    fn from_frame(frame: &Frame<Video>) -> Self {
        let plane = frame.plane(0);

        let pixels: Vec<u8> = plane
            .chunks_exact(plane.linesize)
            .flat_map(|line| line.chunks_exact(3).take(frame.width() as usize))
            .flatten()
            .copied()
            .collect();

        RgbImage::from_vec(frame.width(), frame.height(), pixels)
            .expect("Frame data was not of the right dimensions")
    }
}
