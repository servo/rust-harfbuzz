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

pub struct Features {
    features: Vec<sys::hb_feature_t>,
}

impl Features {
    pub fn add_from_string(&mut self, s: &str) -> Result<(), ()> {
        unsafe {
            let mut feature = std::mem::zeroed();
            if sys::hb_feature_from_string(
                s.as_ptr() as *const std::os::raw::c_char,
                s.len() as std::os::raw::c_int,
                &mut feature
            ) == 0 {
                return Err(())
            }
            self.features.push(feature);
        }
        Ok(())
    }

    pub fn raw_features(&self) -> &[sys::hb_feature_t] {
        &self.features
    }
}
