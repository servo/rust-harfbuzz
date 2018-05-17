// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use std;
use sys;

pub struct Glyphs<'a> {
    infos: *const sys::hb_glyph_info_t,
    positions: *const sys::hb_glyph_position_t,
    len: usize,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Glyphs<'a> {
    pub unsafe fn from_raw(
        infos: *const sys::hb_glyph_info_t,
        positions: *const sys::hb_glyph_position_t,
        len: std::os::raw::c_uint,
    ) -> Self {
        Glyphs {
            infos,
            positions,
            len: len as usize,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Glyph<'a> {
        assert!(idx < self.len, "Index {} is out of range", idx);
        unsafe { self.get_unchecked(idx) }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, idx: usize) -> Glyph<'a> {
        Glyph {
            info: self.infos.add(idx),
            position: self.positions.add(idx),
            phantom: self.phantom,
        }
    }
}

pub struct Glyph<'a> {
    info: *const sys::hb_glyph_info_t,
    position: *const sys::hb_glyph_position_t,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Glyph<'a> {
    /// Either a Unicode code point (before shaping) or a glyph ID (after shaping)
    pub fn id(&self) -> u32 {
        self.raw_info().codepoint
    }

    /// For glyphs that represent Unicode code points (before shaping), returns the Unicode
    /// code point as a `char`.
    ///
    /// Not valid (and may panic) if called after shaping.
    pub fn to_char(&self) -> char {
        std::char::from_u32(self.id()).expect("not a Unicode code point")
    }

    /// Access to the raw `hb_glyph_info_t`
    pub fn raw_info(&self) -> &sys::hb_glyph_info_t {
        unsafe { &*self.info }
    }

    /// Access to the raw `hb_glyph_position_t`
    pub fn raw_position(&self) -> &sys::hb_glyph_position_t {
        unsafe { &*self.position }
    }
}
