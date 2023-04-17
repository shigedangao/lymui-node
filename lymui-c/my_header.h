#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum RgbKind {
  Cymk,
  Hex,
  Hsl,
  Hsv,
  Hwb,
  Rgb,
  YCbCr,
  Yuv,
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
  RgbCompat,
} XyzKind;

typedef enum XyzLight {
  D65,
  D50,
  Adobe,
} XyzLight;

typedef struct ColorSlice {
  size_t len;
  const double *ptr;
} ColorSlice;

typedef struct AnyColor {
  const char *hex;
  struct ColorSlice *slice;
} AnyColor;

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
