use super::kind::{rgb::RgbKind, xyz::XyzKind};
use crate::LightKind;
use anyhow::Result;
use lymui::prelude::*;
use lymui::rgb::FromRgb;
use lymui::util::AsVec;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug)]
#[repr(C)]
pub struct AnyColor {
    pub hex: *const c_char,
    pub slice: *mut ColorSlice,
}

#[derive(Debug)]
#[repr(C)]
pub struct ColorSlice {
    pub len: usize,
    pub ptr: *const f64,
}

impl Default for AnyColor {
    fn default() -> Self {
        AnyColor {
            hex: std::ptr::null(),
            slice: std::ptr::null_mut(),
        }
    }
}

impl AnyColor {
    /// Create a new color based on the target type
    ///
    /// # Arguments
    ///
    /// * `rgb` - Rgb
    /// * `tgt` - RgbKind
    pub fn from_rgb(rgb: Rgb, tgt: RgbKind) -> Result<Self> {
        let mut color = AnyColor::default();

        if tgt == RgbKind::Hex {
            let hex_c_str = CString::new(Hex::from(rgb).0)?;
            color.hex = hex_c_str.into_raw();

            return Ok(color);
        }

        let vec = match tgt {
            RgbKind::Cymk => Cymk::from(rgb).as_vec(),
            RgbKind::Hsl => Hsl::from(rgb).as_vec(),
            RgbKind::Hsv => Hsv::from(rgb).as_vec(),
            RgbKind::Hwb => Hwb::from(rgb).as_vec(),
            RgbKind::Rgb => rgb.as_vec(),
            RgbKind::YCbCr => Ycbcr::from(rgb).as_vec(),
            RgbKind::Yuv => Yuv::from(rgb).as_vec(),
            RgbKind::Ansi16 => vec![Ansi::from_rgb(rgb, AnsiKind::C16).0 as f64],
            RgbKind::Ansi256 => vec![Ansi::from_rgb(rgb, AnsiKind::C256).0 as f64],
            _ => Vec::new(),
        };

        color.get_color_slice(vec);

        Ok(color)
    }

    /// Create a new color from Xyz for a type of light
    ///
    /// # Arguments
    ///
    /// * `xyz` - Xyz
    /// * `to` - XyzKind
    /// * `light` - LightKind
    pub fn from_xyz(xyz: Xyz, to: XyzKind, light: LightKind) -> Result<Self> {
        let mut color = AnyColor::default();

        let vec = match to {
            XyzKind::Xyz => xyz.as_vec(),
            XyzKind::Argb => Argb::from(xyz).as_vec(),
            XyzKind::Hcl => Hcl::from(xyz).as_vec(),
            XyzKind::Hlab => Hlab::from(xyz).as_vec(),
            XyzKind::Lab => Lab::from(xyz).as_vec(),
            XyzKind::LchLab => Lchlab::from(xyz).as_vec(),
            XyzKind::Lchuv => Lchuv::from(xyz).as_vec(),
            XyzKind::Luv => Luv::from(xyz).as_vec(),
            XyzKind::Oklab => OkLab::from(xyz).as_vec(),
            XyzKind::Oklch => OkLch::from(xyz).as_vec(),
            XyzKind::REC709 => Rec709::from(xyz).as_vec(),
            XyzKind::REC2020 => Rec2020::from(xyz).as_vec(),
            XyzKind::REC2100 => Rec2100::from(xyz).as_vec(),
            XyzKind::SRGB => Srgb::from(xyz).as_vec(),
            XyzKind::Xyy => Xyy::from(xyz).as_vec(),
            XyzKind::Rgb => xyz.as_rgb(light).as_vec(),
        };

        color.get_color_slice(vec);

        Ok(color)
    }

    /// Drop inner fields of the AnyColor struct. As the struct has been allocated by Rust
    /// it's then also Rust's job to deallocate the memory inner field
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    pub fn drop_inner_fields(&self) {
        if !self.hex.is_null() {
            let s = unsafe { CString::from_raw(self.hex as *mut _) };

            drop(s);
        }

        if !self.slice.is_null() {
            unsafe {
                let slice = Box::from_raw(self.slice);
                let vec = Vec::from_raw_parts(slice.ptr as *mut f64, slice.len, slice.len);

                drop(vec);
            };
        }
    }

    /// Get a slice of color based on the vector of f64 which represent a color
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    /// * `mut vec` - Vec<f64>
    fn get_color_slice(&mut self, mut vec: Vec<f64>) {
        // a vector is made of the three following components
        // - length
        // - capacity
        // - pointer
        // Reduce the capacity to not waste space
        vec.shrink_to_fit();
        assert!(vec.len() == vec.capacity());

        let len = vec.len();
        let ptr = vec.as_ptr();

        // Prevent dealloc from rust
        // Vec is still present but not a Rust's object responsabi lity
        std::mem::forget(vec);

        let slice = ColorSlice { len, ptr };

        self.slice = Box::into_raw(Box::new(slice));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_rgb_to_color() {
        let color = AnyColor::from_rgb(
            Rgb {
                r: 17,
                g: 112,
                b: 190,
            },
            RgbKind::Hex,
        );

        assert!(color.is_ok());
    }
}
