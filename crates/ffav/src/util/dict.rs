use ffav_sys::{av_dict_copy, av_dict_set, AVDictionary};
use std::ffi::CString;

pub struct Dictionary {
    dict: *mut AVDictionary,
}

impl Dictionary {
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

    pub fn as_dict(&mut self) -> *mut *mut AVDictionary {
        &mut self.dict
    }
}

impl std::default::Default for Dictionary {
    fn default() -> Self {
        Self::new()
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
