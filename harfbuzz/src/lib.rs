// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! HarfBuzz is a text shaping engine. It solves the problem of selecting
//! and positioning glyphs from a font given a Unicode string.

#![no_std]
#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "harfbuzz-sys")]
pub extern crate harfbuzz_sys as sys;

/// An error type for this crate
#[derive(Debug)]
pub enum HarfBuzzError {
    /// Allocation failed within HarfBuzz itself
    Alloc,
}
pub use HarfBuzzError as Error;

#[cfg(feature = "harfbuzz-sys")]
mod buffer;
#[cfg(feature = "harfbuzz-sys")]
pub use self::buffer::Buffer;

#[cfg(feature = "harfbuzz-sys")]
mod direction;
#[cfg(feature = "harfbuzz-sys")]
pub use self::direction::Direction;

#[cfg(feature = "harfbuzz-sys")]
mod language;
#[cfg(feature = "harfbuzz-sys")]
pub use self::language::Language;

#[cfg(feature = "harfbuzz-sys")]
mod blob;
#[cfg(feature = "harfbuzz-sys")]
pub use self::blob::Blob;

mod traits;
pub use self::traits::CombiningClassFunc;
pub use self::traits::ComposeFunc;
pub use self::traits::DecomposeFunc;
pub use self::traits::GeneralCategoryFunc;
pub use self::traits::MirroringFunc;
pub use self::traits::ScriptFunc;

#[cfg(feature = "harfbuzz-sys")]
mod unicode_funcs;
#[cfg(feature = "harfbuzz-sys")]
pub use self::unicode_funcs::{UnicodeFuncs, UnicodeFuncsBuilder};
