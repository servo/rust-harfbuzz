use crate::{hb_face_t, hb_font_t};

use core_graphics::font::CGFont;
use core_text::font::CTFontRef;
use foreign_types::ForeignType;

type CGFontRef = *mut <CGFont as ForeignType>::CType;

extern "C" {
    pub fn hb_coretext_face_create(cg_font: CGFontRef) -> *mut hb_face_t;
    pub fn hb_coretext_font_create(ct_font: CTFontRef) -> *mut hb_font_t;
    pub fn hb_coretext_face_get_cg_font(face: *mut hb_face_t) -> CGFontRef;
    pub fn hb_coretext_font_get_ct_font(font: *mut hb_font_t) -> CTFontRef;
}
