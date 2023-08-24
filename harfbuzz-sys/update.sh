wget https://github.com/harfbuzz/harfbuzz/releases/download/2.7.4/harfbuzz-2.7.4.tar.xz
rm -rf harfbuzz
tar xvf harfbuzz-2.7.4.tar.xz
rm harfbuzz-2.7.4.tar.xz
mv harfbuzz-2.7.4 harfbuzz
bindgen --use-core --no-prepend-enum-name --allowlist-function hb_.\* --allowlist-type hb_.\* wrapper.h | sed 's/ f32/ ::core::ffi::c_float/' | sed 's/u32_/u32/' | sed 's/i32_/i32/' | sed 's/u16_/u16/' | sed 's/i16_/i16/' | sed 's/u8_/u8/' | sed 's/i8_/i8/' >src/bindings.rs
