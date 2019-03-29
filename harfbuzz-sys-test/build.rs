extern crate ctest;

use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Get the include paths from the environment variable exported by harfbuzz-sys. If the
    // variables is not found, fail with a meaningful error.
    match &env::var_os("DEP_HARFBUZZ_INCLUDE") {
        Some(include_paths) => {
            for path in env::split_paths(include_paths) {
                cfg.include(path);
            }
        }
        None => panic!("$DEP_HARFBUZZ_INCLUDE not found."),
    }

    // Include the header files where the C APIs are defined.
    cfg.header("hb.h").header("hb-ot.h").header("hb-aat.h");

    #[cfg(target_os = "macos")]
    cfg.header("hb-coretext.h");

    // Skip structs that are empty on the Rust side.
    cfg.skip_struct(|s| {
        s == "hb_language_impl_t"
            || s == "hb_blob_t"
            || s == "hb_unicode_funcs_t"
            || s == "hb_set_t"
            || s == "hb_face_t"
            || s == "hb_font_t"
            || s == "hb_font_funcs_t"
            || s == "hb_buffer_t"
            || s == "hb_map_t"
            || s == "hb_shape_plan_t"
    });

    // FIXME: I'm not sure why these functions must be skipped.
    cfg.skip_fn(|s| {
        s == "hb_coretext_face_create"
            || s == "hb_coretext_face_get_cg_font"
            || s == "hb_ft_font_create_referenced"
    });

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../harfbuzz-sys/src/lib.rs", "all.rs");
}
