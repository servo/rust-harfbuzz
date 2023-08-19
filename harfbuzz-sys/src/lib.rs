#[cfg(target_vendor = "apple")]
pub mod coretext;

#[cfg(all(target_family = "windows", feature = "directwrite"))]
pub mod directwrite;

#[cfg(feature = "freetype")]
extern "C" {
    pub fn hb_ft_font_create_referenced(face: freetype::freetype::FT_Face) -> *mut hb_font_t;
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(clippy::unreadable_literal)]
#[allow(rustdoc::bare_urls)]
#[allow(rustdoc::broken_intra_doc_links)]
mod bindings;

pub use crate::bindings::*;
