# lymui-c

A thin C ABI (`cdylib`) wrapper over the [lymui](https://github.com/shigedangao/lymui)
color-conversion library. `build.rs` regenerates `bindings.h` via `cbindgen` on every build.

## Commands

```sh
cargo build                 # build the cdylib + regenerate bindings.h
cargo test                  # Rust unit tests (src/lib.rs `mod tests`)
cargo clippy --all-features # lint (CI runs this)
cargo fmt --check           # formatting gate (CI runs this)
make ctest                  # build the cdylib, then compile & run the C tests
```

`make ctest` links `tests/c/lib_test.c` against `target/debug/liblymui_c` and runs it
with the platform's library-path env var (`DYLD_LIBRARY_PATH` on macOS,
`LD_LIBRARY_PATH` on Linux). It exits non-zero if any `CHECK` fails.

## FFI contract (read before touching the C boundary)

- **Input shape:** for `ColorMapping::Rgb` the input pointer is read as a raw `uint8_t[3]`
  (`from_raw_parts(ptr as *mut u8, 3)` in `mapping.rs`) — pass a flat `u8[3]`, never a
  wrapper struct or a wider element type. Other color spaces read `[f64; N]` (or a `u8`
  for ANSI, a C string for Hex); see `get_rgb_from_color_space_rgb`.
- **Ownership:** every pointer returned by `get_*` is owned by Rust and must be released by
  its matching `drop_*` — `drop_color` (by value), `drop_generator`, `drop_grayscale`.
  Never `free()` them directly.
- **Failure:** conversions return `NULL` on error (e.g. `get_generator` with a factor
  outside `[0, 1]`).

## Tests

- Rust unit tests live in `src/lib.rs` under `#[cfg(test)] mod tests`.
- C tests live in `tests/c/lib_test.c` (zero-dependency `CHECK` macro; cargo ignores
  non-`.rs` files under `tests/`). Assertions reuse known-good values from the Rust tests.
