use crate::mapping::ColorMapping;
use lymui::generator::{GeneratorOps, shade::Shade, tint::Tint};
use lymui::rgb::FromRgb;
use std::ffi::{CString, c_char, c_double, c_void};

pub mod color;
pub mod mapping;

/// Represents a color result, including the color vector and optional hex string.
#[derive(Debug, Default)]
#[repr(C)]
pub struct Color {
    pub data: *mut color::ColorResult,
    pub hex: *mut c_char,
}

/// Represents an RGB generator result, including the generated color values.
#[derive(Debug)]
#[repr(C)]
pub struct RgbGenerator {
    r: u8,
    g: u8,
    b: u8,
}

/// Represents a generator result, including the generated color values and length.
/// The generated colors are send as *mut RgbGenerator which represents a pointer to a vector of RgbGenerator results.
#[derive(Debug)]
#[repr(C)]
pub struct Generator {
    pub generated: *const RgbGenerator,
    pub len: usize,
    cap: usize,
}

/// Represents the lumens type for color calculations.
#[derive(Debug, Default)]
#[repr(C)]
pub enum Lumens {
    #[default]
    None,
    D50,
    D65,
    Adobe,
}

/// Represents the grayscale kind for color calculations.
#[derive(Debug, Default)]
#[repr(C)]
pub enum Grayscale {
    #[default]
    Lightness,
    Average,
    Luminosity,
    BT709,
    BT2100,
}

/// Represents the kind of generator used for color calculations.
#[derive(Debug, Default)]
#[repr(C)]
pub enum GeneratorKind {
    #[default]
    Shade,
    Tint,
}

/// Retrieves a color result based on the specified color mapping and lumens type.
///
/// # Arguments
///
/// * `color` - A pointer to the color result to be populated.
/// * `from` - The color mapping to convert from.
/// * `target` - The color mapping to convert to.
/// * `lumens` - The lumens type for color calculations.
///
/// # Returns
///
/// A pointer to the populated `Color` struct.
#[unsafe(no_mangle)]
pub extern "C" fn get_color(
    color: *mut c_void,
    from: ColorMapping,
    target: ColorMapping,
    lumens: Lumens,
) -> *mut Color {
    let Ok(color) = color::get_color(color, from, target, lumens) else {
        return std::ptr::null_mut();
    };

    let mut res = Color::default();

    let (color, hex) = color;
    if let Some(color_vec) = color {
        res.data = Box::into_raw(Box::new(color_vec));
    }

    if let Some(hex_str) = hex {
        res.hex = hex_str as *mut c_char;
    }

    Box::into_raw(Box::new(res))
}

/// Returns a pointer to the grayscale value of the given color data.
///
/// # Arguments
///
/// * `data` - A pointer to the color data.
/// * `from` - The color mapping to use.
/// * `kind` - The grayscale kind to use.
///
/// # Returns
///
/// A pointer to the grayscale value of the given color data.
#[unsafe(no_mangle)]
pub extern "C" fn get_grayscale(data: *mut c_void, from: ColorMapping, kind: Grayscale) -> *mut u8 {
    let Ok(converted_rgb) = from.get_rgb_from_color_space_rgb(data, &Lumens::None) else {
        return std::ptr::null_mut();
    };

    let kind = match kind {
        Grayscale::Lightness => lymui::grayscale::Kind::Lightness,
        Grayscale::Average => lymui::grayscale::Kind::Average,
        Grayscale::Luminosity => lymui::grayscale::Kind::Luminosity,
        Grayscale::BT709 => lymui::grayscale::Kind::BT709,
        Grayscale::BT2100 => lymui::grayscale::Kind::BT2100,
    };

    let grayscale = lymui::grayscale::GrayScale::from_rgb(converted_rgb, kind);

    // Create a box so that the grayscale is stored on the heap allowing C to get the reference
    let grayscale_boxed = Box::new(grayscale.0);

    // Return a raw pointer to the grayscale value
    Box::into_raw(grayscale_boxed)
}

/// Get a generator for the specified color mapping, factor, and kind.
///
/// # Arguments
///
/// * `data` - A pointer to the color mapping data.
/// * `from` - The color mapping to use.
/// * `factor` - The factor to use for the generator.
/// * `kind` - The kind of generator to create.
///
/// # Safety
///
/// The `data` pointer must be valid and not null which represents an array of color e.g for the RGB: [1, 2, 3].
///
/// # Returns
///
/// A raw pointer to the generator, or `null` if the generator could not be created.
#[unsafe(no_mangle)]
pub extern "C" fn get_generator(
    data: *mut c_void,
    from: ColorMapping,
    factor: c_double,
    kind: GeneratorKind,
) -> *mut Generator {
    let Ok(converted_rgb) = from.get_rgb_from_color_space_rgb(data, &Lumens::None) else {
        return std::ptr::null_mut();
    };

    let res = match kind {
        GeneratorKind::Shade => Shade::compute(converted_rgb, factor).map(|v| v.0),
        GeneratorKind::Tint => Tint::compute(converted_rgb, factor).map(|v| v.0),
    };

    let Ok(generated) = res else {
        return std::ptr::null_mut();
    };

    let mut generated_kind = Vec::new();
    for shade in generated {
        generated_kind.push(RgbGenerator {
            r: shade.r,
            g: shade.g,
            b: shade.b,
        });
    }

    let generated_boxed = Box::new(Generator {
        generated: generated_kind.as_ptr(),
        len: generated_kind.len(),
        cap: generated_kind.capacity(),
    });

    // Prevent rust deallocating the vector
    std::mem::forget(generated_kind);

    Box::into_raw(generated_boxed)
}

/// Frees the memory allocated for the given color.
/// /!\ This function must be called when the color is no longer needed. As rust owns
/// the memory for the color, this function should be called to avoid memory leaks.
///
/// # Arguments
///
/// * `color` - The color to free.
///
/// # Safety
///
/// The `data` pointer must be valid and not null.
/// The `hex` pointer must be valid and not null.
#[unsafe(no_mangle)]
pub extern "C" fn drop_color(color: Color) {
    if !color.data.is_null() {
        unsafe {
            color::drop_color_result(color.data);
        }
    }

    if !color.hex.is_null() {
        unsafe {
            let hex = CString::from_raw(color.hex);
            drop(hex);
        }
    }
}

/// Frees the memory allocated for the given generator.
///
/// # Safety
///
/// * `generator` - Must be a valid pointer to a `Generator`.
///
/// This function must be called when the generator is no longer needed. As rust owns
/// the memory for the generator, this function should be called to avoid memory leaks.
///
/// # Arguments
///
/// * `generator` - The generator to free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_generator(generator: *mut Generator) {
    if generator.is_null() {
        return;
    }

    let owned_generator = unsafe { Box::from_raw(generator) };
    // Reconstruct the vector from the boxed generator
    let v = unsafe {
        Vec::from_raw_parts(
            owned_generator.generated as *mut RgbGenerator,
            owned_generator.len,
            owned_generator.cap,
        )
    };

    // Drop the vector to free the memory
    drop(v);
}

/// Frees the memory allocated for the given grayscale scale.
///
/// # Safety
///
/// * `g_scale` - Must be a valid pointer to a `u8` slice.
///
/// This function must be called when the grayscale scale is no longer needed. As rust owns
/// the memory for the grayscale scale, this function should be called to avoid memory leaks.
///
/// # Arguments
///
/// * `g_scale` - The grayscale scale to free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_grayscale(g_scale: *mut u8) {
    if g_scale.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(g_scale));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_color() {
        let color_slice: [u8; 3] = [0, 100, 200];
        let color = get_color(
            color_slice.as_ptr() as *mut c_void,
            ColorMapping::Rgb,
            ColorMapping::YCbCr,
            Lumens::None,
        );

        assert!(!color.is_null());

        // Reconstruct the vec which should contains the color data as a vector of f64
        let boxed_color = unsafe { Box::from_raw(color) };
        let data = unsafe { Box::from_raw(boxed_color.data) };
        let vv = unsafe { Vec::from_raw_parts(data.ptr as *mut f64, data.len, data.cap) };

        // Verify the vec contains the expected color data
        assert_eq!(vv.len(), 3);
        assert_eq!(vv[0], 86.);
        assert_eq!(vv[1], 186.);
        assert_eq!(vv[2], 77.);
    }

    #[test]
    fn expect_to_get_hex() {
        let color_slice: [u8; 3] = [0, 100, 200];
        let color = get_color(
            color_slice.as_ptr() as *mut c_void,
            ColorMapping::Rgb,
            ColorMapping::Hex,
            Lumens::None,
        );

        assert!(!color.is_null());

        let boxed_color = unsafe { Box::from_raw(color) };
        let hex = unsafe { CString::from_raw(boxed_color.hex) };
        assert_eq!(hex.to_string_lossy(), "#0064c8");
    }

    #[test]
    fn expect_to_get_grayscale() {
        let color_vec: [u8; 3] = [255, 0, 0];
        let g_scale = get_grayscale(
            color_vec.as_ptr() as *mut c_void,
            ColorMapping::Rgb,
            Grayscale::Lightness,
        );

        // Leaking in order to access the result.
        assert_eq!(unsafe { *g_scale }, 127);

        assert!(!g_scale.is_null());
        unsafe { drop_grayscale(g_scale) };
    }

    #[test]
    fn expect_to_get_shade() {
        let color_slice: [u8; 3] = [200, 100, 50];
        let generator = get_generator(
            color_slice.as_ptr() as *mut c_void,
            ColorMapping::Rgb,
            0.5,
            GeneratorKind::Shade,
        );
        assert!(!generator.is_null());

        unsafe {
            let output = &*generator;
            // With a 0.5 factor the shades run from f=1.0 down to 0.0: [original, half, black].
            assert_eq!(output.len, 3);

            let shades = std::slice::from_raw_parts(output.generated, output.len);
            assert_eq!((shades[0].r, shades[0].g, shades[0].b), (200, 100, 50));
            assert_eq!((shades[1].r, shades[1].g, shades[1].b), (100, 50, 25));
            assert_eq!((shades[2].r, shades[2].g, shades[2].b), (0, 0, 0));

            drop_generator(generator);
        }
    }
}
