// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// An object to map from code points to general category properties.
pub trait GeneralCategoryFunc {
    /// Given a code point, return the general category as a
    /// [`hb_unicode_general_category_t`].
    fn general_category(&self, ch: u32) -> core::ffi::c_uint;
}

/// An object to map from code points to combining classes.
pub trait CombiningClassFunc {
    /// Given a code point, return the combining class as a
    /// [`hb_unicode_combining_class_t`].
    fn combining_class(&self, ch: u32) -> core::ffi::c_uint;
}

/// An object to map from code points to mirrored code points.
pub trait MirroringFunc {
    /// Given a code point, return the mirrored code point.
    fn mirroring(&self, ch: u32) -> u32;
}

/// An object to map from code points to script names.
pub trait ScriptFunc {
    /// Given a code point, return the script as a 4-byte script name.
    fn script(&self, ch: u32) -> [u8; 4];
}

/// An object to compose two characters.
pub trait ComposeFunc {
    /// Given two code points, return the composed code point.
    fn compose(&self, a: u32, b: u32) -> Option<u32>;
}

/// An object to decompose a character.
pub trait DecomposeFunc {
    /// Given a code point, return the two decomposed code points.
    fn decompose(&self, ab: u32) -> Option<(u32, u32)>;
}
