// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # harfbuzz-traits
//!
//! This crate provides traits for the `harfbuzz` crate that can be implememted without
//! depending on [HarfBuzz](https://harfbuzz.github.io/) itself.

#![no_std]

mod unicode_funcs_traits;

pub use unicode_funcs_traits::*;
