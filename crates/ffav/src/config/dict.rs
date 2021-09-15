use ffav_sys::{av_dict_copy, av_dict_free, av_dict_set, AVDictionary};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::CString,
    iter::FromIterator,
};

/// Dictonary type used by libav
///
/// This type is esentially a `Map<CString, CString>`, this is internally
/// stored as an `AVDictionary` type to allow easily and quickly passing to
/// libav functions.
pub struct Dictionary {
    dict: *mut AVDictionary,
}

impl Dictionary {
    /// Create a new empty Dictionary
    pub fn new() -> Dictionary {
        Dictionary {
            dict: std::ptr::null_mut(),
        }
    }

    /// Add a value to the dictonary
    ///
    /// Panics: If either the Key or Value string cannot be represented as a C-style string (eg. they contain NULL bytes)
    /// or if allocation/reallocation fails when adding items to the dictionary
    pub fn add<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        unsafe {
            let key_str = CString::new(key.as_ref())
                .expect("Provided key could not be represented as a C-style string");
            let value_str = CString::new(value.as_ref())
                .expect("Provided value could not be represented as a C-style string");

            let ret = av_dict_set(&mut self.dict, key_str.as_ptr(), value_str.as_ptr(), 0);

            if ret < 0 {
                panic!("Could not set value")
            }
        }
    }

    /// Get a pointer to the underlying `AVDictionary`
    ///
    /// # Safety
    /// The user must not call `av_dict_free` on the resulting pointer.
    /// The user should assume that the Dictionary is mutably borrowed as long
    /// as the raw pointer exists.
    /// Refer to the expected state of the dictionary after using with any raw
    /// libav function, most modify the dictionary.
    pub unsafe fn as_dict(&mut self) -> *mut *mut AVDictionary {
        &mut self.dict
    }
}

impl std::default::Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { av_dict_free(&mut self.dict) }
    }
}

impl std::clone::Clone for Dictionary {
    fn clone(&self) -> Self {
        let mut new_dict = std::ptr::null_mut();

        // libav will copy this for us
        unsafe {
            av_dict_copy(&mut new_dict, self.dict, 0);
        }

        Dictionary { dict: new_dict }
    }
}

impl<K, V> From<HashMap<K, V>> for Dictionary
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from(hm: HashMap<K, V>) -> Self {
        hm.into_iter().collect()
    }
}

impl<K, V> From<BTreeMap<K, V>> for Dictionary
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from(hm: BTreeMap<K, V>) -> Self {
        hm.into_iter().collect()
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut dict = Dictionary::new();
        for (k, v) in iter {
            dict.add(k, v)
        }
        dict
    }
}
