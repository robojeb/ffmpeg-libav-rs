use crate::{
    decode::simple::SimpleDecoder,
    error::Result,
    raw::{format::Format, stream::Stream},
    util::{
        color::ImageFormat,
        marker::{Input, Video},
    },
    Frame,
};
use std::{marker::PhantomData, path::Path};

/// Decode a video stream from the provided Format into a series of images
///
/// The Images will be returned as an `image::ImageBuffer` of the selected type
pub struct ImageDecoder<I: ImageFormat> {
    simple: SimpleDecoder<Video>,
    frame: Frame<Video>,
    _output: PhantomData<I>,
}

impl<I: ImageFormat> ImageDecoder<I> {
    /// Open an input format and prepare it to decode frames from the default stream into an image
    pub fn open<P: AsRef<Path>>(file: P) -> Result<ImageDecoder<I>> {
        Self::inner_open(SimpleDecoder::open(file)?)
    }

    /// Open an input format and decode frames from the selected stream into an image
    pub fn open_with_stream<P: AsRef<Path>, S>(
        file: P,
        stream_selector: S,
    ) -> Result<ImageDecoder<I>>
    where
        S: FnOnce(&Format<Input>) -> Result<Stream<Video>>,
    {
        Self::inner_open(SimpleDecoder::open_with_stream(file, stream_selector)?)
    }

    /// Set up an existing input format to decoding the default video stream into images
    pub fn from_format(fmt: Format<Input>) -> Result<ImageDecoder<I>> {
        Self::inner_open(SimpleDecoder::from_format(fmt)?)
    }

    /// Set up an existing input format to decoding the selected video stream into images
    pub fn from_format_with_stream<S>(
        fmt: Format<Input>,
        stream_selector: S,
    ) -> Result<ImageDecoder<I>>
    where
        S: FnOnce(&Format<Input>) -> Result<Stream<Video>>,
    {
        Self::inner_open(SimpleDecoder::from_format_with_stream(
            fmt,
            stream_selector,
        )?)
    }

    fn inner_open(simple: SimpleDecoder<AV>) -> Result<ImageDecoder<I>> {
        Ok(ImageDecoder {
            simple,
            frame: Frame::new(),
            _output: PhantomData,
        })
    }

    /// Get the next frame of the output as an image
    pub fn get_next_image(&mut self) -> Result<I> {
        self.simple.get_next_frame_into(&mut self.frame)?;

        // TODO: Filter graph

        Ok(I::from_frame(&self.frame))
    }

    /// Destroy the decoding context and return the contained input format
    ///
    /// The input format will be seeked back to the beginning of the file
    pub fn finish(self) -> Format<Input> {
        self.simple.finish()
    }
}
