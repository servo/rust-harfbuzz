#[cfg(any(target_os = "android", all(unix, not(target_vendor = "apple"))))]
extern crate freetype;

#[cfg(target_vendor = "apple")]
pub mod coretext;

#[cfg(any(target_os = "android", all(unix, not(target_vendor = "apple"))))]
extern "C" {
    pub fn hb_ft_font_create_referenced(face: freetype::freetype::FT_Face) -> *mut hb_font_t;
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(clippy::unreadable_literal)]
#[cfg(not(feature = "bindgen"))]
mod bindings;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(clippy::unreadable_literal)]
#[cfg(feature = "bindgen")]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
