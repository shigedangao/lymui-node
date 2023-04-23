use crate::binding::{
    color::AnyColor,
    generator::ColorMixture,
    kind::grayscale::{self, GrayscaleKind},
    kind::rgb::RgbKind,
    kind::xyz::XyzKind,
    kind::xyz::XyzLight,
};
use std::ffi::c_void;
use lymui::xyz::Kind as LightKind;

pub mod binding;

/// Convert a color space that can be converted as an RGB
///
/// # Arguments
///
/// * `color` - *mut c_void
/// * `from` - Kind
/// * `target` - Kind
#[no_mangle]
pub extern "C" fn convert_color_from_rgb_compatible_color(
    color: *mut c_void,
    from: RgbKind,
    target: RgbKind,
) -> *mut AnyColor {
    let rgb = match from.as_rgb(color) {
        Ok(rgb) => rgb,
        Err(_) => return std::ptr::null_mut(),
    };

    let color = match AnyColor::from_rgb(rgb, target) {
        Ok(res) => res,
        Err(_) => return std::ptr::null_mut(),
    };

    Box::into_raw(Box::new(color))
}

/// Convert a color space that can be represented as an Xyz
///
/// # Arguments
///
/// * `color` - *mut c_void
/// * `from` - XyzKind
/// * `to` - XyzKind
/// * `light` - XyzLight
#[no_mangle]
pub extern "C" fn convert_color_from_xyz_compatible_color(
    color: *mut c_void,
    from: XyzKind,
    to: XyzKind,
    light: XyzLight,
) -> *mut AnyColor {
    let xyz_light_kind = match light {
        XyzLight::D65 => LightKind::D65,
        XyzLight::D50 => LightKind::D50,
        _ => LightKind::Adobe,
    };

    let xyz = match from.as_xyz(color, xyz_light_kind) {
        Ok(res) => res,
        Err(_) => return std::ptr::null_mut(),
    };

    let color = match AnyColor::from_xyz(xyz, to, xyz_light_kind) {
        Ok(res) => res,
        Err(_) => return std::ptr::null_mut(),
    };

    Box::into_raw(Box::new(color))
}

/// Convert an rgb compatible color to a grayscale
///
/// # Arguments
///
/// * `rgb_compat_color` - *mut c_void
/// * `from` - RgbKind
/// * `grayscale_light_kind` - GrayscaleKind
#[no_mangle]
pub extern "C" fn get_grayscale(
    rgb_compat_color: *mut c_void,
    from: RgbKind,
    grayscale_light_kind: GrayscaleKind,
) -> u8 {
    let rgb = match from.as_rgb(rgb_compat_color) {
        Ok(rgb) => rgb,
        Err(_) => return 0,
    };

    grayscale::get_grayscale_from_rgb(rgb, grayscale_light_kind)
}

/// Generate shade or tint
/// 
/// # Arguments
/// 
/// * `rgb_compat_color` - *mut c_void
/// * `from ` - RgbKind
/// * `factor` - f64
/// * `is_shade` - bool
#[no_mangle]
pub extern "C" fn generate_shade_tint(
    rgb_compat_color: *mut c_void,
    from: RgbKind,
    factor: f64,
    is_shade: bool,
) -> *mut ColorMixture {
    let rgb = match from.as_rgb(rgb_compat_color) {
        Ok(rgb) => rgb,
        Err(_) => return std::ptr::null_mut(),
    };

    match ColorMixture::new(rgb, factor, is_shade) {
        Ok(res) => Box::into_raw(Box::new(res)),
        Err(_) => std::ptr::null_mut()
    }
}

/// Drop a color that has been allocated by lymui
///
/// # Arguments
///
/// * `ptr` - *mut AnyColor
///
/// # Safety
///
/// This method should be called with a valid object that has been allocated with Rust
/// /!\ Never use the free method from C as the pointer is allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn drop_any_color(ptr: *mut AnyColor) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let color = Box::from_raw(ptr);
        color.drop_inner_fields();

        drop(color);
    }
}

/// Drop a color mixture (shade or tint)
/// 
/// # Arguments
/// 
/// * `ptr` - *mut ColorMixture
/// 
/// # Safety 
/// 
/// This method should be called with a valid object that has been allocated with Rust
/// /!\ Never use the free method from C as the pointer is allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn drop_color_mixture(ptr: *mut ColorMixture) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let color = Box::from_raw(ptr);
        drop(color);
    }
}