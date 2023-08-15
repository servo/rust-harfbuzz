#[cfg(feature = "use-pkg-config-harfbuzz")]
fn find_harfbuzz_via_pkgconfig() {
    pkg_config::probe_library("harfbuzz").is_ok();
}

#[cfg(feature = "build-native-harfbuzz")]
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
    #[cfg(feature = "use-pkg-config-harfbuzz")]
    {
        find_harfbuzz_via_pkgconfig();
        return;
    }

    #[cfg(feature = "build-native-harfbuzz")]
    build_harfbuzz();
}
