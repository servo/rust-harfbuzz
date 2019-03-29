#![allow(bad_style, improper_ctypes)]

extern crate harfbuzz_sys;

#[cfg(target_os = "macos")]
use harfbuzz_sys::coretext::*;
use harfbuzz_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
