extern crate freetype as ft;
extern crate harfbuzz_sys as hb_sys;

use ft::Library;
use hb_sys::*;
use hb_sys::hb_ft::hb_ft_font_create;
use std::env;
use std::ptr;
use std::ffi::CString;
use std::slice::from_raw_parts;

// This is a partial translated version of:
// https://github.com/behdad/harfbuzz-tutorial/blob/f29d989/hello-harfbuzz.c
fn main() {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Usage: ft_example font-file.ttf 'some string'");
        return;
    }
    let fontfile = args.nth(1).unwrap();
    let text = CString::new(args.next().unwrap()).unwrap();

    let lib = Library::init().unwrap();
    let mut face = lib.new_face(fontfile, 0).unwrap();
    face.set_char_size(36 * 64, 0, 36 * 64, 0).unwrap();

    unsafe {
        let hb_font = hb_ft_font_create(face.raw_mut(), None);

        let hb_buffer = hb_buffer_create();
        hb_buffer_add_utf8(hb_buffer, text.as_ptr(), -1, 0, -1);
        hb_buffer_guess_segment_properties(hb_buffer);

        hb_shape(hb_font, hb_buffer, ptr::null(), 0);

        let len = hb_buffer_get_length(hb_buffer);
        let info_raw = hb_buffer_get_glyph_infos(hb_buffer, ptr::null_mut());
        let pos_raw = hb_buffer_get_glyph_positions(hb_buffer, ptr::null_mut());
        let info = from_raw_parts(info_raw, len as usize);
        let pos = from_raw_parts(pos_raw, len as usize);

        for(p, i) in pos.iter().zip(info) {
            let gid       = i.codepoint;
            let cluster   = i.cluster;
            let x_advance = p.x_advance as f32 / 64.;
            let y_advance = p.y_advance as f32 / 64.;
            let x_offset  = p.x_offset as f32 / 64.;
            let y_offset  = p.y_offset as f32 / 64.;
            println!("gid={} cluster={} advance=({},{}) offset=({},{})",
                     gid, cluster, x_advance, y_advance, x_offset, y_offset);
        }

        hb_buffer_destroy(hb_buffer);
        hb_font_destroy(hb_font);
    }
}
