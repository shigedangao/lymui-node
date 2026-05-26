## Lymui-c 🍭

A thin C ABI wrapper over the [lymui](https://github.com/shigedangao/lymui) color conversion library. It compiles to a C-compatible shared library (`cdylib`) so you can consume the library from C (and any language with C FFI).

## Features

- Convert between a wide range of color spaces. See the list of supported color spaces [here](https://github.com/shigedangao/lymui#lymui-).
- Compute grayscale values using several strategies (Lightness, Average, Luminosity, BT709, BT2100).
- Generate shades and tints from a given color.
- Optional reference white selection (`D50`, `D65`, `Adobe`) for XYZ-based conversions.

## Build

Build the shared library and let `build.rs` regenerate the `bindings.h` header through `cbindgen`.

```sh
cargo build --release
```

This produces:

- `target/release/liblymui_c.{so,dylib,dll}` — the shared library.
- `bindings.h` — the C header (regenerated on every build).

If you want to regenerate the header manually:

```sh
cbindgen --config cbindgen.toml --crate lymui-c --output bindings.h --lang c
```

## Using it from C

A complete example lives in [`examples/rgb.c`](examples/rgb.c). Compile and run it against the debug build:

```sh
cd examples
gcc rgb.c -o test -llymui_c -L../target/debug
LD_LIBRARY_PATH=../target/debug ./test
```

On macOS replace `LD_LIBRARY_PATH` with `DYLD_LIBRARY_PATH`.

### Minimal example

```c
#include "../bindings.h"
#include <stdint.h>
#include <stdio.h>

int main(void) {
    uint8_t rgb[3] = {100, 0, 194};

    // RGB -> CYMK
    Color *cymk = get_color(rgb, Rgb, Cymk, None);
    printf("c: %f, y: %f, m: %f, k: %f\n",
        cymk->data->ptr[0], cymk->data->ptr[1],
        cymk->data->ptr[2], cymk->data->ptr[3]);

    // RGB -> Hex
    Color *hex = get_color(rgb, Rgb, Hex, None);
    printf("hex: %s\n", hex->hex);

    // Grayscale
    uint8_t *gray = get_grayscale(rgb, Rgb, Lightness);
    printf("grayscale: %u\n", *gray);

    // Shade generator with a 0.1 factor
    Generator *shade = get_generator(rgb, Rgb, 0.1, Shade);
    printf("shade[1] = (%d, %d, %d)\n",
        shade->generated[1].r, shade->generated[1].g, shade->generated[1].b);

    // Always release memory allocated by Rust
    drop_color(*cymk);
    drop_color(*hex);
    drop_grayscale(gray);
    drop_generator(shade);
    return 0;
}
```

## API overview

| Function | Description |
| --- | --- |
| `get_color(data, from, target, lumens)` | Convert `data` from the `from` color space to `target`. Returns a `Color*` whose `data->ptr` holds the component vector, or `hex` for hex output. |
| `get_grayscale(data, from, kind)` | Returns a `uint8_t*` grayscale value computed with the chosen strategy. |
| `get_generator(data, from, factor, kind)` | Returns a `Generator*` of `RgbGenerator` values (Shade or Tint). |
| `drop_color(color)` | Frees memory allocated by `get_color`. Must be called to avoid leaks. |
| `drop_generator(generator)` | Frees the `Generator*` allocated by `get_generator`. Must be called to avoid leaks. |
| `drop_grayscale(g_scale)` | Frees the `uint8_t*` allocated by `get_grayscale`. Must be called to avoid leaks. |

### Memory ownership

Any pointer returned by the library is owned by Rust. Release each result with its matching function — `drop_color` for `Color`, `drop_generator` for `Generator`, and `drop_grayscale` for the grayscale value; do not `free()` these pointers directly.

## License

See [LICENSE](LICENSE).
