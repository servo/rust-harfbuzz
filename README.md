# rust-harfbuzz

[![Build Status](https://github.com/servo/rust-harfbuzz/actions/workflows/main.yml/badge.svg)](https://github.com/servo/rust-harfbuzz/actions)

[Documentation](https://docs.rs/harfbuzz-sys)

Bindings to the HarfBuzz text shaping engine.

## Vendored Harfbuzz Sources

This repository contains a vendored copy of the `harfbuzz` sources
as a git submodule. If building with the `bundled` feature, be sure
to initialize submodules:

```
git clone --recursive https://github.com/servo/rust-harfbuzz.git
```

Or, after a clone:

```
git submodule update --init
```
