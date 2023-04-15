use crate::binding::{
    color::AnyColor, kind::rgb::RgbKind, kind::xyz::XyzKind, kind::xyz::XyzLight,
};
use libc::c_void;
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
