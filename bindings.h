#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Represents the color mapping to use when converting a color.
 */
typedef enum ColorMapping {
  Rgb,
  Cymk,
  Hex,
  Hsl,
  Hsv,
  Hwb,
  YCbCr,
  Yuv,
  Ansi16,
  Ansi256,
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
} ColorMapping;

/**
 * Represents the lumens type for color calculations.
 */
typedef enum Lumens {
  None,
  D50,
  D65,
  Adobe,
} Lumens;

/**
 * Represents the grayscale kind for color calculations.
 */
typedef enum Grayscale {
  Lightness,
  Average,
  Luminosity,
  BT709,
  BT2100,
} Grayscale;

/**
 * Represents the kind of generator used for color calculations.
 */
typedef enum GeneratorKind {
  Shade,
  Tint,
} GeneratorKind;

typedef struct ColorResult {
  uintptr_t len;
  const double *ptr;
} ColorResult;

/**
 * Represents a color result, including the color vector and optional hex string.
 */
typedef struct Color {
  struct ColorResult *data;
  char *hex;
} Color;

/**
 * Represents an RGB generator result, including the generated color values.
 */
typedef struct RgbGenerator {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} RgbGenerator;

/**
 * Represents a generator result, including the generated color values and length.
 * The generated colors are send as *mut RgbGenerator which represents a pointer to a vector of RgbGenerator results.
 */
typedef struct Generator {
  struct RgbGenerator *generated;
  uintptr_t len;
} Generator;

/**
 * Retrieves a color result based on the specified color mapping and lumens type.
 *
 * # Arguments
 *
 * * `color` - A pointer to the color result to be populated.
 * * `from` - The color mapping to convert from.
 * * `target` - The color mapping to convert to.
 * * `lumens` - The lumens type for color calculations.
 *
 * # Returns
 *
 * A pointer to the populated `Color` struct.
 */
struct Color *get_color(void *color,
                        enum ColorMapping from,
                        enum ColorMapping target,
                        enum Lumens lumens);

/**
 * Returns a pointer to the grayscale value of the given color data.
 *
 * # Arguments
 *
 * * `data` - A pointer to the color data.
 * * `from` - The color mapping to use.
 * * `kind` - The grayscale kind to use.
 *
 * # Returns
 *
 * A pointer to the grayscale value of the given color data.
 */
uint8_t *get_grayscale(void *data, enum ColorMapping from, enum Grayscale kind);

/**
 * Get a generator for the specified color mapping, factor, and kind.
 *
 * # Arguments
 *
 * * `data` - A pointer to the color mapping data.
 * * `from` - The color mapping to use.
 * * `factor` - The factor to use for the generator.
 * * `kind` - The kind of generator to create.
 *
 * # Returns
 *
 * A raw pointer to the generator, or `null` if the generator could not be created.
 */
struct Generator *get_generator(void *data,
                                enum ColorMapping from,
                                double factor,
                                enum GeneratorKind kind);

/**
 * Frees the memory allocated for the given color.
 * /!\ This function must be called when the color is no longer needed. As rust owns
 * the memory for the color, this function should be called to avoid memory leaks.
 *
 * # Arguments
 *
 * * `color` - The color to free.
 */
void drop_color(struct Color color);
