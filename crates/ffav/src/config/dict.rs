use ffav_sys::{
    av_dict_copy, av_dict_free, av_dict_get, av_dict_set, AVDictionary, AVDictionaryEntry,
    AV_DICT_APPEND, AV_DICT_IGNORE_SUFFIX, AV_DICT_MATCH_CASE, AV_DICT_MULTIKEY,
};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::{CStr, CString},
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

    /// Create a new owned dictionary from an existing `AVDictionary` pointer
    ///
    /// This operation clones the existing dictionary so that this dictionary
    /// has full ownership.
    ///
    /// # Safety
    /// The provided `AVDictionary` must have been allocated by libav
    pub unsafe fn from_av_dict(dict: *mut AVDictionary) -> Dictionary {
        let mut new_dict = std::ptr::null_mut();

        // libav will copy this for us
        av_dict_copy(&mut new_dict, dict, 0);

        Dictionary { dict: new_dict }
    }

    /// Add a value to the dictonary
    ///
    /// # Panics
    /// If either the Key or Value string cannot be represented as a C-style string (eg. they contain NULL bytes)
    /// or if allocation/reallocation fails when adding items to the dictionary
    pub fn add<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key_str = CString::new(key.as_ref())
            .expect("Provided key could not be represented as a C-style string");
        let value_str = CString::new(value.as_ref())
            .expect("Provided value could not be represented as a C-style string");

        self.raw_add_with_flags(&key_str, &value_str, AddFlags::empty());
    }

    /// Add a value to the dictonary with the specified flags
    ///
    /// # Panics
    /// If either the Key or Value string cannot be represented as a C-style string (eg. they contain NULL bytes)
    /// or if allocation/reallocation fails when adding items to the dictionary
    pub fn add_with_flags<K, V>(&mut self, key: K, value: V, flags: AddFlags)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key_str = CString::new(key.as_ref())
            .expect("Provided key could not be represented as a C-style string");
        let value_str = CString::new(value.as_ref())
            .expect("Provided value could not be represented as a C-style string");

        self.raw_add_with_flags(&key_str, &value_str, flags);
    }

    /// Add a CStr value to the dictonary
    ///
    /// This function exists for when the Key/Value needed cannot be represented
    /// as a Rust String, or when the Rust String would fail conversion to a
    /// C String.
    ///
    /// # Panics
    /// If allocation/reallocation fails when adding items to the dictionary
    pub fn raw_add<K, V>(&mut self, key: &CStr, value: &CStr) {
        self.raw_add_with_flags(key, value, AddFlags::empty());
    }

    /// Add a CStr value to the dictionary with the provided key
    ///
    /// This function exists for when the Key/Value needed cannot be represented
    /// as a Rust String, or when the Rust String would fail conversion to a
    /// C String.
    ///
    /// # Panics
    /// If allocation/reallocation fails when adding items to the dictionary
    pub fn raw_add_with_flags(&mut self, key: &CStr, value: &CStr, flags: AddFlags) {
        unsafe {
            let ret = av_dict_set(&mut self.dict, key.as_ptr(), value.as_ptr(), flags.bits());

            if ret < 0 {
                panic!("Could not set value")
            }
        }
    }

    /// Get the first item from the dictionary which matches the key
    ///
    /// This is equivalent to `dict.get_with_flags(key, GetFlags::MATCH_CASE)`
    ///
    /// # Panics
    /// If the provided key cannot be safely converted into a CStr
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&CStr> {
        self.get_with_flags(key, GetFlags::MATCH_CASE)
    }

    /// Get the first item from the dictionary with the specified flags
    ///
    /// # Panics
    /// If the provided key cannot be safely converted into a CStr
    pub fn get_with_flags<K: AsRef<str>>(&self, key: K, flags: GetFlags) -> Option<&CStr> {
        let key_str = CString::new(key.as_ref())
            .expect("Provided key could not be represented as a C-style string");

        unsafe {
            let entry = av_dict_get(
                self.as_const_dict(),
                key_str.as_ptr(),
                std::ptr::null(),
                flags.bits(),
            );

            if entry.is_null() {
                None
            } else {
                Some(CStr::from_ptr((*entry).value))
            }
        }
    }

    /// Get the first item from the dictionary which matches the CStr key
    ///
    /// This is equivalent to `dict.raw_get_with_flags(key, GetFlags::MATCH_CASE)`
    pub fn raw_get(&self, key: &CStr) -> Option<&CStr> {
        self.raw_get_with_flags(key, GetFlags::MATCH_CASE)
    }

    /// Get the first item from the dictionary which matches the CStr key with the specified flags
    pub fn raw_get_with_flags(&self, key: &CStr, flags: GetFlags) -> Option<&CStr> {
        unsafe {
            let entry = av_dict_get(
                self.as_const_dict(),
                key.as_ptr(),
                std::ptr::null(),
                flags.bits(),
            );

            if entry.is_null() {
                None
            } else {
                Some(CStr::from_ptr((*entry).value))
            }
        }
    }

    /// Get all items from the dictionary which match the key
    ///
    /// # Panics
    /// If the provided key cannot be safely converted into a CStr
    pub fn get_all<K: AsRef<str>>(&self, key: K) -> Vec<&CStr> {
        self.get_all_with_flags(key, GetFlags::MATCH_CASE)
    }

    /// Get all items from the dictionary which match the key with the specified flags
    ///
    /// # Panics
    /// If the provided key cannot be safely converted into a CStr
    pub fn get_all_with_flags<K: AsRef<str>>(&self, key: K, flags: GetFlags) -> Vec<&CStr> {
        let key_str = CString::new(key.as_ref())
            .expect("Provided key could not be represented as a C-style string");

        self.raw_get_all_with_flags(&key_str, flags)
    }

    /// Get all items from the dictionary which match the CStr key
    pub fn raw_get_all(&self, key: &CStr) -> Vec<&CStr> {
        self.raw_get_all_with_flags(key, GetFlags::MATCH_CASE)
    }

    /// Get all the items from the dictionary which match the CStr key with the specified flags
    pub fn raw_get_all_with_flags(&self, key: &CStr, flags: GetFlags) -> Vec<&CStr> {
        let mut prev = std::ptr::null();

        let mut out = Vec::new();

        unsafe {
            loop {
                let entry = av_dict_get(self.as_const_dict(), key.as_ptr(), prev, flags.bits());

                if entry.is_null() {
                    return out;
                } else {
                    out.push(CStr::from_ptr((*entry).value));
                    prev = entry;
                }
            }
        }
    }

    /// Get an iterator over the keys in this Dictionary
    pub fn keys(&self) -> Keys<'_> {
        Keys {
            entry: std::ptr::null_mut(),
            dict: self,
        }
    }

    /// Get an iterator over all the values in this Dictionary
    pub fn values(&self) -> Values<'_> {
        Values {
            entry: std::ptr::null_mut(),
            dict: self,
        }
    }

    /// Get an iterator over all key/value pairs in this Dictionary
    pub fn items(&self) -> Items<'_> {
        Items {
            entry: std::ptr::null_mut(),
            dict: self,
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

    /// Get a const pointer to the underlying `AVDictionary`
    ///
    /// # Safety
    /// The user must not call `av_dict_free` on the resulting pointer.
    /// The user should assume that the Dictionary is mutably borrowed as long
    /// as the raw pointer exists.
    /// Refer to the expected state of the dictionary after using with any raw
    /// libav function, most modify the dictionary.
    pub unsafe fn as_const_dict(&self) -> *const AVDictionary {
        self.dict
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

/// Convert a Dictionary into a HashMap
///
/// NOTE: CStr in the dictionary will be converted lossily, additionally
/// a Dictionary with multiple entries per Key will only contain one entry
impl From<Dictionary> for HashMap<String, String> {
    fn from(dict: Dictionary) -> Self {
        dict.items()
            .map(|(key, value)| (key.to_string_lossy().into(), value.to_string_lossy().into()))
            .collect()
    }
}

/// Convert a Dictionary into a BTreeMap
///
/// NOTE: CStr in the dictionary will be converted lossily, additionally
/// a Dictionary with multiple entries per Key will only contain one entry
impl From<Dictionary> for BTreeMap<String, String> {
    fn from(dict: Dictionary) -> Self {
        dict.items()
            .map(|(key, value)| (key.to_string_lossy().into(), value.to_string_lossy().into()))
            .collect()
    }
}

/// An iterator over all of the Keys in a Dictionary
pub struct Keys<'dict> {
    entry: *mut AVDictionaryEntry,
    dict: &'dict Dictionary,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
        let empty = CString::new("").unwrap();
        unsafe {
            let next = av_dict_get(
                self.dict.as_const_dict(),
                empty.as_ptr(),
                self.entry,
                AV_DICT_IGNORE_SUFFIX,
            );

            if next.is_null() {
                return None;
            }

            let out = CStr::from_ptr((*next).key);

            self.entry = next;

            Some(out)
        }
    }
}

/// An iterator over all of the Values in a Dictionary
pub struct Values<'dict> {
    entry: *mut AVDictionaryEntry,
    dict: &'dict Dictionary,
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
        let empty = CString::new("").unwrap();
        unsafe {
            let next = av_dict_get(
                self.dict.as_const_dict(),
                empty.as_ptr(),
                self.entry,
                AV_DICT_IGNORE_SUFFIX,
            );

            if next.is_null() {
                return None;
            }

            let out = CStr::from_ptr((*next).value);

            self.entry = next;

            Some(out)
        }
    }
}

/// An iterator over all the item pairs `(K,V)` in a Dictionary
pub struct Items<'dict> {
    entry: *mut AVDictionaryEntry,
    dict: &'dict Dictionary,
}

impl<'a> Iterator for Items<'a> {
    type Item = (&'a CStr, &'a CStr);

    fn next(&mut self) -> Option<Self::Item> {
        let empty = CString::new("").unwrap();
        unsafe {
            let next = av_dict_get(
                self.dict.as_const_dict(),
                empty.as_ptr(),
                self.entry,
                AV_DICT_IGNORE_SUFFIX,
            );

            if next.is_null() {
                return None;
            }

            let key = CStr::from_ptr((*next).key);
            let value = CStr::from_ptr((*next).value);

            self.entry = next;

            Some((key, value))
        }
    }
}

bitflags::bitflags! {
    /// Flags which can be used when getting entries from a Dictionary
    pub struct GetFlags: i32 {
        /// Match a key with exact capitalization
        const MATCH_CASE = AV_DICT_MATCH_CASE;
        /// Find a key with a matching prefix
        const IGNORE_SUFFIX = AV_DICT_IGNORE_SUFFIX;
    }

    /// Flags which can be used when adding entries to a Dictionary
    pub struct AddFlags: i32 {
        /// Append the provided value string to any existing value
        ///
        /// No delimiter will be added the two values will be concatenated
        const APPEND = AV_DICT_APPEND;
        /// Allow one Key to have two independent values
        ///
        /// NOTE: If a dictionary containing multiple Keys is converted to a
        /// `HashMap<String, String>` or `BTreeMap<String, String>` only one
        /// entry will be preserved.
        const MULTIPLE = AV_DICT_MULTIKEY;
    }
}
