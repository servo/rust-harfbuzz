#[cfg(feature = "bindgen")]
extern crate bindgen_;
#[cfg(feature = "build-native-harfbuzz")]
extern crate cc;
#[cfg(feature = "build-native-harfbuzz")]
extern crate pkg_config;

#[cfg(feature = "bindgen")]
mod bindings {
    use std::env;
    use std::fs;
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

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
            .header_contents(
                "wrapper.h",
                "#include \"hb.h\"\n#include \"hb-ot.h\"\n#include \"hb-aat.h\"",
            );

        for include_dir in include_dirs {
            builder = builder
                .clang_arg("-I")
                .clang_arg(include_dir.display().to_string());
        }

        let writer = BindingsWriter::new(&out_dir.join("bindings.rs"));
        builder.generate().unwrap().write(Box::new(writer)).unwrap();
    }
}

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-env-changed=HARFBUZZ_SYS_NO_PKG_CONFIG");
    if target.contains("wasm32") || env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if let Ok(_lib) = pkg_config::probe_library("harfbuzz") {
            #[cfg(feature = "bindgen")]
            {
                bindings::gen(&_lib.include_paths);
            }
            return;
        }
    }

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

    #[cfg(feature = "bindgen")]
    {
        bindings::gen(&[include_dir.clone()]);
    }

    println!("cargo:include={}", include_dir.display());
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {}
