use anyhow::Result;
use lymui::create_color_from_vec;
use lymui::prelude::*;
use std::ffi::c_void;
use std::ffi::CStr;

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
    Ansi16,
    Ansi256,
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
            Self::Hsl
            | Self::Hsv
            | Self::Hwb
            | Self::YCbCr
            | Self::Yuv
            | Self::Rgb
            | Self::Cymk => Ok(self.create_rgb_from_slice(ptr)),
            Self::Ansi16 | Self::Ansi256 => {
                let u = ptr as u8;
                Rgb::try_from(Ansi(u))
                    .map_err(|err| anyhow::format_err!(err.to_string()))
            }
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
            Self::Cymk => unsafe { std::slice::from_raw_parts(ptr as *mut f64, 4) }.to_vec(),
            _ => unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) }.to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_hex_to_rgb() {
        let tgt = RgbKind::Hex;
        let hex = "#ffffff";
        let hex_ptr = hex.as_ptr() as *mut c_void;

        let res = tgt.as_rgb(hex_ptr);
        assert!(res.is_ok());
        
        let rgb = res.unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn expect_to_convert_slice_to_rgb() {
        let tgt = RgbKind::Hsl;
        let mut hsl = vec![237_f64, 90_f64, 19.6];
        let hsl_ptr = hsl.as_mut_ptr() as *mut c_void;

        let res = tgt.as_rgb(hsl_ptr);
        assert!(res.is_ok());

        let rgb = res.unwrap();
        assert_eq!(rgb.r, 5);
        assert_eq!(rgb.g, 9);
        assert_eq!(rgb.b, 95);
    }

    #[test]
    fn expect_to_convert_ansi_to_rgb() {
        let tgt = RgbKind::Ansi256;
        let ansi = Ansi(114);
        let ansi_ptr = Box::into_raw(Box::new(ansi)) as *mut c_void;

        let res = tgt.as_rgb(ansi_ptr);
        assert!(res.is_ok());

        let rgb = res.unwrap();
        assert_eq!(rgb.r, 92);
        assert_eq!(rgb.g, 191);
        assert_eq!(rgb.b, 84);
    }

    #[test]
    fn expect_to_handle_null_pointer() {
        let tgt = RgbKind::Hsv;
        let null_ptr = std::ptr::null_mut();

        let res = tgt.as_rgb(null_ptr);
        assert!(res.is_err());
    }
}
