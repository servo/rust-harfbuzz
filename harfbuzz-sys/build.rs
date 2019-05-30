#[cfg(feature = "build-native-harfbuzz")]
extern crate cmake;
#[cfg(feature = "build-native-harfbuzz")]
extern crate pkg_config;

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-env-changed=HARFBUZZ_SYS_NO_PKG_CONFIG");
    if target.contains("wasm32") || env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::probe_library("harfbuzz").is_ok() {
            return;
        }
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // On Windows, HarfBuzz configures atomics directly; otherwise,
    // it needs assistance from configure to do so.  Just use the makefile
    // build for now elsewhere.
    if target.contains("windows") || target.contains("wasm32") {
        let mut cfg = cmake::Config::new("harfbuzz");
        if target.contains("wasm") {
            // When building on macOS for wasm32, make sure we aren't picking up
            // CoreText.
            cfg.define("HB_HAVE_CORETEXT", "OFF");
            if target == "wasm32-unknown-unknown" {
                // Switch to the correct target triple for the underlying toolchain.
                cfg.target("wasm32-unknown-none");
            }
        }
        let dst = cfg.build();
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=harfbuzz");
        if target.contains("gnu") {
            println!("cargo:rustc-link-lib=stdc++");
        }
    } else {
        assert!(Command::new("make")
            .env("MAKEFLAGS", env::var("CARGO_MAKEFLAGS").unwrap_or_default())
            .args(&["-R", "-f", "makefile.cargo"])
            .status()
            .unwrap()
            .success());

        println!(
            "cargo:rustc-link-search=native={}",
            out_dir.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=harfbuzz");
    }

    // DEP_HARFBUZZ_INCLUDE has the path of the vendored harfbuzz.
    println!(
        "cargo:include={}",
        out_dir.join("include").join("harfbuzz").display()
    );
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {}
