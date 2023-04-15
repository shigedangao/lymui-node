use anyhow::Result;
use core::ffi::c_void;
use lymui::create_color_from_vec;
use lymui::rgb::{FromRgb, Rgb};
use lymui::xyz::hlab::Hlab;
use lymui::xyz::lab::Lab;
use lymui::xyz::lchlab::Lchlab;
use lymui::xyz::lchuv::Lchuv;
use lymui::xyz::luv::Luv;
use lymui::xyz::oklab::OkLab;
use lymui::xyz::oklch::OkLch;
use lymui::xyz::rec2020::Rec2020;
use lymui::xyz::rec2100::Rec2100;
use lymui::xyz::rec709::Rec709;
use lymui::xyz::srgb::Srgb;
use lymui::xyz::xyy::Xyy;
use lymui::xyz::{argb::Argb, hcl::Hcl, Kind as LightKind, Xyz};

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
                let slice = unsafe { std::slice::from_raw_parts(ptr as *mut f64, 3) }.to_vec();
                let rgb = Rgb {
                    r: slice.first().copied().unwrap_or_default() as u8,
                    g: slice.get(1).copied().unwrap_or_default() as u8,
                    b: slice.get(2).copied().unwrap_or_default() as u8,
                };

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
