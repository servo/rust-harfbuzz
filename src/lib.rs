// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "harfbuzz"]
#![crate_type = "rlib"]

#![feature(globs)]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::*;

pub mod linkhack;

pub type hb_bool_t = c_int;

pub type hb_codepoint_t = uint32_t;

pub type hb_position_t = int32_t;

pub type hb_mask_t = uint32_t;

pub type union__hb_var_int_t = i32;

pub type hb_var_int_t = union__hb_var_int_t;

pub type hb_tag_t = uint32_t;


pub type hb_direction_t = c_uint;
pub const HB_DIRECTION_INVALID: u32 = 0_u32;
pub const HB_DIRECTION_LTR: u32 = 4_u32;
pub const HB_DIRECTION_RTL: u32 = 5_u32;
pub const HB_DIRECTION_TTB: u32 = 6_u32;
pub const HB_DIRECTION_BTT: u32 = 7_u32;

pub type struct__hb_language_t = c_void;

pub type hb_language_t = *mut struct__hb_language_t;


pub type hb_unicode_general_category_t = c_uint;
pub const HB_UNICODE_GENERAL_CATEGORY_CONTROL: u32 = 0_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_FORMAT: u32 = 1_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED: u32 = 2_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE: u32 = 3_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_SURROGATE: u32 = 4_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER: u32 = 5_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER: u32 = 6_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER: u32 = 7_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER: u32 = 8_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER: u32 = 9_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_SPACING_MARK: u32 = 10_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK: u32 = 11_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK: u32 = 12_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER: u32 = 13_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER: u32 = 14_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER: u32 = 15_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_CONNECT_PUNCTUATION: u32 = 16_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_DASH_PUNCTUATION: u32 = 17_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_CLOSE_PUNCTUATION: u32 = 18_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_FINAL_PUNCTUATION: u32 = 19_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_INITIAL_PUNCTUATION: u32 = 20_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_PUNCTUATION: u32 = 21_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_OPEN_PUNCTUATION: u32 = 22_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL: u32 = 23_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL: u32 = 24_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL: u32 = 25_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL: u32 = 26_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_LINE_SEPARATOR: u32 = 27_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_PARAGRAPH_SEPARATOR: u32 = 28_u32;
pub const HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR: u32 = 29_u32;


pub type hb_script_t = c_uint;
pub const HB_SCRIPT_COMMON: u32 = 1517910393_u32;
pub const HB_SCRIPT_ARABIC: u32 = 1098015074_u32;
pub const HB_SCRIPT_ARMENIAN: u32 = 1098018158_u32;
pub const HB_SCRIPT_BENGALI: u32 = 1113943655_u32;
pub const HB_SCRIPT_BOPOMOFO: u32 = 1114599535_u32;
pub const HB_SCRIPT_CANADIAN_ABORIGINAL: u32 = 1130458739_u32;
pub const HB_SCRIPT_CHEROKEE: u32 = 1130915186_u32;
pub const HB_SCRIPT_COPTIC: u32 = 1131376756_u32;
pub const HB_SCRIPT_CYRILLIC: u32 = 1132032620_u32;
pub const HB_SCRIPT_DEVANAGARI: u32 = 1147500129_u32;
pub const HB_SCRIPT_GEORGIAN: u32 = 1197830002_u32;
pub const HB_SCRIPT_GREEK: u32 = 1198679403_u32;
pub const HB_SCRIPT_GUJARATI: u32 = 1198877298_u32;
pub const HB_SCRIPT_GURMUKHI: u32 = 1198879349_u32;
pub const HB_SCRIPT_HANGUL: u32 = 1214344807_u32;
pub const HB_SCRIPT_HAN: u32 = 1214344809_u32;
pub const HB_SCRIPT_HEBREW: u32 = 1214603890_u32;
pub const HB_SCRIPT_HIRAGANA: u32 = 1214870113_u32;
pub const HB_SCRIPT_INHERITED: u32 = 1516858984_u32;
pub const HB_SCRIPT_KANNADA: u32 = 1265525857_u32;
pub const HB_SCRIPT_KATAKANA: u32 = 1264676449_u32;
pub const HB_SCRIPT_LAO: u32 = 1281453935_u32;
pub const HB_SCRIPT_LATIN: u32 = 1281455214_u32;
pub const HB_SCRIPT_MALAYALAM: u32 = 1298954605_u32;
pub const HB_SCRIPT_MONGOLIAN: u32 = 1299148391_u32;
pub const HB_SCRIPT_OGHAM: u32 = 1332175213_u32;
pub const HB_SCRIPT_ORIYA: u32 = 1332902241_u32;
pub const HB_SCRIPT_RUNIC: u32 = 1383427698_u32;
pub const HB_SCRIPT_SYRIAC: u32 = 1400468067_u32;
pub const HB_SCRIPT_TAMIL: u32 = 1415671148_u32;
pub const HB_SCRIPT_TELUGU: u32 = 1415933045_u32;
pub const HB_SCRIPT_THAI: u32 = 1416126825_u32;
pub const HB_SCRIPT_YI: u32 = 1500080489_u32;
pub const HB_SCRIPT_TIBETAN: u32 = 1416192628_u32;
pub const HB_SCRIPT_ETHIOPIC: u32 = 1165256809_u32;
pub const HB_SCRIPT_KHMER: u32 = 1265134962_u32;
pub const HB_SCRIPT_MYANMAR: u32 = 1299803506_u32;
pub const HB_SCRIPT_SINHALA: u32 = 1399418472_u32;
pub const HB_SCRIPT_THAANA: u32 = 1416126817_u32;
pub const HB_SCRIPT_DESERET: u32 = 1148416628_u32;
pub const HB_SCRIPT_GOTHIC: u32 = 1198486632_u32;
pub const HB_SCRIPT_OLD_ITALIC: u32 = 1232363884_u32;
pub const HB_SCRIPT_BUHID: u32 = 1114990692_u32;
pub const HB_SCRIPT_HANUNOO: u32 = 1214344815_u32;
pub const HB_SCRIPT_TAGALOG: u32 = 1416064103_u32;
pub const HB_SCRIPT_TAGBANWA: u32 = 1415669602_u32;
pub const HB_SCRIPT_BRAILLE: u32 = 1114792297_u32;
pub const HB_SCRIPT_CYPRIOT: u32 = 1131442804_u32;
pub const HB_SCRIPT_LIMBU: u32 = 1281977698_u32;
pub const HB_SCRIPT_LINEAR_B: u32 = 1281977954_u32;
pub const HB_SCRIPT_OSMANYA: u32 = 1332964705_u32;
pub const HB_SCRIPT_SHAVIAN: u32 = 1399349623_u32;
pub const HB_SCRIPT_TAI_LE: u32 = 1415670885_u32;
pub const HB_SCRIPT_UGARITIC: u32 = 1432838514_u32;
pub const HB_SCRIPT_BUGINESE: u32 = 1114990441_u32;
pub const HB_SCRIPT_GLAGOLITIC: u32 = 1198285159_u32;
pub const HB_SCRIPT_KHAROSHTHI: u32 = 1265131890_u32;
pub const HB_SCRIPT_NEW_TAI_LUE: u32 = 1415670901_u32;
pub const HB_SCRIPT_OLD_PERSIAN: u32 = 1483761007_u32;
pub const HB_SCRIPT_SYLOTI_NAGRI: u32 = 1400466543_u32;
pub const HB_SCRIPT_TIFINAGH: u32 = 1415999079_u32;
pub const HB_SCRIPT_BALINESE: u32 = 1113681001_u32;
pub const HB_SCRIPT_CUNEIFORM: u32 = 1483961720_u32;
pub const HB_SCRIPT_NKO: u32 = 1315663727_u32;
pub const HB_SCRIPT_PHAGS_PA: u32 = 1349017959_u32;
pub const HB_SCRIPT_PHOENICIAN: u32 = 1349021304_u32;
pub const HB_SCRIPT_UNKNOWN: u32 = 1517976186_u32;
pub const HB_SCRIPT_CARIAN: u32 = 1130459753_u32;
pub const HB_SCRIPT_CHAM: u32 = 1130914157_u32;
pub const HB_SCRIPT_KAYAH_LI: u32 = 1264675945_u32;
pub const HB_SCRIPT_LEPCHA: u32 = 1281716323_u32;
pub const HB_SCRIPT_LYCIAN: u32 = 1283023721_u32;
pub const HB_SCRIPT_LYDIAN: u32 = 1283023977_u32;
pub const HB_SCRIPT_OL_CHIKI: u32 = 1332503403_u32;
pub const HB_SCRIPT_REJANG: u32 = 1382706791_u32;
pub const HB_SCRIPT_SAURASHTRA: u32 = 1398895986_u32;
pub const HB_SCRIPT_SUNDANESE: u32 = 1400204900_u32;
pub const HB_SCRIPT_VAI: u32 = 1449224553_u32;
pub const HB_SCRIPT_AVESTAN: u32 = 1098281844_u32;
pub const HB_SCRIPT_BAMUM: u32 = 1113681269_u32;
pub const HB_SCRIPT_EGYPTIAN_HIEROGLYPHS: u32 = 1164409200_u32;
pub const HB_SCRIPT_IMPERIAL_ARAMAIC: u32 = 1098018153_u32;
pub const HB_SCRIPT_INSCRIPTIONAL_PAHLAVI: u32 = 1349020777_u32;
pub const HB_SCRIPT_INSCRIPTIONAL_PARTHIAN: u32 = 1349678185_u32;
pub const HB_SCRIPT_JAVANESE: u32 = 1247901281_u32;
pub const HB_SCRIPT_KAITHI: u32 = 1265920105_u32;
pub const HB_SCRIPT_LISU: u32 = 1281979253_u32;
pub const HB_SCRIPT_MEETEI_MAYEK: u32 = 1299473769_u32;
pub const HB_SCRIPT_OLD_SOUTH_ARABIAN: u32 = 1398895202_u32;
pub const HB_SCRIPT_OLD_TURKIC: u32 = 1332898664_u32;
pub const HB_SCRIPT_SAMARITAN: u32 = 1398893938_u32;
pub const HB_SCRIPT_TAI_THAM: u32 = 1281453665_u32;
pub const HB_SCRIPT_TAI_VIET: u32 = 1415673460_u32;
pub const HB_SCRIPT_BATAK: u32 = 1113683051_u32;
pub const HB_SCRIPT_BRAHMI: u32 = 1114792296_u32;
pub const HB_SCRIPT_MANDAIC: u32 = 1298230884_u32;
pub const HB_SCRIPT_CHAKMA: u32 = 1130457965_u32;
pub const HB_SCRIPT_MEROITIC_CURSIVE: u32 = 1298494051_u32;
pub const HB_SCRIPT_MEROITIC_HIEROGLYPHS: u32 = 1298494063_u32;
pub const HB_SCRIPT_MIAO: u32 = 1349284452_u32;
pub const HB_SCRIPT_SHARADA: u32 = 1399353956_u32;
pub const HB_SCRIPT_SORA_SOMPENG: u32 = 1399812705_u32;
pub const HB_SCRIPT_TAKRI: u32 = 1415670642_u32;
pub const HB_SCRIPT_INVALID: u32 = 0_u32;

#[repr(C)]
pub struct struct__hb_user_data_key_t {
    _unused: c_char,
}

pub type hb_user_data_key_t = struct__hb_user_data_key_t;

pub type hb_destroy_func_t = extern "C" fn(*mut c_void);


pub type hb_memory_mode_t = c_uint;
pub const HB_MEMORY_MODE_DUPLICATE: u32 = 0_u32;
pub const HB_MEMORY_MODE_READONLY: u32 = 1_u32;
pub const HB_MEMORY_MODE_WRITABLE: u32 = 2_u32;
pub const HB_MEMORY_MODE_READONLY_MAY_MAKE_WRITABLE: u32 = 3_u32;

pub type struct__hb_blob_t = c_void;

pub type hb_blob_t = struct__hb_blob_t;

pub type struct__hb_unicode_funcs_t = c_void;

pub type hb_unicode_funcs_t = struct__hb_unicode_funcs_t;

pub type hb_unicode_combining_class_func_t = *mut u8;

pub type hb_unicode_eastasian_width_func_t = *mut u8;

pub type hb_unicode_general_category_func_t = *mut u8;

pub type hb_unicode_mirroring_func_t = *mut u8;

pub type hb_unicode_script_func_t = *mut u8;

pub type hb_unicode_compose_func_t = *mut u8;

pub type hb_unicode_decompose_func_t = *mut u8;

pub type struct__hb_buffer_t = c_void;

pub type hb_buffer_t = struct__hb_buffer_t;

#[repr(C)]
pub struct struct__hb_glyph_info_t {
    pub codepoint: hb_codepoint_t,
    pub mask: hb_mask_t,
    pub cluster: uint32_t,
    _var1: hb_var_int_t,
    _var2: hb_var_int_t,
}

pub type hb_glyph_info_t = struct__hb_glyph_info_t;

#[repr(C)]
pub struct struct__hb_glyph_position_t {
    pub x_advance: hb_position_t,
    pub y_advance: hb_position_t,
    pub x_offset: hb_position_t,
    pub y_offset: hb_position_t,
    _var: hb_var_int_t,
}

pub type hb_glyph_position_t = struct__hb_glyph_position_t;

pub type struct__hb_face_t = c_void;

pub type hb_face_t = struct__hb_face_t;

pub type struct__hb_font_t = c_void;

pub type hb_font_t = struct__hb_font_t;

pub type hb_reference_table_func_t = extern "C" fn(*mut hb_face_t, hb_tag_t, *mut c_void) -> *mut hb_blob_t;

pub type struct__hb_font_funcs_t = c_void;

pub type hb_font_funcs_t = struct__hb_font_funcs_t;

#[repr(C)]
pub struct struct__hb_glyph_extents_t {
    _x_bearing: hb_position_t,
    _y_bearing: hb_position_t,
    _width: hb_position_t,
    _height: hb_position_t,
}

pub type hb_glyph_extents_t = struct__hb_glyph_extents_t;

pub type hb_font_get_glyph_func_t = extern "C" fn(*mut hb_font_t, *mut c_void, hb_codepoint_t, hb_codepoint_t, *mut hb_codepoint_t, *mut c_void) -> hb_bool_t;

pub type hb_font_get_glyph_advance_func_t = extern "C" fn(*mut hb_font_t, *mut c_void, hb_codepoint_t, *mut c_void) -> hb_position_t;

pub type hb_font_get_glyph_h_advance_func_t = hb_font_get_glyph_advance_func_t;

pub type hb_font_get_glyph_v_advance_func_t = hb_font_get_glyph_advance_func_t;

pub type hb_font_get_glyph_origin_func_t = *mut u8;

pub type hb_font_get_glyph_h_origin_func_t = hb_font_get_glyph_origin_func_t;

pub type hb_font_get_glyph_v_origin_func_t = hb_font_get_glyph_origin_func_t;

pub type hb_font_get_glyph_kerning_func_t = extern "C" fn(*mut hb_font_t, *mut c_void, first_glyph: hb_codepoint_t, second_glyph: hb_codepoint_t, *mut c_void) -> hb_position_t;

pub type hb_font_get_glyph_h_kerning_func_t = hb_font_get_glyph_kerning_func_t;

pub type hb_font_get_glyph_v_kerning_func_t = hb_font_get_glyph_kerning_func_t;

pub type hb_font_get_glyph_extents_func_t = *mut u8;

pub type hb_font_get_glyph_contour_point_func_t = *mut u8;

pub type struct__hb_set_t = c_void;

pub type hb_set_t = struct__hb_set_t;

#[repr(C)]
pub struct struct__hb_feature_t {
    pub _tag: hb_tag_t,
    pub _value: uint32_t,
    pub _start: c_uint,
    pub _end: c_uint,
}

pub type hb_feature_t = struct__hb_feature_t;

extern {

pub fn RUST_hb_tag_from_string(str: *mut c_char, len: c_int) -> hb_tag_t;

pub fn RUST_hb_direction_from_string(str: *mut c_char, len: c_int) -> hb_direction_t;

pub fn RUST_hb_direction_to_string(direction: hb_direction_t) -> *mut c_char;

pub fn RUST_hb_language_from_string(str: *mut c_char, len: c_int) -> hb_language_t;

pub fn RUST_hb_language_to_string(language: hb_language_t) -> *mut c_char;

pub fn RUST_hb_language_get_default() -> hb_language_t;

pub fn RUST_hb_script_from_iso15924_tag(tag: hb_tag_t) -> hb_script_t;

pub fn RUST_hb_script_from_string(s: *mut c_char, len: c_int) -> hb_script_t;

pub fn RUST_hb_script_to_iso15924_tag(script: hb_script_t) -> hb_tag_t;

pub fn RUST_hb_script_get_horizontal_direction(script: hb_script_t) -> hb_direction_t;

pub fn RUST_hb_blob_create(data: *const c_char, length: c_uint, mode: hb_memory_mode_t, user_data: *mut c_void, destroy: hb_destroy_func_t) -> *mut hb_blob_t;

pub fn RUST_hb_blob_create_sub_blob(parent: *mut hb_blob_t, offset: c_uint, length: c_uint) -> *mut hb_blob_t;

pub fn RUST_hb_blob_get_empty() -> *mut hb_blob_t;

pub fn RUST_hb_blob_reference(blob: *mut hb_blob_t) -> *mut hb_blob_t;

pub fn RUST_hb_blob_destroy(blob: *mut hb_blob_t);

pub fn RUST_hb_blob_set_user_data(blob: *mut hb_blob_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_blob_get_user_data(blob: *mut hb_blob_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_blob_make_immutable(blob: *mut hb_blob_t);

pub fn RUST_hb_blob_is_immutable(blob: *mut hb_blob_t) -> hb_bool_t;

pub fn RUST_hb_blob_get_length(blob: *mut hb_blob_t) -> c_uint;

pub fn RUST_hb_blob_get_data(blob: *mut hb_blob_t, length: *mut c_uint) -> *mut c_char;

pub fn RUST_hb_blob_get_data_writable(blob: *mut hb_blob_t, length: *mut c_uint) -> *mut c_char;

pub fn RUST_hb_unicode_funcs_get_default() -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_unicode_funcs_create(parent: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_unicode_funcs_get_empty() -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_unicode_funcs_reference(ufuncs: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_unicode_funcs_destroy(ufuncs: *mut hb_unicode_funcs_t);

pub fn RUST_hb_unicode_funcs_set_user_data(ufuncs: *mut hb_unicode_funcs_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_unicode_funcs_get_user_data(ufuncs: *mut hb_unicode_funcs_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_unicode_funcs_make_immutable(ufuncs: *mut hb_unicode_funcs_t);

pub fn RUST_hb_unicode_funcs_is_immutable(ufuncs: *mut hb_unicode_funcs_t) -> hb_bool_t;

pub fn RUST_hb_unicode_funcs_get_parent(ufuncs: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_unicode_funcs_set_combining_class_func(ufuncs: *mut hb_unicode_funcs_t, combining_class_func: hb_unicode_combining_class_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_eastasian_width_func(ufuncs: *mut hb_unicode_funcs_t, eastasian_width_func: hb_unicode_eastasian_width_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_general_category_func(ufuncs: *mut hb_unicode_funcs_t, general_category_func: hb_unicode_general_category_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_mirroring_func(ufuncs: *mut hb_unicode_funcs_t, mirroring_func: hb_unicode_mirroring_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_script_func(ufuncs: *mut hb_unicode_funcs_t, script_func: hb_unicode_script_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_compose_func(ufuncs: *mut hb_unicode_funcs_t, compose_func: hb_unicode_compose_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_funcs_set_decompose_func(ufuncs: *mut hb_unicode_funcs_t, decompose_func: hb_unicode_decompose_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_unicode_combining_class(ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t) -> c_uint;

pub fn RUST_hb_unicode_eastasian_width(ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t) -> c_uint;

pub fn RUST_hb_unicode_general_category(ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t) -> hb_unicode_general_category_t;

pub fn RUST_hb_unicode_mirroring(ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t) -> hb_codepoint_t;

pub fn RUST_hb_unicode_script(ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t) -> hb_script_t;

pub fn RUST_hb_unicode_compose(ufuncs: *mut hb_unicode_funcs_t, a: hb_codepoint_t, b: hb_codepoint_t, ab: *mut hb_codepoint_t) -> hb_bool_t;

pub fn RUST_hb_unicode_decompose(ufuncs: *mut hb_unicode_funcs_t, ab: hb_codepoint_t, a: *mut hb_codepoint_t, b: *mut hb_codepoint_t) -> hb_bool_t;

pub fn RUST_hb_buffer_create() -> *mut hb_buffer_t;

pub fn RUST_hb_buffer_get_empty() -> *mut hb_buffer_t;

pub fn RUST_hb_buffer_reference(buffer: *mut hb_buffer_t) -> *mut hb_buffer_t;

pub fn RUST_hb_buffer_destroy(buffer: *mut hb_buffer_t);

pub fn RUST_hb_buffer_set_user_data(buffer: *mut hb_buffer_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_buffer_get_user_data(buffer: *mut hb_buffer_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_buffer_set_unicode_funcs(buffer: *mut hb_buffer_t, unicode_funcs: *mut hb_unicode_funcs_t);

pub fn RUST_hb_buffer_get_unicode_funcs(buffer: *mut hb_buffer_t) -> *mut hb_unicode_funcs_t;

pub fn RUST_hb_buffer_set_direction(buffer: *mut hb_buffer_t, direction: hb_direction_t);

pub fn RUST_hb_buffer_get_direction(buffer: *mut hb_buffer_t) -> hb_direction_t;

pub fn RUST_hb_buffer_set_script(buffer: *mut hb_buffer_t, script: hb_script_t);

pub fn RUST_hb_buffer_get_script(buffer: *mut hb_buffer_t) -> hb_script_t;

pub fn RUST_hb_buffer_set_language(buffer: *mut hb_buffer_t, language: hb_language_t);

pub fn RUST_hb_buffer_get_language(buffer: *mut hb_buffer_t) -> hb_language_t;

pub fn RUST_hb_buffer_reset(buffer: *mut hb_buffer_t);

pub fn RUST_hb_buffer_pre_allocate(buffer: *mut hb_buffer_t, size: c_uint) -> hb_bool_t;

pub fn RUST_hb_buffer_allocation_successful(buffer: *mut hb_buffer_t) -> hb_bool_t;

pub fn RUST_hb_buffer_reverse(buffer: *mut hb_buffer_t);

pub fn RUST_hb_buffer_reverse_clusters(buffer: *mut hb_buffer_t);

pub fn RUST_hb_buffer_guess_properties(buffer: *mut hb_buffer_t);

pub fn RUST_hb_buffer_add(buffer: *mut hb_buffer_t, codepoint: hb_codepoint_t, mask: hb_mask_t, cluster: c_uint);

pub fn RUST_hb_buffer_add_utf8(buffer: *mut hb_buffer_t, text: *const c_char, text_length: c_int, item_offset: c_uint, item_length: c_int);

pub fn RUST_hb_buffer_add_utf16(buffer: *mut hb_buffer_t, text: *const uint16_t, text_length: c_int, item_offset: c_uint, item_length: c_int);

pub fn RUST_hb_buffer_add_utf32(buffer: *mut hb_buffer_t, text: *const uint32_t, text_length: c_int, item_offset: c_uint, item_length: c_int);

pub fn RUST_hb_buffer_set_length(buffer: *mut hb_buffer_t, length: c_uint) -> hb_bool_t;

pub fn RUST_hb_buffer_get_length(buffer: *mut hb_buffer_t) -> c_uint;

pub fn RUST_hb_buffer_get_glyph_infos(buffer: *mut hb_buffer_t, length: *mut c_uint) -> *mut hb_glyph_info_t;

pub fn RUST_hb_buffer_get_glyph_positions(buffer: *mut hb_buffer_t, length: *mut c_uint) -> *mut hb_glyph_position_t;

pub fn RUST_hb_face_create(blob: *mut hb_blob_t, index: c_uint) -> *mut hb_face_t;

pub fn RUST_hb_face_create_for_tables(reference_table: hb_reference_table_func_t, user_data: *mut c_void, destroy: Option<hb_destroy_func_t>) -> *mut hb_face_t;

pub fn RUST_hb_face_get_empty() -> *mut hb_face_t;

pub fn RUST_hb_face_reference(face: *mut hb_face_t) -> *mut hb_face_t;

pub fn RUST_hb_face_destroy(face: *mut hb_face_t);

pub fn RUST_hb_face_set_user_data(face: *mut hb_face_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_face_get_user_data(face: *mut hb_face_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_face_make_immutable(face: *mut hb_face_t);

pub fn RUST_hb_face_is_immutable(face: *mut hb_face_t) -> hb_bool_t;

pub fn RUST_hb_face_reference_table(face: *mut hb_face_t, tag: hb_tag_t) -> *mut hb_blob_t;

pub fn RUST_hb_face_reference_blob(face: *mut hb_face_t) -> *mut hb_blob_t;

pub fn RUST_hb_face_set_index(face: *mut hb_face_t, index: c_uint);

pub fn RUST_hb_face_get_index(face: *mut hb_face_t) -> c_uint;

pub fn RUST_hb_face_set_upem(face: *mut hb_face_t, upem: c_uint);

pub fn RUST_hb_face_get_upem(face: *mut hb_face_t) -> c_uint;

pub fn RUST_hb_font_funcs_create() -> *mut hb_font_funcs_t;

pub fn RUST_hb_font_funcs_get_empty() -> *mut hb_font_funcs_t;

pub fn RUST_hb_font_funcs_reference(ffuncs: *mut hb_font_funcs_t) -> *mut hb_font_funcs_t;

pub fn RUST_hb_font_funcs_destroy(ffuncs: *mut hb_font_funcs_t);

pub fn RUST_hb_font_funcs_set_user_data(ffuncs: *mut hb_font_funcs_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_font_funcs_get_user_data(ffuncs: *mut hb_font_funcs_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_font_funcs_make_immutable(ffuncs: *mut hb_font_funcs_t);

pub fn RUST_hb_font_funcs_is_immutable(ffuncs: *mut hb_font_funcs_t) -> hb_bool_t;

pub fn RUST_hb_font_funcs_set_glyph_func(ffuncs: *mut hb_font_funcs_t, glyph_func: hb_font_get_glyph_func_t, user_data: *mut c_void, destroy: Option<hb_destroy_func_t>);

pub fn RUST_hb_font_funcs_set_glyph_h_advance_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_h_advance_func_t, user_data: *mut c_void, destroy: Option<hb_destroy_func_t>);

pub fn RUST_hb_font_funcs_set_glyph_v_advance_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_v_advance_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_funcs_set_glyph_h_origin_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_h_origin_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_funcs_set_glyph_v_origin_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_v_origin_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_funcs_set_glyph_h_kerning_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_h_kerning_func_t, user_data: *mut c_void, destroy: *mut c_void);

pub fn RUST_hb_font_funcs_set_glyph_v_kerning_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_v_kerning_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_funcs_set_glyph_extents_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_extents_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_funcs_set_glyph_contour_point_func(ffuncs: *mut hb_font_funcs_t, func: hb_font_get_glyph_contour_point_func_t, user_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_get_glyph(font: *mut hb_font_t, unicode: hb_codepoint_t, variation_selector: hb_codepoint_t, glyph: *mut hb_codepoint_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_h_advance(font: *mut hb_font_t, glyph: hb_codepoint_t) -> hb_position_t;

pub fn RUST_hb_font_get_glyph_v_advance(font: *mut hb_font_t, glyph: hb_codepoint_t) -> hb_position_t;

pub fn RUST_hb_font_get_glyph_h_origin(font: *mut hb_font_t, glyph: hb_codepoint_t, x: *mut hb_position_t, y: *mut hb_position_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_v_origin(font: *mut hb_font_t, glyph: hb_codepoint_t, x: *mut hb_position_t, y: *mut hb_position_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_h_kerning(font: *mut hb_font_t, left_glyph: hb_codepoint_t, right_glyph: hb_codepoint_t) -> hb_position_t;

pub fn RUST_hb_font_get_glyph_v_kerning(font: *mut hb_font_t, top_glyph: hb_codepoint_t, bottom_glyph: hb_codepoint_t) -> hb_position_t;

pub fn RUST_hb_font_get_glyph_extents(font: *mut hb_font_t, glyph: hb_codepoint_t, extents: *mut hb_glyph_extents_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_contour_point(font: *mut hb_font_t, glyph: hb_codepoint_t, point_index: c_uint, x: *mut hb_position_t, y: *mut hb_position_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_advance_for_direction(font: *mut hb_font_t, glyph: hb_codepoint_t, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t);

pub fn RUST_hb_font_get_glyph_origin_for_direction(font: *mut hb_font_t, glyph: hb_codepoint_t, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t);

pub fn RUST_hb_font_add_glyph_origin_for_direction(font: *mut hb_font_t, glyph: hb_codepoint_t, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t);

pub fn RUST_hb_font_subtract_glyph_origin_for_direction(font: *mut hb_font_t, glyph: hb_codepoint_t, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t);

pub fn RUST_hb_font_get_glyph_kerning_for_direction(font: *mut hb_font_t, first_glyph: hb_codepoint_t, second_glyph: hb_codepoint_t, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t);

pub fn RUST_hb_font_get_glyph_extents_for_origin(font: *mut hb_font_t, glyph: hb_codepoint_t, direction: hb_direction_t, extents: *mut hb_glyph_extents_t) -> hb_bool_t;

pub fn RUST_hb_font_get_glyph_contour_point_for_origin(font: *mut hb_font_t, glyph: hb_codepoint_t, point_index: c_uint, direction: hb_direction_t, x: *mut hb_position_t, y: *mut hb_position_t) -> hb_bool_t;

pub fn RUST_hb_font_create(face: *mut hb_face_t) -> *mut hb_font_t;

pub fn RUST_hb_font_create_sub_font(parent: *mut hb_font_t) -> *mut hb_font_t;

pub fn RUST_hb_font_get_empty() -> *mut hb_font_t;

pub fn RUST_hb_font_reference(font: *mut hb_font_t) -> *mut hb_font_t;

pub fn RUST_hb_font_destroy(font: *mut hb_font_t);

pub fn RUST_hb_font_set_user_data(font: *mut hb_font_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_font_get_user_data(font: *mut hb_font_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_font_make_immutable(font: *mut hb_font_t);

pub fn RUST_hb_font_is_immutable(font: *mut hb_font_t) -> hb_bool_t;

pub fn RUST_hb_font_get_parent(font: *mut hb_font_t) -> *mut hb_font_t;

pub fn RUST_hb_font_get_face(font: *mut hb_font_t) -> *mut hb_face_t;

pub fn RUST_hb_font_set_funcs(font: *mut hb_font_t, klass: *mut hb_font_funcs_t, font_data: *mut c_void, destroy: Option<hb_destroy_func_t>);

pub fn RUST_hb_font_set_funcs_data(font: *mut hb_font_t, font_data: *mut c_void, destroy: hb_destroy_func_t);

pub fn RUST_hb_font_set_scale(font: *mut hb_font_t, x_scale: c_int, y_scale: c_int);

pub fn RUST_hb_font_get_scale(font: *mut hb_font_t, x_scale: *mut c_int, y_scale: *mut c_int);

pub fn RUST_hb_font_set_ppem(font: *mut hb_font_t, x_ppem: c_uint, y_ppem: c_uint);

pub fn RUST_hb_font_get_ppem(font: *mut hb_font_t, x_ppem: *mut c_uint, y_ppem: *mut c_uint);

pub fn RUST_hb_set_create() -> *mut hb_set_t;

pub fn RUST_hb_set_get_empty() -> *mut hb_set_t;

pub fn RUST_hb_set_reference(set: *mut hb_set_t) -> *mut hb_set_t;

pub fn RUST_hb_set_destroy(set: *mut hb_set_t);

pub fn RUST_hb_set_set_user_data(set: *mut hb_set_t, key: *mut hb_user_data_key_t, data: *mut c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;

pub fn RUST_hb_set_get_user_data(set: *mut hb_set_t, key: *mut hb_user_data_key_t) -> *mut c_void;

pub fn RUST_hb_set_allocation_successful(set: *mut hb_set_t) -> hb_bool_t;

pub fn RUST_hb_set_clear(set: *mut hb_set_t);

pub fn RUST_hb_set_empty(set: *mut hb_set_t) -> hb_bool_t;

pub fn RUST_hb_set_has(set: *mut hb_set_t, codepoint: hb_codepoint_t) -> hb_bool_t;

pub fn RUST_hb_set_add(set: *mut hb_set_t, codepoint: hb_codepoint_t);

pub fn RUST_hb_set_del(set: *mut hb_set_t, codepoint: hb_codepoint_t);

pub fn RUST_hb_set_equal(set: *mut hb_set_t, other: *mut hb_set_t) -> hb_bool_t;

pub fn RUST_hb_set_set(set: *mut hb_set_t, other: *mut hb_set_t);

pub fn RUST_hb_set_union(set: *mut hb_set_t, other: *mut hb_set_t);

pub fn RUST_hb_set_intersect(set: *mut hb_set_t, other: *mut hb_set_t);

pub fn RUST_hb_set_subtract(set: *mut hb_set_t, other: *mut hb_set_t);

pub fn RUST_hb_set_min(set: *mut hb_set_t) -> hb_codepoint_t;

pub fn RUST_hb_set_max(set: *mut hb_set_t) -> hb_codepoint_t;

pub fn RUST_hb_shape(font: *mut hb_font_t, buffer: *mut hb_buffer_t, features: *mut hb_feature_t, num_features: c_uint);

pub fn RUST_hb_shape_full(font: *mut hb_font_t, buffer: *mut hb_buffer_t, features: *mut hb_feature_t, num_features: c_uint, shaper_list: *mut *mut c_char) -> hb_bool_t;

pub fn RUST_hb_shape_list_shapers() -> *mut *mut c_char;

pub fn RUST_hb_version(major: *mut c_uint, minor: *mut c_uint, micro: *mut c_uint);

pub fn RUST_hb_version_string() -> *mut c_char;

pub fn RUST_hb_version_check(major: c_uint, minor: c_uint, micro: c_uint) -> hb_bool_t;

}
