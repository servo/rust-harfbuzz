// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;
use std::os::raw::{c_char, c_uint};
use std::{mem, ops, ptr, slice};
use sys;

/// Blobs wrap a chunk of binary data to handle lifecycle management of data
/// while it is passed between client and HarfBuzz.
///
/// Blobs are primarily used to create font faces, but also to access font face
/// tables, as well as pass around other binary data.
pub struct Blob<'a> {
    raw: *mut sys::hb_blob_t,
    phantom: PhantomData<&'a sys::hb_blob_t>,
}

impl<'a> Blob<'a> {
    /// Create a new read-only blob.
    ///
    /// The data is not copied, so it must outlive the
    /// `Blob`.
    ///
    /// ```
    /// let data = vec![1; 256];
    /// let blob = Blob::new_read_only(&data);
    /// assert_eq!(blob.len(), 256);
    /// assert!(!blob.is_empty());
    /// ```
    pub fn new_read_only(data: &'a [u8]) -> Blob<'a> {
        assert!(data.len() <= c_uint::max_value() as usize);
        unsafe {
            Blob::from_raw(sys::hb_blob_create(
                data.as_ptr() as *const c_char,
                data.len() as c_uint,
                sys::HB_MEMORY_MODE_READONLY,
                ptr::null_mut(), // user data
                None, // destroy callback
            ))
        }
    }

    /// Construct a `Blob` from a raw pointer. Takes ownership of the blob.
    pub unsafe fn from_raw(raw: *mut sys::hb_blob_t) -> Self {
        Blob {
            raw,
            phantom: PhantomData,
        }
    }

    /// Returns the size of the blob in bytes.
    pub fn len(&self) -> usize {
        unsafe { sys::hb_blob_get_length(self.raw) as usize }
    }

    /// Returns true if the length is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Make this blob immutable.
    pub fn make_immutable(&mut self) {
        unsafe { sys::hb_blob_make_immutable(self.raw); }
    }

    /// Returns true if the blob is immutable.
    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_blob_is_immutable(self.raw) != 0 }
    }

    /// Borrows a raw pointer to the blob.
    pub fn as_raw(&self) -> *mut sys::hb_blob_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the blob.
    pub fn into_raw(self) -> *mut sys::hb_blob_t {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

impl<'a> ops::Deref for Blob<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = sys::hb_blob_get_data(self.raw, &mut len);
            assert!(!ptr.is_null(), "hb_blob_get_data failed");
            slice::from_raw_parts(ptr as *const u8, len as usize)
        }
    }
}

impl<'a> ops::DerefMut for Blob<'a> {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            let mut len = 0;
            let ptr = sys::hb_blob_get_data_writable(self.raw, &mut len);
            assert!(!ptr.is_null(), "hb_blob_get_data_writable failed");
            slice::from_raw_parts_mut(ptr as *mut u8, len as usize)
        }
    }
}

impl<'a> Drop for Blob<'a> {
    /// Decrement the reference count, and destroy the blob if the reference count is zero.
    fn drop(&mut self) {
        unsafe { sys::hb_blob_destroy(self.raw); }
    }
}
