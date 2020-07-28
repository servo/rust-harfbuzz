wget https://github.com/harfbuzz/harfbuzz/archive/2.6.8.zip
rm -rf harfbuzz
unzip harfbuzz-2.6.8.zip
mv harfbuzz-2.6.8 harfbuzz
bindgen harfbuzz/src/hb.h >src/bindings.rs
