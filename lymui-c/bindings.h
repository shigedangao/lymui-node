#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum GrayscaleKind {
  Lightness,
  Average,
  Luminosity,
  BT709,
  BT2100,
} GrayscaleKind;

typedef enum RgbKind {
  Cymk,
  Hex,
  Hsl,
  Hsv,
  Hwb,
  Rgb,
  YCbCr,
  Yuv,
  Ansi16,
  Ansi256,
} RgbKind;

typedef enum XyzKind {
  Xyz,
  Argb,
  Hcl,
  Hlab,
  Lab,
  LchLab,
  Lchuv,
  Luv,
  Oklab,
  Oklch,
  REC709,
  REC2020,
  REC2100,
  SRGB,
  Xyy,
  Rgb,
} XyzKind;

typedef enum XyzLight {
  D65,
  D50,
  Adobe,
} XyzLight;

typedef struct ColorSlice {
  uintptr_t len;
  const double *ptr;
} ColorSlice;

typedef struct AnyColor {
  const char *hex;
  struct ColorSlice *slice;
} AnyColor;

typedef struct RgbMixture {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} RgbMixture;

typedef struct ColorMixture {
  uintptr_t len;
  const struct RgbMixture *ptr;
} ColorMixture;

/**
 * Convert a color space that can be converted as an RGB
 *
 * # Arguments
 *
 * * `color` - *mut c_void
 * * `from` - Kind
 * * `target` - Kind
 */
struct AnyColor *convert_color_from_rgb_compatible_color(void *color,
                                                         enum RgbKind from,
                                                         enum RgbKind target);

/**
 * Convert a color space that can be represented as an Xyz
 *
 * # Arguments
 *
 * * `color` - *mut c_void
 * * `from` - XyzKind
 * * `to` - XyzKind
 * * `light` - XyzLight
 */
struct AnyColor *convert_color_from_xyz_compatible_color(void *color,
                                                         enum XyzKind from,
                                                         enum XyzKind to,
                                                         enum XyzLight light);

/**
 * Convert an rgb compatible color to a grayscale
 *
 * # Arguments
 *
 * * `rgb_compat_color` - *mut c_void
 * * `from` - RgbKind
 * * `grayscale_light_kind` - GrayscaleKind
 */
uint8_t get_grayscale(void *rgb_compat_color,
                      enum RgbKind from,
                      enum GrayscaleKind grayscale_light_kind);

/**
 * Generate shade or tint
 *
 * # Arguments
 *
 * * `rgb_compat_color` - *mut c_void
 * * `from ` - RgbKind
 * * `factor` - f64
 * * `is_shade` - bool
 */
struct ColorMixture *generate_shade_tint(void *rgb_compat_color,
                                         enum RgbKind from,
                                         double factor,
                                         bool is_shade);

/**
 * Drop a color that has been allocated by lymui
 *
 * # Arguments
 *
 * * `ptr` - *mut AnyColor
 *
 * # Safety
 *
 * This method should be called with a valid object that has been allocated with Rust
 * /!\ Never use the free method from C as the pointer is allocated by Rust
 */
void drop_any_color(struct AnyColor *ptr);

/**
 * Drop a color mixture (shade or tint)
 *
 * # Arguments
 *
 * * `ptr` - *mut ColorMixture
 *
 * # Safety
 *
 * This method should be called with a valid object that has been allocated with Rust
 * /!\ Never use the free method from C as the pointer is allocated by Rust
 */
void drop_color_mixture(struct ColorMixture *ptr);
