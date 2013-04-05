# Copyright 2013 The Servo Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

import argparse, subprocess;

bindgen = "bindgen"

harfbuzz = "../harfbuzz/src/hb.h"
includes = [
    "-I", "../harfbuzz/src",
    "-I", "."
    ]
sysincludes = [
    "-isystem", "/usr/lib/x86_64-linux-gnu/gcc/x86_64-linux-gnu/4.5/include",
    "-isystem", "/usr/lib/gcc/x86_64-linux-gnu/4.6/include"
    ]
otherflags = [
    ]

args = [
    bindgen,
    "-l", "harfbuzz",
    "-o", "harfbuzz.rs",
    "-match", "hb",
    harfbuzz]
args += includes + sysincludes + otherflags

subprocess.call(args)

        
