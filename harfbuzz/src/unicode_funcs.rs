// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sys::*;
use std::ffi::c_void;

/// An object to map from code points to general category properties.
pub trait GeneralCategoryFunc {
    /// Given a code point, return the general category as a
    /// [`hb_unicode_general_category_t`].
    fn general_category(&self, ch: u32) -> hb_unicode_general_category_t;
}

impl GeneralCategoryFunc for () {
    fn general_category(&self, ch: u32) -> hb_unicode_general_category_t {
        unreachable!()
    }
}

/// An object to map from code points to combining classes.
pub trait CombiningClassFunc {
    /// Given a code point, return the combining class as a
    /// [`hb_unicode_combining_class_t`].
    fn combining_class(&self, ch: u32) -> hb_unicode_combining_class_t;
}

impl CombiningClassFunc for () {
    fn combining_class(&self, ch: u32) -> hb_unicode_combining_class_t {
        unreachable!()
    }
}

/// An object to map from code points to mirrored code points.
pub trait MirroringFunc {
    /// Given a code point, return the mirrored code point.
    fn mirroring(&self, ch: u32) -> u32;
}

impl MirroringFunc for () {
    fn mirroring(&self, ch: u32) -> u32 {
        unreachable!()
    }
}

/// An object to map from code points to script names.
pub trait ScriptFunc {
    /// Given a code point, return the script as a 4-byte script name.
    fn script(&self, ch: u32) -> [u8; 4];
}

impl ScriptFunc for () {
    fn script(&self, ch: u32) -> [u8; 4] {
        unreachable!()
    }
}

/// An object to compose two characters.
pub trait ComposeFunc {
    /// Given two code points, return the composed code point.
    fn compose(&self, a: u32, b: u32) -> Option<u32>;
}

impl ComposeFunc for () {
    fn compose(&self, a: u32, b: u32) -> Option<u32> {
        unreachable!()
    }
}

/// An object to decompose a character.
pub trait DecomposeFunc {
    /// Given a code point, return the two decomposed code points.
    fn decompose(&self, ab: u32) -> Option<(u32, u32)>;
}

impl DecomposeFunc for () {
    fn decompose(&self, ab: u32) -> Option<(u32, u32)> {
        unreachable!()
    }
}

/// A builder for [`UnicodeFuncs`].
#[non_exhaustive]
pub struct UnicodeFuncsBuilder<F0, F1, F2, F3, F4, F5> {
    /// Optional implementation of [`hb_unicode_general_category_func_t`].
    pub general_category: Option<Box<F0>>,
    /// Optional implementation of [`hb_unicode_combining_class_func_t`].
    pub combining_class: Option<Box<F1>>,
    /// Optional implementation of [`hb_unicode_mirroring_func_t`].
    pub mirroring: Option<Box<F2>>,
    /// Optional implementation of [`hb_unicode_script_func_t`].
    pub script: Option<Box<F3>>,
    /// Optional implementation of [`hb_unicode_compose_func_t`].
    pub compose: Option<Box<F4>>,
    /// Optional implementation of [`hb_unicode_decompose_func_t`].
    pub decompose: Option<Box<F5>>,
}

impl<F0, F1, F2, F3, F4, F5> UnicodeFuncsBuilder<F0, F1, F2, F3, F4, F5> {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Self {
            general_category: None,
            combining_class: None,
            mirroring: None,
            script: None,
            compose: None,
            decompose: None,
        }
    }
}

impl<F0, F1, F2, F3, F4, F5> UnicodeFuncsBuilder<F0, F1, F2, F3, F4, F5>
where
    F0: GeneralCategoryFunc,
    F1: CombiningClassFunc,
    F2: MirroringFunc,
    F3: ScriptFunc,
    F4: ComposeFunc,
    F5: DecomposeFunc,
{
    /// Creates a [`UnicodeFuncs`] based on the fields in this builder.
    pub fn build(self) -> Result<UnicodeFuncs, ()> {
        unsafe { self.build_unsafe() }
    }

    unsafe fn build_unsafe(self) -> Result<UnicodeFuncs, ()> {
        let empty = hb_unicode_funcs_get_empty();
        // The HarfBuzz refcounting convention is that "create"
        // sets refcount to one, not zero.
        // https://harfbuzz.github.io/object-model-lifecycle.html
        let ufuncs = hb_unicode_funcs_create(empty);
        if ufuncs == empty {
            // return Err(HarfBuzzError::Alloc);
            return Err(());
        }

        if let Some(general_category) = self.general_category {
            let general_category_ptr: *mut F0 = Box::into_raw(general_category);
            extern "C" fn impl_general_category<F0: GeneralCategoryFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                unicode: hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_unicode_general_category_t {
                unsafe { &*(user_data as *mut F0) }.general_category(unicode)
            }
            extern "C" fn destroy_general_category<F0>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F0) };
            }
            hb_unicode_funcs_set_general_category_func(
                ufuncs,
                Some(impl_general_category::<F0>),
                general_category_ptr as *mut c_void,
                Some(destroy_general_category::<F0>),
            );
        }

        if let Some(combining_class) = self.combining_class {
            let combining_class_ptr: *mut F1 = Box::into_raw(combining_class);
            extern "C" fn impl_combining_class<F1: CombiningClassFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                unicode: hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_unicode_combining_class_t {
                unsafe { &*(user_data as *mut F1) }.combining_class(unicode)
            }
            extern "C" fn destroy_combining_class<F1>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F1) };
            }
            hb_unicode_funcs_set_combining_class_func(
                ufuncs,
                Some(impl_combining_class::<F1>),
                combining_class_ptr as *mut c_void,
                Some(destroy_combining_class::<F1>),
            );
        }

        if let Some(mirroring) = self.mirroring {
            let mirroring_ptr: *mut F2 = Box::into_raw(mirroring);
            extern "C" fn impl_mirroring<F2: MirroringFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                unicode: hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_codepoint_t {
                unsafe { &*(user_data as *mut F2) }.mirroring(unicode)
            }
            extern "C" fn destroy_mirroring<F2>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F2) };
            }
            hb_unicode_funcs_set_mirroring_func(
                ufuncs,
                Some(impl_mirroring::<F2>),
                mirroring_ptr as *mut c_void,
                Some(destroy_mirroring::<F2>),
            );
        }

        if let Some(script) = self.script {
            let script_ptr: *mut F3 = Box::into_raw(script);
            extern "C" fn impl_script<F3: ScriptFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                unicode: hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_codepoint_t {
                let code = unsafe { &*(user_data as *mut F3) }.script(unicode);
                unsafe { hb_script_from_string(code.as_ptr() as *const i8, 4) }
            }
            extern "C" fn destroy_script<F3>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F3) };
            }
            hb_unicode_funcs_set_script_func(
                ufuncs,
                Some(impl_script::<F3>),
                script_ptr as *mut c_void,
                Some(destroy_script::<F3>),
            );
        }

        if let Some(compose) = self.compose {
            let compose_ptr: *mut F4 = Box::into_raw(compose);
            extern "C" fn impl_compose<F4: ComposeFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                a: hb_codepoint_t,
                b: hb_codepoint_t,
                ab: *mut hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_bool_t {
                let result = unsafe { &*(user_data as *mut F4) }.compose(a, b);
                match result {
                    Some(ab_x) => {
                        unsafe { *ab = ab_x };
                        true as hb_bool_t
                    }
                    None => false as hb_bool_t,
                }
            }
            extern "C" fn destroy_compose<F4>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F4) };
            }
            hb_unicode_funcs_set_compose_func(
                ufuncs,
                Some(impl_compose::<F4>),
                compose_ptr as *mut c_void,
                Some(destroy_compose::<F4>),
            );
        }

        if let Some(decompose) = self.decompose {
            let decompose_ptr: *mut F5 = Box::into_raw(decompose);
            extern "C" fn impl_decompose<F5: DecomposeFunc>(
                _ufuncs: *mut hb_unicode_funcs_t,
                ab: hb_codepoint_t,
                a: *mut hb_codepoint_t,
                b: *mut hb_codepoint_t,
                user_data: *mut c_void,
            ) -> hb_bool_t {
                let result = unsafe { &*(user_data as *mut F5) }.decompose(ab);
                match result {
                    Some((a_x, b_x)) => {
                        unsafe { *a = a_x };
                        unsafe { *b = b_x };
                        true as hb_bool_t
                    }
                    None => false as hb_bool_t,
                }
            }
            extern "C" fn destroy_decompose<F5>(user_data: *mut c_void) {
                let _ = unsafe { Box::from_raw(user_data as *mut F5) };
            }
            hb_unicode_funcs_set_decompose_func(
                ufuncs,
                Some(impl_decompose::<F5>),
                decompose_ptr as *mut c_void,
                Some(destroy_decompose::<F5>),
            );
        }

        // Compatibility decomposition and East Asian Width lookups
        // are deprecated, and there's no need to set up the callbacks
        // for those.

        hb_unicode_funcs_make_immutable(ufuncs);
        Ok(UnicodeFuncs::from_raw(ufuncs))
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
/// let mut unicode_funcs = UnicodeFuncsBuilder::<_, (), (), (), (), ()>::new();
/// unicode_funcs.general_category = Some(Box::new(GeneralCategoryCalculator));
/// let unicode_funcs = unicode_funcs.build().unwrap();
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
    pub fn into_raw(funcs: Self) -> *mut hb_unicode_funcs_t {
        let ret = funcs.raw;
        core::mem::forget(funcs);
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
