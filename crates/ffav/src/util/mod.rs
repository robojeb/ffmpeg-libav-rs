//! Structures and types which help enforce type safety in the use of ffav

pub mod color;
pub mod marker;
pub mod sampling;
pub mod time;
pub mod traits;
mod types;

use crate::error::Result;
use std::{ffi::CString, path::Path};

pub use types::*;

#[cfg(unix)]
pub(crate) fn path_to_cstr(p: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(p.as_os_str().as_bytes())?)
}

#[cfg(not(unix))]
pub(crate) fn path_to_cstr(p: &Path) -> Result<CString> {
    let s = p.to_str().ok_or_else(|| Error::InvalidPath(p.to_owned()))?;
    Ok(CString::new(s)?)
}

pub(crate) fn make_id_from_ptr<T>(ptr: *mut T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut s = DefaultHasher::new();
    ptr.hash(&mut s);
    s.finish()
}
