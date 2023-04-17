use anyhow::Result;
use libc::c_void;
use lymui::{cymk::Cymk, hsl::Hsl, hsv::Hsv, hwb::Hwb, rgb::Rgb, ycbcr::Ycbcr, yuv::Yuv};
use std::ffi::CStr;
use lymui::create_color_from_vec;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum RgbKind {
    Cymk,
    Hex,
    Hsl,
    Hsv,
    Hwb,
    Rgb,
    YCbCr,
    Yuv,
}

impl RgbKind {
    /// Convert a c_void pointer into a valid RGB color
    ///
    /// # Arguments
    ///
    /// * `&self` - Kind
    /// * `ptr` - *mut c_void
    pub fn as_rgb(&self, ptr: *mut c_void) -> Result<Rgb> {
        if ptr.is_null() {
            return Err(anyhow::format_err!(
                "Pointer is null. Unable to convert color as RGB"
            ));
        }

        let rgb = match self {
            Self::Hex => {
                let hex = unsafe { CStr::from_ptr(ptr as *const _) }
                    .to_string_lossy()
                    .into_owned();

                Rgb::try_from(hex)?
            }
            Self::Rgb => unsafe { *(ptr as *mut Rgb) },
            Self::Cymk => {
                let slice = unsafe { std::slice::from_raw_parts(ptr as *mut f64, 4) };
                let cymk = Cymk {
                    c: slice.first().copied().unwrap_or_default(),
                    y: slice.get(1).copied().unwrap_or_default(),
                    m: slice.get(2).copied().unwrap_or_default(),
                    k: slice.get(3).copied().unwrap_or_default(),
                };

                Rgb::from(cymk)
            }
            Self::Hsl | Self::Hsv | Self::Hwb | Self::YCbCr | Self::Yuv => {
                let slice = unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) };
                self.create_rgb_from_array_three_items(slice.to_vec())
            }
        };

        Ok(rgb)
    }

    /// Create an RGB from a vector which should contains 3 elements of a color space
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `vec` - Vec<f64>
    fn create_rgb_from_array_three_items(&self, vec: Vec<f64>) -> Rgb {
        match self {
            Self::Hsl => Rgb::from(create_color_from_vec::<f64, Hsl>(vec)),
            Self::Hsv => Rgb::from(create_color_from_vec::<f64, Hsv>(vec)),
            Self::Hwb => Rgb::from(create_color_from_vec::<f64, Hwb>(vec)),
            Self::YCbCr => {
                let u8_vec = vec.into_iter().map(|c| c as u8).collect::<Vec<_>>();
                Rgb::from(create_color_from_vec::<u8, Ycbcr>(u8_vec))
            },
            Self::Yuv => Rgb::from(create_color_from_vec::<f64, Yuv>(vec)),
            _ => Rgb::default(),
        }
    }
}
