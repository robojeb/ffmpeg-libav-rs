use crate::{
    config::FormatConfig,
    error::{Error, Result},
    raw::stream::Stream,
    tags::{Input, Output, Unknown},
    util::MediaType,
};
use ffav_sys::{
    av_find_best_stream, av_read_frame, avformat_close_input, avformat_find_stream_info,
    avformat_free_context, avformat_open_input,
    err::{AVERROR_DECODER_NOT_FOUND, AVERROR_STREAM_NOT_FOUND},
    AVFormatContext,
};
use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use super::packet::Packet;

/// Represents a file or data stream which contains A/V information.
///
/// The marker type I/O can either be `crate::tags::Input` or `crate::tags::Output`
/// indicating if this format is configured as source or sink of data.
#[derive(Debug)]
pub struct Format<IO> {
    ctx: *mut AVFormatContext,
    config: FormatConfig<IO>,
}

impl<T> Format<T> {
    /// Get the raw `ffav_sys::AVFormatContext` pointer
    ///
    /// This is exposed as an "escape hatch" if the abstraction layer doesn't
    /// allow some useful functionality.
    ///
    /// # Safety
    /// The pointer should not be held longer than the lifetime of the `Format` structure.
    /// It should be assumed that the `Format` object is mutibly borrowoed while the
    /// pointer is being used.
    pub unsafe fn as_raw(&self) -> *mut AVFormatContext {
        self.ctx
    }

    /// Get information about the configuration of the input format
    pub fn get_configuration(&self) -> &FormatConfig<T> {
        &self.config
    }
}

impl Format<Input> {
    /// Open a file as an input context
    ///
    /// The format type will be determined by the file name
    pub fn open_input<P: AsRef<Path>>(file: P) -> Result<Format<Input>> {
        // FIXME: This is only valid on Unix systems
        let cfile_path = CString::new(file.as_ref().as_os_str().as_bytes())?;

        unsafe {
            let mut ctx = std::ptr::null_mut();

            let ret = avformat_open_input(
                &mut ctx,
                cfile_path.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            if ret < 0 {
                return Err(Error::from_av_err("opening input format", ret));
            }

            let ret = avformat_find_stream_info(ctx, std::ptr::null_mut());

            if ret < 0 {
                return Err(Error::from_av_err("getting stream info", ret));
            }

            let config = FormatConfig::from_input_fmt_ctx(ctx);

            Ok(Format { ctx, config })
        }
    }

    /// Tries to get the "best" stream for the requested type
    ///
    /// Internally this uses `av_find_best_stream()` which uses a series of hueristics
    /// to determine what is the best available stream. Documentation for that
    /// function is available [here](https://ffmpeg.org/doxygen/2.4/group__lavf__decoding.html#gaa6fa468c922ff5c60a6021dcac09aff9)
    pub fn get_best_stream<T: MediaType>(&self) -> Option<Stream<'_, T>> {
        unsafe {
            let idx = av_find_best_stream(self.ctx, T::MEDIA_TYPE, -1, -1, std::ptr::null_mut(), 0);

            if idx < 0 {
                dbg!(AVERROR_STREAM_NOT_FOUND);
                if idx != AVERROR_STREAM_NOT_FOUND && idx != AVERROR_DECODER_NOT_FOUND {
                    panic!("Unexpected error returned from `av_find_best_stream()`");
                }

                None
            } else {
                self.streams()[idx as usize].try_as_type()
            }
        }
    }

    /// Get all the streams contained in the Format
    ///
    /// The returned streams are of an unknown type and will have to be converted appropriately
    pub fn streams(&self) -> &'_ [Stream<'_, Unknown>] {
        unsafe {
            // This should never happen so only assert in debug mode
            debug_assert!(!self.ctx.is_null(), "An invalid NULL format was used");
            // This could potentially happen in some pathological case, so panic
            // in release if this is the case
            assert!(
                !(*self.ctx).streams.is_null(),
                "Streams member of the format was NULL"
            );

            let streams =
                std::slice::from_raw_parts((*self.ctx).streams, (*self.ctx).nb_streams as usize);

            assert!(
                streams.iter().all(|ptr| { !ptr.is_null() }),
                "One of the streams in the specified size was NULL, corrupted format?"
            );

            // The UCG says that this transmute *should be* sound as long as the `Stream` struct
            // only contains the pointer and the 1-ZST:
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html#structs-with-1-zst-fields
            // So it should be the same size and alignment as the underlying pointer.
            static_assertions::assert_eq_size!(*mut ffav_sys::AVStream, Stream<Unknown>);
            static_assertions::assert_eq_align!(*mut ffav_sys::AVStream, Stream<Unknown>);
            std::mem::transmute(streams)
        }
    }

    /// Get the next packet from the Format and place it in the provided packet object
    ///
    /// Note: Existing data will be unreferenced from the packet
    pub fn get_next_packet_into(&mut self, packet: &mut Packet) -> Result<()> {
        packet.unref();
        unsafe {
            let err = av_read_frame(self.ctx, packet.as_raw());

            if err < 0 {
                return Err(Error::from_av_err("reading next packet from format", err));
            }
        }

        Ok(())
    }

    /// Get the next packet from the Format and return it
    pub fn get_next_packet(&mut self) -> Result<Packet> {
        let mut packet = Packet::new();

        self.get_next_packet_into(&mut packet)?;

        Ok(packet)
    }
}

impl Format<Output> {
    /// Open a file as an output target
    ///
    /// The format type will be automatically determined by the file extension
    /// provided
    pub fn open_output<P: AsRef<Path>>(_file: P) -> Result<Format<Output>> {
        unimplemented!()
    }

    /// Open a filoe as an output target with a specific format type
    pub fn open_with_format_type(_info: ()) -> Result<Format<Output>> {
        unimplemented!()
    }
}

impl<T> std::ops::Drop for Format<T> {
    fn drop(&mut self) {
        unsafe {
            // Make sure we aren't null before dereferencing to check if we are
            // input or output
            if !self.ctx.is_null() {
                // Input types will have a valid `iformat` pointer
                let input_format = (*self.ctx).iformat;
                if !input_format.is_null() {
                    avformat_close_input(&mut self.ctx);
                }
            }

            avformat_free_context(self.ctx);

            self.ctx = std::ptr::null_mut();
        }
    }
}

// SAFETY: Format<T> has no interior mutabillity and is the single owner of the
// pointer contained within.
unsafe impl<T> std::marker::Send for Format<T> {}
unsafe impl<T> std::marker::Sync for Format<T> {}
