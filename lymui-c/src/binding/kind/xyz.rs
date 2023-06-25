use crate::LightKind;
use anyhow::Result;
use core::ffi::c_void;
use lymui::create_color_from_vec;
use lymui::prelude::*;
use lymui::rgb::FromRgb;
use super::rgb::RgbKind;

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
    Rgb,
}

impl XyzKind {
    /// Convert a c_void pointer into a valid Xyz color
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    /// * `ptr` - *mut c_void
    /// * `light` - XyzLight
    pub fn as_xyz(&self, ptr: *mut c_void, light: Option<LightKind>) -> Result<Xyz> {
        if ptr.is_null() {
            return Err(anyhow::format_err!(
                "Pointer is null. Unable to convert reflect the color as an Xyz"
            ));
        }

        let xyz = match self {
            Self::Rgb => {
                let Some(xyz_light) = light else {
                    return Err(anyhow::format_err!("Lightkind is not present in the parameter"))
                };

                let rgb_kind = RgbKind::Rgb;
                let rgb = rgb_kind.as_rgb(ptr)?;

                Xyz::from_rgb(rgb, xyz_light)
            }
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
            Self::Xyz => create_color_from_vec::<f64, Xyz>(vec),
            _ => Xyz::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_rgb_to_xyz() {
        let tgt = XyzKind::Rgb;
        let mut rgb = vec![17.0, 18.0, 92.0];
        let rgb_ptr = rgb.as_mut_ptr() as *mut c_void;

        let res = tgt.as_xyz(rgb_ptr, Some(LightKind::D65));
        assert!(res.is_ok());

        let xyz = res.unwrap();
        assert_eq!(xyz.x, 0.023785878915414407);
        assert_eq!(xyz.y, 0.013242343593540241);
        assert_eq!(xyz.z, 0.10253384014175347);
    }

    #[test]
    fn expect_to_convert_slice_to_xyz() {
        let tgt = XyzKind::Oklab;
        let mut lab: Vec<f64> = vec![
            0.26368,
            0.06116,
            -0.1258
        ];
        let lab_ptr = lab.as_mut_ptr() as *mut c_void;
        
        let res = tgt.as_xyz(lab_ptr, None);
        assert!(res.is_ok());
        
        let xyz = res.unwrap();
        assert!(xyz.x.is_sign_positive());
        assert!(xyz.y.is_sign_positive());
        assert!(xyz.z.is_sign_positive());
    }
}
