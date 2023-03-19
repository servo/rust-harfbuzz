#[cfg(feature = "bindgen")]
extern crate bindgen_;
#[cfg(feature = "build-native-harfbuzz")]
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(feature = "bindgen")]
mod bindings {
    use std::env;
    use std::fs;
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

    static WRAPPER: &'static str = "#include \"hb.h\"
#include \"hb-ot.h\"
#include \"hb-aat.h\"
";

    struct BindingsWriter {
        file: fs::File,
        ignore: regex::Regex,
        replacer: regex::Regex,
    }
    impl BindingsWriter {
        fn new(path: &Path) -> BindingsWriter {
            BindingsWriter {
                file: fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)
                    .unwrap(),
                ignore: regex::Regex::new("__u?int(8|16|32)_t ").unwrap(),
                replacer: regex::Regex::new("(i|u)(8|16|32)_").unwrap(),
            }
        }
    }

    impl Write for BindingsWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let c = unsafe { std::str::from_utf8_unchecked(buf) };
            for line in c.lines() {
                if !self.ignore.is_match(&line) {
                    let rep = self.replacer.replacen(&line, 0, "${1}${2}");
                    self.file.write(rep.as_bytes())?;
                    self.file.write("\n".as_bytes())?;
                }
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.file.flush()
        }
    }

    pub(super) fn gen(include_dirs: &[PathBuf]) {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let mut builder = bindgen_::builder()
            .no_convert_floats()
            .prepend_enum_name(false)
            .allowlist_type("hb_.*")
            .allowlist_function("hb_.*")
            .header_contents("wrapper.h", WRAPPER);

        for include_dir in include_dirs {
            builder = builder
                .clang_arg("-I")
                .clang_arg(include_dir.display().to_string());
        }

        let writer = BindingsWriter::new(&out_dir.join("bindings.rs"));
        println!("cargo:bindings={}", out_dir.join("bindings.rs").display());
        builder.generate().unwrap().write(Box::new(writer)).unwrap();
    }
}

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    let target = env::var("TARGET").unwrap();

    let mut cfg = cc::Build::new();
    cfg.cpp(true)
        .flag_if_supported("-std=c++11") // for unix
        .warnings(false)
        .file("harfbuzz/src/harfbuzz.cc");

    if !target.contains("windows") {
        cfg.define("HAVE_PTHREAD", "1");
    }

    if target.contains("apple") {
        cfg.define("HAVE_CORETEXT", "1");
    }

    if target.contains("windows-gnu") {
        cfg.flag("-Wa,-mbig-obj");
    }

    cfg.compile("embedded_harfbuzz");

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = out_dir.join("harfbuzz").join("src");

    println!("cargo:include={}", include_dir.display());
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {
    #[allow(unused_mut)]
    let mut pkgcfg = pkg_config::Config::new();

    // allow other version of harfbuzz when bindgen enabled.
    #[cfg(not(feature = "bindgen"))]
    pkgcfg.range_version("7.1".."8");

    match pkgcfg.probe("harfbuzz") {
        Ok(_lib) => {
            #[cfg(feature = "bindgen")]
            bindings::gen(&_lib.include_paths);
            return;
        }
        Err(_) => {}
    }

    if let Ok(libdir) = env::var("HARFBUZZ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", &libdir);
        if is_static_available("harfbuzz", &PathBuf::from(&libdir)) {
            println!("cargo:rustc-link-lib=static={}", "harfbuzz");
        } else {
            println!("cargo:rustc-link-lib={}", "harfbuzz");
        }
    }
    if let Ok(include_dir) = env::var("HARFBUZZ_INCLUDE_DIR") {
        #[cfg(feature = "bindgen")]
        bindings::gen(&[PathBuf::from(&include_dir)]);
    }
}

#[allow(dead_code)]
/// System libraries should only be linked dynamically
fn is_static_available(name: &str, dir: &Path) -> bool {
    let libname = format!("lib{}.a", name);
    dir.join(&libname).exists()
}
