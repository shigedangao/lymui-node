use super::Lumens;
use crate::mapping::{ColorConversionResult, ColorMapping};
use anyhow::Result;
use lymui::xyz::Kind;
use std::ffi::{CString, c_char, c_void};

#[derive(Debug)]
#[repr(C)]
pub struct ColorResult {
    pub cap: usize,
    pub len: usize,
    pub ptr: *const f64,
}

/// Get the color from the given data using the specified color space, target mapping, and lumens type.
///
/// # Arguments
///
/// * `data` - The raw color data to convert.
/// * `from` - The color space to convert from.
/// * `target` - The color mapping to convert to.
/// * `lumens` - The lumens type for color calculations.
///
/// # Errors
///
/// Returns an error if the color conversion fails.
///
/// # Returns
///
/// A pointer to the populated `Color` struct.
pub fn get_color(
    data: *mut c_void,
    from: ColorMapping,
    target: ColorMapping,
    lumens: Lumens,
) -> Result<(Option<ColorResult>, Option<*const c_char>)> {
    // Convert the raw color data to RGB using the specified color space.
    let converted_rgb = from.get_rgb_from_color_space_rgb(data)?;

    // Convert the RGB color to the target color space using the specified lumens type.
    let lib_lumens = match lumens {
        Lumens::None => None,
        Lumens::D65 => Some(Kind::D65),
        Lumens::D50 => Some(Kind::D50),
        Lumens::Adobe => Some(Kind::Adobe),
    };

    // Convert the RGB color to the target color space using the specified lumens type.
    let color_slice = target.wrap_lymui_convert_space(converted_rgb, lib_lumens);

    let Ok(color_slice) = color_slice else {
        return Err(anyhow::anyhow!("Failed to convert color space"));
    };

    match color_slice {
        ColorConversionResult::Color(color_slice) => {
            let cap = color_slice.capacity();
            let boxed = color_slice.into_boxed_slice();
            let len = boxed.len();
            let ptr = Box::into_raw(boxed) as *const f64;

            Ok((Some(ColorResult { cap, len, ptr }), None))
        }
        ColorConversionResult::Hex(hex) => {
            let hex_c_str = CString::new(hex)?;

            Ok((None, Some(hex_c_str.into_raw())))
        }
    }
}

/// Frees memory allocated by `get_color` when `target` is `Hex`.
///
/// # Arguments
///
/// * `color` - A pointer to the `ColorResult` to be freed.
pub unsafe fn drop_color_result(color: *mut ColorResult) {
    if !color.is_null() {
        let boxed_color = unsafe { Box::from_raw(color) };
        if !boxed_color.ptr.is_null() {
            // reconstruct the fat pointer using the boxed color's length and pointer
            let sliced_ptr =
                std::ptr::slice_from_raw_parts_mut(boxed_color.ptr as *mut f64, boxed_color.len);

            // convert the sliced pointer back to a boxed slice and drop it
            let slice = unsafe { Box::from_raw(sliced_ptr) };

            // drop the boxed slice
            drop(slice);
            // drop the boxed color
            drop(boxed_color);
        }
    }
}
