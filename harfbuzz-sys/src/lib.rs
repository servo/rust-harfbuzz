//! # harfbuzz-sys
//!
//! This crate provides raw bindings to the [HarfBuzz](https://harfbuzz.github.io/)
//! text shaping library.
//!
//! ## Features
//!
//! - `freetype` - Enables bindings to the FreeType font engine. (Enabled by default.)
//! - `coretext` - Enables bindings to the CoreText font engine. (Apple platforms only) (Enabled by default.)
//! - `directwrite` - Enables bindings to the DirectWrite font engine. (Windows only) (Enabled by default.)
//!
//! - `bundled` - Use the bundled copy of the harfbuzz library rather than one installed on the system.

#[cfg(all(target_vendor = "apple", feature = "coretext"))]
pub mod coretext;

#[cfg(all(target_family = "windows", feature = "directwrite"))]
pub mod directwrite;

#[cfg(feature = "freetype")]
extern "C" {
    /// This requires that the `freetype` feature is enabled.
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
