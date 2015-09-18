/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::process::Command;
use std::env;


fn main() {
    assert!(Command::new("make")
        .args(&["-R", "-f", "makefile.cargo", &format!("-j{}", env::var("NUM_JOBS").unwrap())])
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=harfbuzz");
    if env::var("TARGET").unwrap().contains("unknown-linux-gnu") {
        println!("cargo:rustc-link-lib=glib-2.0");
    }
    println!("cargo:rustc-link-lib=stdc++");
}
