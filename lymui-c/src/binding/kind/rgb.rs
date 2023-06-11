use anyhow::Result;
use std::ffi::c_void;
use lymui::create_color_from_vec;
use lymui::prelude::*;
use std::ffi::CStr;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum RgbKind {
    Cymk,
    Hex,
    Hue,
    Hsl,
    Hsv,
    Hwb,
    Rgb,
    YCbCr,
    Yuv,
    Ansi16,
    Ansi256
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

        match self {
            Self::Hex => {
                let hex = unsafe { CStr::from_ptr(ptr as *const _) }
                    .to_string_lossy()
                    .into_owned();

                Ok(Rgb::try_from(Hex(hex))?)
            }
            Self::Hsl | Self::Hsv | Self::Hwb | Self::YCbCr | Self::Yuv | Self::Rgb | Self::Cymk => {                
                Ok(self.create_rgb_from_slice(ptr))
            }
            _ => Err(anyhow::format_err!(
                "Targeted color could not be convert into Rgb"
            ))
        }
    }

    /// Create an RGB from a vector which should contains 3 elements of a color space
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `ptr` - *mut c_void
    fn create_rgb_from_slice(&self, ptr: *mut c_void) -> Rgb {
        let vec = match self {
            Self::Cymk =>  unsafe { std::slice::from_raw_parts(ptr as *mut f64, 4) }.to_vec(),
            _ => unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) }.to_vec()
        };

        match self {
            Self::Rgb => {
                let u8_vec: Vec<u8> = vec.into_iter().map(|c| c as u8).collect::<Vec<_>>();
                create_color_from_vec::<u8, Rgb>(u8_vec)
            }
            Self::Hsl => Rgb::from(create_color_from_vec::<f64, Hsl>(vec)),
            Self::Hsv => Rgb::from(create_color_from_vec::<f64, Hsv>(vec)),
            Self::Hwb => Rgb::from(create_color_from_vec::<f64, Hwb>(vec)),
            Self::YCbCr => {
                let u8_vec: Vec<u8> = vec.into_iter().map(|c| c as u8).collect::<Vec<_>>();
                Rgb::from(create_color_from_vec::<u8, Ycbcr>(u8_vec))
            }
            Self::Yuv => Rgb::from(create_color_from_vec::<f64, Yuv>(vec)),
            Self::Cymk => Rgb::from(create_color_from_vec::<f64, Cymk>(vec)),
            _ => Rgb::default(),
        }
    }
}
