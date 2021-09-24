//! Structures related to decoded Frame data
use ffav_sys::{av_frame_alloc, av_frame_free, av_frame_unref, AVFrame, AV_NUM_DATA_POINTERS};
use std::{marker::PhantomData, ops::Deref};

use crate::util::{marker::Video, time::TimeBaseTicks, traits::MediaMarker};

/// How many data planes exist in each Frame
pub const NUM_DATA_PLANES: usize = AV_NUM_DATA_POINTERS as usize;

/// A decodec frame of data of type `AV`
pub struct Frame<AV: MediaMarker> {
    frame: *mut AVFrame,
    _type: PhantomData<AV>,
}

impl<AV: MediaMarker> Frame<AV> {
    /// Create a new frame of the specified type
    ///
    /// # Panics
    /// If the new frame cannot be allocated
    pub fn new() -> Frame<AV> {
        Frame {
            frame: unsafe {
                let f = av_frame_alloc();
                // Quick check that we aren't creating a monster (eg invalid Frame struct)
                // Normally we would use NonNull, but because when freeing the `libav*`
                // functions NULL the pointer out we cannot use NonNull or we would
                // violate that invariant which would be bad
                if f.is_null() {
                    panic!("Could not allocate requested frame");
                }
                f
            },
            _type: PhantomData,
        }
    }

    /// Unreference the data held by this frame
    ///
    /// Used storage from an associated Codec may be released.
    /// This frame can be reused to read data from another Format
    pub fn unref(&mut self) {
        unsafe {
            // No need for debug NULL check because `av_frame_unref()` handles
            // that internally
            av_frame_unref(self.frame);
        }
    }

    /// Get the expected presentation time of this frame
    pub fn get_pts(&self) -> TimeBaseTicks {
        unsafe { TimeBaseTicks::new((*self.frame).pts as u64) }
    }

    /// Get the raw mutable pointer to the frame
    ///
    /// Intended as an escape hatch if something is impossible with the abstraction
    /// layer.
    ///
    /// # Safety
    /// The pointer should not be held longer than the life of the `Frame`.
    /// While using the raw pointer it should be considered that the `Frame`
    /// is mutably borrowed.
    pub unsafe fn as_ptr_mut(&mut self) -> *mut AVFrame {
        self.frame
    }

    /// Get the raw constant pointer to the frame
    ///
    /// Intended as an escape hatch if something is impossible with the abstraction
    /// layer.
    ///
    /// # Safety
    /// The pointer should not be held longer than the life of the `Frame`.
    /// While using the raw pointer it should be considered that the `Frame`
    /// is mutably borrowed.
    pub unsafe fn as_ptr(&self) -> *const AVFrame {
        self.frame
    }

    /// Transform this Frame from its current type to another type
    ///
    /// Existing data in this frame will be unreferenced and no longer
    /// accessible.
    pub fn as_other<T: MediaMarker>(self) -> Frame<T> {
        let mut out: Frame<T> = unsafe { std::mem::transmute(self) };
        out.unref();
        out
    }

    // /// Get the data planes for this frame
    // pub fn planes(&self) -> &[&[u8]; NUM_DATA_PLANES] {
    //     unimplemented!()
    // }

    /// Get a read-only slice of the specified plane
    ///
    /// # Panics
    /// Plane must be less than `NUM_DATA_PLANES` or this function
    /// will panic
    pub fn plane(&self, plane: usize) -> Plane<'_> {
        if plane > NUM_DATA_PLANES {
            panic!("The requested plane is outside the range supported by this version of ffmpeg");
        }

        unsafe {
            Plane {
                data: std::slice::from_raw_parts(
                    (*self.frame).data[plane],
                    (*self.frame).linesize[plane] as usize * (*self.frame).height as usize,
                ),
                linesize: (*self.frame).linesize[plane] as usize,
            }
        }
    }
}

impl Frame<Video> {
    /// Get the width of this frame
    pub fn width(&self) -> u32 {
        unsafe { (*self.frame).width as u32 }
    }
    /// Get the height of this frame
    pub fn height(&self) -> u32 {
        unsafe { (*self.frame).height as u32 }
    }
}

impl<AV: MediaMarker> std::ops::Drop for Frame<AV> {
    fn drop(&mut self) {
        unsafe {
            av_frame_unref(self.frame);
            av_frame_free(&mut self.frame);

            // `av_frame_free()` should NULL this pointer, but just in case
            self.frame = std::ptr::null_mut();
        }
    }
}

impl<AV: MediaMarker> std::default::Default for Frame<AV> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: Frame has no interior mutability and is the sole owner of its internal
// pointer.
unsafe impl<AV: MediaMarker> std::marker::Send for Frame<AV> {}
unsafe impl<AV: MediaMarker> std::marker::Sync for Frame<AV> {}

/// A single plane of data from a Frame
pub struct Plane<'frame> {
    /// The underlying data of the Plane
    pub data: &'frame [u8],
    /// How many bytes correspond to each line
    ///
    /// For video data there will be `frame.height()` lines in the `data` buffer
    /// For audio, there will be one line per plane of audio data
    pub linesize: usize,
}

impl<'frame> Deref for Plane<'frame> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}
