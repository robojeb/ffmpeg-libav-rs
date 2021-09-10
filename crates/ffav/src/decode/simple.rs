use std::path::Path;

use crate::{
    error::{Error, Result},
    raw::{codec::Codec, format::Format, frame::Frame, packet::Packet, stream::Stream},
    tags::{Decode, Input},
    util::MediaType,
};

/// Decode a single stream from a selected file with no processing
pub struct SimpleDecoder<AV> {
    fmt: Format<Input>,
    codec: Codec<Decode, AV>,
    packet: Packet,
}

impl<AV: MediaType> SimpleDecoder<AV> {
    /// Open a file for decoding selecting the best stream of type `AV`
    pub fn open<P: AsRef<Path>>(file: P) -> Result<SimpleDecoder<AV>> {
        Self::open_with_stream(file, default_stream_selector)
    }

    /// Open a file for decoding.
    ///
    /// The provided function `stream_selector` will
    /// be provided the opened Format and should return the stream that should
    /// be decoded.
    pub fn open_with_stream<P: AsRef<Path>, S>(
        file: P,
        stream_selector: S,
    ) -> Result<SimpleDecoder<AV>>
    where
        S: FnOnce(&Format<Input>) -> Result<Stream<AV>>,
    {
        let format = Format::open_input(file)?;
        let stream = stream_selector(&format)?;
        let codec = Codec::open_decode(&stream)?;

        let packet = Packet::new();

        Ok(SimpleDecoder {
            fmt: format,
            codec,
            packet,
        })
    }

    /// Get the next decoded frame from the stream
    pub fn get_next_frame(&mut self) -> Result<Frame<AV>> {
        loop {
            match self.codec.get_next_frame() {
                Ok(f) => return Ok(f),
                Err(Error::SubmitMoreInput) => {}
                Err(x) => return Err(x),
            }

            self.fmt.get_next_packet_into(&mut self.packet)?;
            self.codec.submit_packet(&mut self.packet)?;
        }
    }

    /// Get the next decoded frame from the stream and place it into the provided frame
    ///
    /// Existing data in the frame will be unreferenced
    pub fn get_next_frame_into(&mut self, frame: &mut Frame<AV>) -> Result<()> {
        loop {
            match self.codec.get_next_frame_into(frame) {
                Ok(_) => return Ok(()),
                Err(Error::SubmitMoreInput) => {}
                Err(x) => return Err(x),
            }

            self.fmt.get_next_packet_into(&mut self.packet)?;
            self.codec.submit_packet(&mut self.packet)?;
        }
    }
}

fn default_stream_selector<T: MediaType>(fmt: &Format<Input>) -> Result<Stream<T>> {
    fmt.get_best_stream().ok_or(Error::StreamNotFound)
}
