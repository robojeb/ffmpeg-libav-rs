//! Structures for handling encoded packets of data from a Container

use ffav_sys::{av_packet_alloc, av_packet_free, av_packet_unref, AVPacket};

/// An encoded packet of data from a Format data contents are of an unknown
/// type and must be passed to the proper Codec to be decoded into a frame.
pub struct Packet {
    pkt: *mut AVPacket,
}

impl Packet {
    /// Create a new Packet which references no data
    ///
    /// # Panics
    /// Pancis if the Packet cannot be allocated
    pub fn new() -> Packet {
        Packet {
            pkt: unsafe {
                let p = av_packet_alloc();
                // Quick check that we aren't creating a monster (eg invalid Packet struct)
                // Normally we would use NonNull, but because when freeing the `libav*`
                // functions will NULL the pointer out we cannot use NonNull or we would
                // violate that invariant which would be bad
                if p.is_null() {
                    panic!("Could not allocate requested packet");
                }
                p
            },
        }
    }

    /// Unreference the data held by this packet
    ///
    /// Used storage from an associated Format may be released.
    /// This packet can be reused to read data from another Format
    pub fn unref(&mut self) {
        unsafe {
            // No need for debug NULL check because `av_packet_unref()` handles
            // that internally
            av_packet_unref(self.pkt);
        }
    }

    /// Get the index of the stream this packet was received from, or is intended
    /// to be added to.
    pub fn stream_index(&self) -> usize {
        debug_assert!(
            !self.pkt.is_null(),
            "Invalid NULL packet was created and used"
        );
        unsafe {
            debug_assert!((*self.pkt).stream_index >= 0, "Negative index was provided");
            (*self.pkt).stream_index as usize
        }
    }

    // /// Chack if this packet belongs to the provided stream
    // pub fn is_for_stream<AV>(&self, stream: &Stream<AV>) -> bool {
    //     stream.is_packet_for_stream(self)
    // }

    /// Get the raw mutable pointer to the Packet
    ///
    /// Intended as an escape hatch if something is impossible with the abstraction
    /// layer.
    ///
    /// # Safety
    /// The pointer should not be held longer than the life of the `Packet`.
    /// While using the raw pointer it should be considered that the `Packet`
    /// is mutably borrowed.
    pub unsafe fn as_ptr_mut(&mut self) -> *mut AVPacket {
        self.pkt
    }

    /// Get the raw const pointer to the Packet
    ///
    /// Intended as an escape hatch if something is impossible with the abstraction
    /// layer.
    ///
    /// # Safety
    /// The pointer should not be held longer than the life of the `Packet`.
    /// While using the raw pointer it should be considered that the `Packet`
    /// is mutably borrowed.
    pub unsafe fn as_ptr(&self) -> *const AVPacket {
        self.pkt
    }
}

impl std::ops::Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            av_packet_unref(self.pkt);
            av_packet_free(&mut self.pkt);

            // `av_packet_free()` should NULL this pointer, but just in case
            self.pkt = std::ptr::null_mut();
        }
    }
}

impl std::default::Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: Packet has no interior mutability and is the sole owner of its internal
// pointer.
unsafe impl std::marker::Send for Packet {}
unsafe impl std::marker::Sync for Packet {}
