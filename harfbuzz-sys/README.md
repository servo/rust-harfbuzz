<div align="center">

# harfbuzz-sys

[![Build Status](https://github.com/servo/rust-harfbuzz/actions/workflows/main.yml/badge.svg)](https://github.com/servo/rust-harfbuzz/actions)
[![Crates.io](https://img.shields.io/crates/v/harfbuzz-sys.svg)](https://crates.io/crates/harfbuzz-sys)
[![Docs](https://docs.rs/harfbuzz-sys/badge.svg)](https://docs.rs/harfbuzz-sys)

</div>

[HarfBuzz](https://harfbuzz.github.io/) is a text shaping engine. It
solves the problem of selecting and positioning glyphs from a font
given a Unicode string.

This crate provides low-level bindings to the C API.

## Features

- `freetype` - Enables bindings to the FreeType font engine. (Enabled by default.)
- `coretext` - Enables bindings to the CoreText font engine. (Apple platforms only) (Enabled by default.)
- `directwrite` - Enables bindings to the DirectWrite font engine. (Windows only) (Enabled by default.)

- `bundled` - Use the bundled copy of the harfbuzz library rather than one installed on the system.

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <https://opensource.org/licenses/MIT>).

## Contribution

Contributions are welcome by pull request.