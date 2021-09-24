/// A Container specific identifier for a Stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamID(i32);

impl StreamID {
    /// Create a new StreamID
    pub fn new(id: i32) -> StreamID {
        StreamID(id)
    }
}
