set -ex
cd harfbuzz && \
git fetch -t && \
git checkout 4.3.0 && \
cd - && \
bindgen --no-prepend-enum-name \
    --whitelist-function hb_.\* \
    --whitelist-type hb_.\* wrapper.h | \
    grep -Pv "^pub type __u?int(8|16|32)_t = " | \
    sed 's/ f32/ ::std::os::raw::c_float/' | \
    sed 's/u32_/u32/' | \
    sed 's/i32_/i32/' | \
    sed 's/u16_/u16/' | \
    sed 's/i16_/i16/' | \
    sed 's/u8_/u8/' | 
    sed 's/i8_/i8/' >src/bindings.rs
