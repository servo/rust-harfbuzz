fn build_harfbuzz() {
    use std::env;
    use std::path::PathBuf;

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

    if target.contains("windows") {
        cfg.define("HAVE_DIRECTWRITE", "1");
    }

    if target.contains("windows-gnu") {
        cfg.flag("-Wa,-mbig-obj");
    }

    cfg.compile("embedded_harfbuzz");

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:include={}",
        out_dir.join("harfbuzz").join("src").display()
    );
}

fn main() {
    if cfg!(feature = "bundled") {
        build_harfbuzz();
    } else {
        // Use the pre-installed harfbuzz.
        pkg_config::probe_library("harfbuzz").unwrap();
    }
}
