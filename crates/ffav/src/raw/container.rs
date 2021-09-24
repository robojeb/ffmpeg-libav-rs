//! The Container type

use crate::{
    config::Dictionary,
    error::{Error, Result},
    util::{marker::Input, path_to_cstr},
};
use ffav_sys::{av_read_frame, avformat_find_stream_info, avformat_open_input, AVFormatContext};
use std::{marker::PhantomData, path::Path};

use super::packet::Packet;

/// A Continer or Format which has one or more streams of media data and associated meta-data
pub struct Container<IO> {
    fmt: *mut AVFormatContext,
    _io: PhantomData<IO>,
}

impl Container<Input> {
    /// Open a media file as input
    ///
    /// The format type will be automatically determined by the extension and magic-number information
    ///
    /// # Panics
    /// Will panic if any allocation required to open the file fails
    pub fn open_input<P: AsRef<Path>>(file_path: P) -> Result<Container<Input>> {
        let mut empty_params = Dictionary::new();
        Container::open_input_with_options(file_path, &mut empty_params)
    }

    /// Open a media file as input with the specified parameter dictionary
    ///
    /// The format type will be automatically determined by the extension and magic-number information
    ///
    /// # Panics
    /// Will panic if any allocation required to open the file fails
    pub fn open_input_with_options<P: AsRef<Path>>(
        file_path: P,
        options: &mut Dictionary,
    ) -> Result<Container<Input>> {
        Self::inner_open(file_path.as_ref(), options)
    }

    // Helper function to reduce the amount of LLVM-IR is generated based on the number
    // of instantiations of the various open functions are used
    fn inner_open(file_path: &Path, options: &mut Dictionary) -> Result<Container<Input>> {
        let cfile_path = path_to_cstr(file_path)?;
        unsafe {
            let mut ctx = std::ptr::null_mut();

            let ret = avformat_open_input(
                &mut ctx,
                cfile_path.as_ptr(),
                std::ptr::null_mut(),
                options.as_dict(),
            );

            if ret < 0 {
                return Err(Error::Unknown("Error while openting input format"));
                // return Err(Error::from_av_err("opening input format", ret));
            }

            let ret = avformat_find_stream_info(ctx, std::ptr::null_mut());

            if ret < 0 {
                return Err(Error::Unknown("Error while fetching stream information"));
                //return Err(Error::from_av_err("getting stream info", ret));
            }

            // let config = FormatConfig::from_input_fmt_ctx(ctx);

            Ok(Container {
                fmt: ctx,
                _io: PhantomData,
            })
        }
    }

    /// Get the next packet from the Format and place it in the provided packet object
    ///
    /// Note: Existing data will be unreferenced from the packet
    pub fn get_next_packet_into(&mut self, packet: &mut Packet) -> Result<()> {
        packet.unref();
        unsafe {
            let err = av_read_frame(self.fmt, packet.as_ptr_mut());

            if err < 0 {
                return Err(Error::Unknown("Error while reading packet from Container"));
                //return Err(Error::from_av_err("reading next packet from format", err));
            }
        }

        Ok(())
    }

    /// Get the next packet from the Format and return a newly allocated Packet
    ///
    /// # Panics
    /// Pancis if the new Packet cannot be allocated
    pub fn get_next_packet(&mut self) -> Result<Packet> {
        let mut packet = Packet::new();

        self.get_next_packet_into(&mut packet)?;

        Ok(packet)
    }
}
