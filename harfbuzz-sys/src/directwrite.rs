use crate::hb_face_t;

use winapi::um::dwrite::IDWriteFontFace;

extern "C" {
    pub fn hb_directwrite_face_create(font_face: *mut IDWriteFontFace) -> *mut hb_face_t;
}
