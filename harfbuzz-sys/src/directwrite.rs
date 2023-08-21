use crate::hb_face_t;

use winapi::um::dwrite::IDWriteFontFace;

extern "C" {
    /// This requires that the `directwrite` feature is enabled.
    pub fn hb_directwrite_face_create(font_face: *mut IDWriteFontFace) -> *mut hb_face_t;
}
