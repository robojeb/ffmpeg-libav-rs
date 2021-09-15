//! Structures related to handling codec objects

use std::marker::PhantomData;

use ffav_sys::{
    avcodec_alloc_context3, avcodec_find_decoder, avcodec_free_context, avcodec_open2,
    avcodec_parameters_to_context, avcodec_receive_frame, avcodec_send_packet, err::av_err,
    AVCodecContext,
};

use crate::{
    config::{stream::DecodedStreamConfig, StreamConfig},
    error::{Error, Result},
    util::{marker::Decode, MediaType},
};

use super::{frame::Frame, packet::Packet, stream::Stream};

/// An instantiated raw codec instance
///
/// The parameter `EnDec` can either be `crate::tags::Encode` or `crate::tags::Decode`
/// The parameter `AV` indicates the type of frame produced by the decoding
/// process.
pub struct Codec<EnDec, AV> {
    codec: *mut AVCodecContext,
    stream_config: DecodedStreamConfig<AV>,
    _codec: PhantomData<(EnDec, AV)>,
}

impl<AV: MediaType> Codec<Decode, AV> {
    /// Open a codec for decoding based on the configuration in the provided
    /// stream.
    pub fn open_decode(stream: &Stream<'_, AV>) -> Result<Codec<Decode, AV>> {
        unsafe {
            let stream = stream.as_raw();

            let codec = {
                let decoder = avcodec_find_decoder((*(*stream).codecpar).codec_id);

                if decoder.is_null() {
                    return Err(Error::ResourceNotFound("decoder"));
                }

                let decoder_ctx = avcodec_alloc_context3(decoder);

                if decoder_ctx.is_null() {
                    return Err(Error::AllocationFailed("initializing decoder context"));
                }

                let err = avcodec_parameters_to_context(decoder_ctx, (*stream).codecpar);

                if err < 0 {
                    return Err(Error::from_av_err("copying codec parameters", err));
                }

                let err = avcodec_open2(decoder_ctx, decoder, std::ptr::null_mut());

                if err < 0 {
                    return Err(Error::from_av_err("opening decoder", err));
                }

                decoder_ctx
            };

            let cfg: StreamConfig<AV> = StreamConfig::<AV>::from_av_stream(stream)
                .try_as_type()
                .unwrap();

            let stream_config = DecodedStreamConfig::new(cfg, codec);

            Ok(Codec {
                codec,
                stream_config,
                _codec: PhantomData,
            })
        }
    }

    /// Submit a packet for decoding
    ///
    /// The supplied packet will be unreferenced by this operation and will
    /// be available for later use.
    pub fn submit_packet(&mut self, packet: &mut Packet) -> Result<()> {
        if packet.stream_index() != self.stream_config.stream_index() {
            return Err(Error::PacketFromInvalidStream);
        }

        unsafe {
            let pkt = packet.as_raw();

            let err = avcodec_send_packet(self.codec, pkt);

            if err < 0 {
                return Err(Error::from_av_err("submitting packet to decoder", err));
            }
        }

        // `avcodec_send_packet()` makes an internal copy of the packet so it
        // is safe to unreference it here.
        packet.unref();

        Ok(())
    }

    /// Get the next decoded frame and put it into the provided frame object
    ///
    /// Any existing data contained in the frame will be unreferenced by this
    /// function.
    pub fn get_next_frame_into(&mut self, frame: &mut Frame<AV>) -> Result<()> {
        unsafe {
            let err = avcodec_receive_frame(self.codec, frame.as_raw());

            if err == av_err(ffav_sys::err::EAGAIN) {
                return Err(Error::SubmitMoreInput);
            } else if err < 0 {
                return Err(Error::from_av_err("receiving frame from codec", err));
            }
        }

        Ok(())
    }

    /// Get the next decoded frame and return a new frame object
    ///
    /// Panics: If a new frame cannot be allocated
    pub fn get_next_frame(&mut self) -> Result<Frame<AV>> {
        let mut frame = Frame::new();

        self.get_next_frame_into(&mut frame)?;

        Ok(frame)
    }

    pub fn out_stream_config(&self) -> &DecodedStreamConfig<AV> {
        &self.stream_config
    }
}

impl<EnDec, AV> Codec<EnDec, AV> {
    /// Get the raw AVCdecContext pointer
    ///
    /// # Safety
    /// The pointer will never be NULL.
    ///
    /// The pointer should not be held longer than the lifetime of the containing
    /// `Codec` struct. While holding the pointer it should be considered that
    /// the `Codec` struct is mutibly borrowoed.
    pub unsafe fn as_raw(&self) -> *mut AVCodecContext {
        self.codec
    }
}

impl<EnDec, AV> std::ops::Drop for Codec<EnDec, AV> {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self.codec);
            // `avcodec_free_context` should set this to NULL but just in case
            self.codec = std::ptr::null_mut();
        }
    }
}

// SAFTEY: The Codec type is the sole owner of the contained pointer and has
// no interior mutability.
unsafe impl<EnDec, AV> std::marker::Send for Codec<EnDec, AV> {}
unsafe impl<EnDec, AV> std::marker::Sync for Codec<EnDec, AV> {}
