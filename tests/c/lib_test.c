/*
 * C test suite for the lymui-c FFI surface.
 *
 * Exercises every function exposed in src/lib.rs against the debug cdylib and
 * asserts known-good values (mirrored from the Rust unit tests). Build & run
 * with `make ctest` from the repository root.
 */
#include "../../bindings.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static int tests = 0;
static int fails = 0;

#define CHECK(cond, msg)                \
    do {                                \
        tests++;                        \
        if (cond) {                     \
            printf("ok   - %s\n", msg); \
        } else {                        \
            printf("FAIL - %s\n", msg); \
            fails++;                    \
        }                               \
    } while (0)

/* get_color: vector output (RGB -> YCbCr). Values match the Rust unit test. */
static void test_get_color_vector(void) {
    uint8_t rgb[3] = {0, 100, 200};
    Color *ycbcr = get_color(rgb, Rgb, YCbCr, None);

    CHECK(ycbcr != NULL, "get_color RGB->YCbCr returns non-null");
    if (ycbcr == NULL) {
        return;
    }

    CHECK(ycbcr->data != NULL, "YCbCr result carries a data vector");
    CHECK(ycbcr->data->len == 3, "YCbCr vector has 3 components");
    CHECK(ycbcr->data->ptr[0] == 86.0, "YCbCr Y == 86");
    CHECK(ycbcr->data->ptr[1] == 186.0, "YCbCr Cb == 186");
    CHECK(ycbcr->data->ptr[2] == 77.0, "YCbCr Cr == 77");

    drop_color(*ycbcr);
}

/* get_color: hex string output (RGB -> Hex). lymui emits lowercase #rrggbb. */
static void test_get_color_hex(void) {
    uint8_t rgb[3] = {0, 100, 200};
    Color *hex = get_color(rgb, Rgb, Hex, None);

    CHECK(hex != NULL, "get_color RGB->Hex returns non-null");
    if (hex == NULL) {
        return;
    }

    CHECK(hex->hex != NULL, "Hex result carries a string");
    CHECK(hex->hex != NULL && strcmp(hex->hex, "#0064c8") == 0, "Hex string == #0064c8");

    drop_color(*hex);
}

/* get_grayscale: Lightness of pure red == (255 + 0) / 2 == 127. */
static void test_get_grayscale(void) {
    uint8_t rgb[3] = {255, 0, 0};
    uint8_t *gray = get_grayscale(rgb, Rgb, Lightness);

    CHECK(gray != NULL, "get_grayscale returns non-null");
    if (gray == NULL) {
        return;
    }

    CHECK(*gray == 127, "Lightness of pure red == 127");

    drop_grayscale(gray);
}

/* get_generator: a 0.5 factor walks f from 1.0 to 0.0 -> [original, half, black]. */
static void test_get_generator_shade(void) {
    uint8_t rgb[3] = {200, 100, 50};
    Generator *shade = get_generator(rgb, Rgb, 0.5, Shade);

    CHECK(shade != NULL, "get_generator Shade returns non-null");
    if (shade == NULL) {
        return;
    }

    CHECK(shade->len == 3, "Shade 0.5 yields 3 entries");
    CHECK(shade->generated[0].r == 200 && shade->generated[0].g == 100 &&
              shade->generated[0].b == 50,
          "shade[0] == original (200,100,50)");
    CHECK(shade->generated[1].r == 100 && shade->generated[1].g == 50 &&
              shade->generated[1].b == 25,
          "shade[1] == half (100,50,25)");
    CHECK(shade->generated[2].r == 0 && shade->generated[2].g == 0 &&
              shade->generated[2].b == 0,
          "shade[2] == black (0,0,0)");

    drop_generator(shade);
}

/* get_generator: factor must lie in [0, 1]; an out-of-range factor yields NULL. */
static void test_get_generator_invalid_factor(void) {
    uint8_t rgb[3] = {200, 100, 50};
    Generator *bad = get_generator(rgb, Rgb, 1.1, Shade);

    CHECK(bad == NULL, "get_generator with factor 1.1 returns NULL");
}

int main(void) {
    test_get_color_vector();
    test_get_color_hex();
    test_get_grayscale();
    test_get_generator_shade();
    test_get_generator_invalid_factor();

    printf("\n%d checks, %d failed\n", tests, fails);
    return fails ? 1 : 0;
}
