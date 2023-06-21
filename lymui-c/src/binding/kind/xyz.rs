use crate::LightKind;
use anyhow::Result;
use core::ffi::c_void;
use lymui::create_color_from_vec;
use lymui::prelude::*;
use lymui::rgb::{FromRgb, Rgb};
use lymui::util::FromVec;

#[derive(Debug)]
#[repr(C)]
pub enum XyzLight {
    D65,
    D50,
    Adobe,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum XyzKind {
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
    // Special case when a user ask for a Xyz from an RGB
    RgbCompat,
}

impl XyzKind {
    /// Convert a c_void pointer into a valid Xyz color
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    /// * `ptr` - *mut c_void
    /// * `light` - XyzLight
    pub fn as_xyz(&self, ptr: *mut c_void, light: LightKind) -> Result<Xyz> {
        if ptr.is_null() {
            return Err(anyhow::format_err!(
                "Pointer is null. Unable to convert reflect the color as an Xyz"
            ));
        }

        let xyz = match self {
            Self::RgbCompat => {
                let slice = unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) }
                    .iter()
                    .copied()
                    .map(|v| v as u8)
                    .collect::<Vec<_>>();

                let rgb = Rgb::from_vec(slice.to_vec());

                Xyz::from_rgb(rgb, light)
            }
            Self::Xyz => unsafe { *(ptr as *mut Xyz) },
            _ => {
                let slice = unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) };
                self.create_xyz_from_vec(slice.to_vec())
            }
        };

        Ok(xyz)
    }

    /// Convert a color to an Xyz color space
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    /// * `vec` - Vec<f64>
    /// * `kind` - XyzLight
    fn create_xyz_from_vec(&self, vec: Vec<f64>) -> Xyz {
        // convert to a vector

        match self {
            Self::Argb => Xyz::from(create_color_from_vec::<f64, Argb>(vec)),
            Self::Hcl => Xyz::from(create_color_from_vec::<f64, Hcl>(vec)),
            Self::Hlab => Xyz::from(create_color_from_vec::<f64, Hlab>(vec)),
            Self::Lab => Xyz::from(create_color_from_vec::<f64, Lab>(vec)),
            Self::LchLab => Xyz::from(create_color_from_vec::<f64, Lchlab>(vec)),
            Self::Lchuv => Xyz::from(create_color_from_vec::<f64, Lchuv>(vec)),
            Self::Luv => Xyz::from(create_color_from_vec::<f64, Luv>(vec)),
            Self::Oklab => Xyz::from(create_color_from_vec::<f64, OkLab>(vec)),
            Self::Oklch => Xyz::from(create_color_from_vec::<f64, OkLch>(vec)),
            Self::REC709 => Xyz::from(create_color_from_vec::<f64, Rec709>(vec)),
            Self::REC2020 => Xyz::from(create_color_from_vec::<f64, Rec2020>(vec)),
            Self::REC2100 => Xyz::from(create_color_from_vec::<f64, Rec2100>(vec)),
            Self::SRGB => Xyz::from(create_color_from_vec::<f64, Srgb>(vec)),
            Self::Xyy => Xyz::from(create_color_from_vec::<f64, Xyy>(vec)),
            _ => Xyz::default(),
        }
    }
}
