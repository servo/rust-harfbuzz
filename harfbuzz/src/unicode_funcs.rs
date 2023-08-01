// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::traits::*;
use crate::Error;
use alloc::boxed::Box;
use core::ffi::c_void;
use sys::*;

/// A builder for [`UnicodeFuncs`].
///
/// If one or more of the functions is not provided, set its type parameter to `()`.
#[non_exhaustive]
#[derive(Debug)]
pub struct UnicodeFuncsBuilder {
    raw: *mut hb_unicode_funcs_t,
}

impl UnicodeFuncsBuilder {
    /// Creates a new, empty builder.
    pub fn new_with_empty_parent() -> Result<Self, Error> {
        let parent = unsafe { hb_unicode_funcs_get_empty() };
        // The HarfBuzz refcounting convention is that "create"
        // sets refcount to one, not zero.
        // https://harfbuzz.github.io/object-model-lifecycle.html
        let ufuncs = unsafe { hb_unicode_funcs_create(parent) };
        if ufuncs == parent {
            return Err(Error::Alloc);
        }
        Ok(Self { raw: ufuncs })
    }

    /// Creates a new builder with the parent set to the HarfBuzz default.
    pub fn new_with_harfbuzz_default_parent() -> Result<Self, Error> {
        let parent = unsafe { hb_unicode_funcs_get_default() };
        // The HarfBuzz refcounting convention is that "create"
        // sets refcount to one, not zero.
        // https://harfbuzz.github.io/object-model-lifecycle.html
        let ufuncs = unsafe { hb_unicode_funcs_create(parent) };
        if ufuncs == parent {
            return Err(Error::Alloc);
        }
        Ok(Self { raw: ufuncs })
    }

    /// Sets an implementation of [`hb_unicode_general_category_func_t`].
    pub fn set_general_category_func<F: GeneralCategoryFunc>(&mut self, f: Box<F>) {
        let general_category_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_general_category<F: GeneralCategoryFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            unicode: hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_unicode_general_category_t {
            unsafe { &*(user_data as *mut F) }.general_category(unicode)
        }
        extern "C" fn destroy_general_category<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_general_category_func(
                self.raw,
                Some(impl_general_category::<F>),
                general_category_ptr as *mut c_void,
                Some(destroy_general_category::<F>),
            );
        }
    }

    /// Sets an implementation of [`hb_unicode_combining_class_func_t`].
    pub fn set_combining_class_func<F: CombiningClassFunc>(&mut self, f: Box<F>) {
        let combining_class_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_combining_class<F: CombiningClassFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            unicode: hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_unicode_combining_class_t {
            unsafe { &*(user_data as *mut F) }.combining_class(unicode)
        }
        extern "C" fn destroy_combining_class<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_combining_class_func(
                self.raw,
                Some(impl_combining_class::<F>),
                combining_class_ptr as *mut c_void,
                Some(destroy_combining_class::<F>),
            );
        }
    }

    /// Sets an implementation of [`hb_unicode_mirroring_func_t`].
    pub fn set_mirroring_func<F: MirroringFunc>(&mut self, f: Box<F>) {
        let mirroring_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_mirroring<F: MirroringFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            unicode: hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_codepoint_t {
            unsafe { &*(user_data as *mut F) }.mirroring(unicode)
        }
        extern "C" fn destroy_mirroring<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_mirroring_func(
                self.raw,
                Some(impl_mirroring::<F>),
                mirroring_ptr as *mut c_void,
                Some(destroy_mirroring::<F>),
            );
        }
    }

    /// Sets an implementation of [`hb_unicode_script_func_t`].
    pub fn set_script_func<F: ScriptFunc>(&mut self, f: Box<F>) {
        let script_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_script<F: ScriptFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            unicode: hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_codepoint_t {
            let code = unsafe { &*(user_data as *mut F) }.script(unicode);
            unsafe { hb_script_from_string(code.as_ptr() as *const i8, 4) }
        }
        extern "C" fn destroy_script<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_script_func(
                self.raw,
                Some(impl_script::<F>),
                script_ptr as *mut c_void,
                Some(destroy_script::<F>),
            );
        }
    }

    /// Sets an implementation of [`hb_unicode_compose_func_t`].
    pub fn set_compose_func<F: ComposeFunc>(&mut self, f: Box<F>) {
        let compose_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_compose<F: ComposeFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            a: hb_codepoint_t,
            b: hb_codepoint_t,
            ab: *mut hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_bool_t {
            let result = unsafe { &*(user_data as *mut F) }.compose(a, b);
            match result {
                Some(ab_x) => {
                    unsafe { *ab = ab_x };
                    true as hb_bool_t
                }
                None => false as hb_bool_t,
            }
        }
        extern "C" fn destroy_compose<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_compose_func(
                self.raw,
                Some(impl_compose::<F>),
                compose_ptr as *mut c_void,
                Some(destroy_compose::<F>),
            );
        }
    }

    /// Sets an implementation of [`hb_unicode_decompose_func_t`].
    pub fn set_decompose_func<F: DecomposeFunc>(&mut self, f: Box<F>) {
        let decompose_ptr: *mut F = Box::into_raw(f);
        extern "C" fn impl_decompose<F: DecomposeFunc>(
            _ufuncs: *mut hb_unicode_funcs_t,
            ab: hb_codepoint_t,
            a: *mut hb_codepoint_t,
            b: *mut hb_codepoint_t,
            user_data: *mut c_void,
        ) -> hb_bool_t {
            let result = unsafe { &*(user_data as *mut F) }.decompose(ab);
            match result {
                Some((a_x, b_x)) => {
                    unsafe { *a = a_x };
                    unsafe { *b = b_x };
                    true as hb_bool_t
                }
                None => false as hb_bool_t,
            }
        }
        extern "C" fn destroy_decompose<F>(user_data: *mut c_void) {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
        unsafe {
            hb_unicode_funcs_set_decompose_func(
                self.raw,
                Some(impl_decompose::<F>),
                decompose_ptr as *mut c_void,
                Some(destroy_decompose::<F>),
            );
        }
    }

    /// Creates a [`UnicodeFuncs`] based on this builder.
    pub fn build(self) -> UnicodeFuncs {
        let raw = self.raw;
        unsafe {
            hb_unicode_funcs_make_immutable(raw);
        }
        core::mem::forget(self);
        // Safety: Ownership is being transferred
        unsafe { UnicodeFuncs::from_raw(raw) }
    }

    /// Takes ownership of a `*mut hb_unicode_funcs_t` without incrementing
    /// the refcount.
    ///
    /// # Safety
    ///
    /// After the call, the previous owner must not call
    /// `hb_unicode_funcs_destroy()`, since `UnicodeFuncs` will now
    /// take care of it.
    pub unsafe fn from_raw(raw: *mut hb_unicode_funcs_t) -> Self {
        Self { raw }
    }

    /// Transfers the ownership of the wrapped pointer to the caller.
    /// The caller is responsible for calling `hb_unicode_funcs_destroy()`;
    /// `UnicodeFuncs` will no longer take care of it.
    pub fn into_raw(self) -> *mut hb_unicode_funcs_t {
        let ret = self.raw;
        core::mem::forget(self);
        ret
    }
}

impl Drop for UnicodeFuncsBuilder {
    fn drop(&mut self) {
        unsafe {
            hb_unicode_funcs_destroy(self.raw);
        }
    }
}

/// A collection of functions to run Unicode algorithms required by HarfBuzz.
///
/// Create one of these using a [`UnicodeFuncsBuilder`].
///
/// # Example
///
/// ```
/// # use harfbuzz::{Buffer, Direction, UnicodeFuncsBuilder, sys};
/// struct GeneralCategoryCalculator;
///
/// impl harfbuzz::GeneralCategoryFunc for GeneralCategoryCalculator {
///     fn general_category(&self, ch: u32) -> harfbuzz::sys::hb_unicode_general_category_t {
///         todo!()
///     }
/// }
///
/// let mut unicode_funcs = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
/// unicode_funcs.set_general_category_func(Box::new(GeneralCategoryCalculator));
/// let unicode_funcs = unicode_funcs.build();
///
/// let mut b = Buffer::with("مساء الخير");
/// b.set_unicode_funcs(&unicode_funcs);
/// b.guess_segment_properties();
/// assert_eq!(b.get_direction(), Direction::RTL);
/// assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
/// ```
pub struct UnicodeFuncs {
    raw: *mut hb_unicode_funcs_t,
}

impl UnicodeFuncs {
    /// Takes ownership of a `*mut hb_unicode_funcs_t` without incrementing
    /// the refcount.
    ///
    /// # Safety
    ///
    /// After the call, the previous owner must not call
    /// `hb_unicode_funcs_destroy()`, since `UnicodeFuncs` will now
    /// take care of it.
    pub unsafe fn from_raw(raw: *mut hb_unicode_funcs_t) -> Self {
        Self { raw }
    }

    /// Transfers the ownership of the wrapped pointer to the caller.
    /// The caller is responsible for calling `hb_unicode_funcs_destroy()`;
    /// `UnicodeFuncs` will no longer take care of it.
    pub fn into_raw(self) -> *mut hb_unicode_funcs_t {
        let ret = self.raw;
        core::mem::forget(self);
        ret
    }

    /// Borrows the wrapped raw pointer without transferring ownership
    /// and without affecting the refcount.
    pub fn as_ptr(&self) -> *mut hb_unicode_funcs_t {
        self.raw
    }
}

impl Drop for UnicodeFuncs {
    fn drop(&mut self) {
        unsafe {
            hb_unicode_funcs_destroy(self.raw);
        }
    }
}
