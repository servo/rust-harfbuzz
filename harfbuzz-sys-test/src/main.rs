#![allow(bad_style, improper_ctypes)]

extern crate harfbuzz_sys;

use harfbuzz_sys::coretext::*;
use harfbuzz_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
