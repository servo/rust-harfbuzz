extern crate winapi;

use hb_face_t;

use self::winapi::um::dwrite::IDWriteFontFace;

extern "C" {
    pub fn hb_directwrite_face_create(font_face: *mut IDWriteFontFace) -> *mut hb_face_t;
}
