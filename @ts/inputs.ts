export interface Color {}

/**
 * Rgb interface use by the input Object
 */
export interface Rgb extends Color {
  r: number;
  g: number;
  b: number;
}

/**
 * Cymk interface use by the input Object (toRGB)
 */
export interface Cymk extends Color {
  c: number;
  y: number;
  m: number;
  k: number;
}

/**
 * Hsl interface use by the input Object (toRGB)
 */
export interface Hsl extends Color {
  h: number;
  s: number;
  l: number;
}

/**
 * Hsv interface use by the input Object (toRGB)
 */
export interface Hsv extends Color {
  h: number;
  s: number;
  v: number;
}

/**
 * Ycbcr interface use by the input Object (toRGB)
 */
export interface Ycbcr extends Color {
  y: number;
  cb: number;
  cr: number;
}

/**
 * Yuv interface use by the input Object (toRGB)
 */
export interface Yuv extends Color {
  y: number;
  u: number;
  v: number;
}

/**
 * Hwb interface use by the input Object (toRGB)
 */
export interface Hwb extends Color {
  h: number;
  w: number;
  b: number;
}

/**
 * Tsl interface use by the input Object (toRGB)
 */
export interface Tsl extends Color {
  t: number;
  s: number;
  l: number;
}

/**
 * Xyz interface use by the input Object
 */
export interface Xyz extends Color {
  x: number;
  y: number;
  z: number;
}

/**
 * Lab interface use by the input Object (toXYZ)
 */
export interface Lab extends Color {
  l: number;
  a: number;
  b: number;
}

/**
 * Lch interface use by the input Object (toXYZ)
 */
export interface Lch extends Color {
  l: number;
  c: number;
  h: number;
}

/**
 * Luv interface use by the input Object (toXYZ)
 */
export interface Luv extends Color {
  l: number;
  u: number;
  v: number;
}

/**
 * Xyy interface use by the input Object (toXYZ)
 */
export interface Xyy extends Color {
  x: number;
  y: number;
  Y: number;
}

/**
 * Argb interface use by the input Object (toXYZ)
 * Alias to RGB
 */
export interface Argb extends Rgb {};

/**
 * Srgb interface use by the input Object (toXYZ)
 * Alias to RGB
 */
export interface Srgb extends Rgb {};

/**
 * LchLab interface use by the input Object (toXYZ)
 * Alias to Lch
 */
export interface LchLab extends Lch {};

/**
 * Hex type
 */
export type Hex = string;
