use anyhow::{Result, anyhow};
use lymui::{
    convert_rgb_subcolor, from_rgb_space_to_xyz_space, ops::SliceOps, prelude::*, rgb::FromRgb,
    xyz::Kind,
};
use std::ffi::{CStr, c_void};
use std::ops::Add;

use crate::Lumens;

/// Represents the color mapping to use when converting a color.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum ColorMapping {
    Rgb,
    Cymk,
    Hex,
    Hsl,
    Hsv,
    Hwb,
    YCbCr,
    Yuv,
    Ansi16,
    Ansi256,
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
}

/// Represents the kind of color slice to use when converting a color.
pub enum ColorSliceKind<'a> {
    Standard3Size(&'a [f64; 3]),
    Standard3SizeU8(&'a [u8; 3]),
    Standard4Size(&'a [f64; 4]),
    Hex(String),
}

/// Represents the result of a color conversion.
#[derive(Debug)]
pub enum ColorConversionResult {
    Color(Vec<f64>),
    Hex(String),
}

/// Construct an RGB color from a slice of values.
///
/// # Arguments
///
/// * `ptr` - The pointer to the slice of values.
///
/// # Returns
///
/// The RGB color constructed from the slice of values.
pub fn construct_rgb_from_slice<const S: usize, T: SliceOps<S, Item = E> + Into<Rgb>, E: Add>(
    ptr: *mut c_void,
) -> Rgb {
    // Recreate the slice from the pointer which represents a color that can be convert directly to RGB.
    let slice =
        get_color_slice_from_ptr::<E, S>(ptr).expect("Expect to have convert the type into rgb");

    // Create the color from the slice (e.g: RGB, CYMK, etc.).
    let tyy: T = lymui::create_color_from_slice(slice);

    // Convert the color to RGB.
    tyy.into()
}

/// Construct an RGB color from a slice of XYZ values.
///
/// # Arguments
///
/// * `ptr` - The pointer to the slice of XYZ values.
///
/// # Returns
///
/// The RGB color constructed from the slice of XYZ values.
pub fn construct_rgb_from_xyz_slice<
    const S: usize,
    T: SliceOps<S, Item = E> + Into<Xyz>,
    E: Add,
>(
    ptr: *mut c_void,
    lumens: &Lumens,
) -> Rgb {
    // Recreate the slice from the pointer which represents a color that can be convert directly to XYZ.
    let slice =
        get_color_slice_from_ptr::<E, S>(ptr).expect("Expect to have convert the type into xyz");

    // Create the color from the slice (e.g: XYZ, YUV, etc.).
    let tyy: T = lymui::create_color_from_slice(slice);
    // Convert the color to XYZ.
    let xyz = tyy.into();

    let lib_lumens = match lumens {
        Lumens::None => Kind::D65,
        Lumens::D65 => Kind::D65,
        Lumens::D50 => Kind::D50,
        Lumens::Adobe => Kind::Adobe,
    };

    // Convert the XYZ color to RGB.
    xyz.as_rgb(lib_lumens)
}

/// Converts a raw pointer to a color slice of the specified size.
///
/// # Arguments
///
/// * `ptr` - The raw pointer to convert.
///
/// # Returns
///
/// A color slice of the specified size.
fn get_color_slice_from_ptr<'a, T, const S: usize>(ptr: *mut c_void) -> Result<&'a [T; S]> {
    let s = unsafe { std::slice::from_raw_parts(ptr as *mut T, S) };
    let slice = s
        .try_into()
        .map_err(|err| anyhow!("invalid size {err:?}"))?;

    Ok(slice)
}

impl ColorMapping {
    /// Get an RGB color from a color space RGB pointer.
    ///
    /// # Arguments
    ///
    /// * `self` - The color mapping to get the RGB color from.
    /// * `ptr` - The pointer to the color space RGB data.
    ///
    /// # Returns
    ///
    /// Returns an [`Rgb`] color.
    pub(crate) fn get_rgb_from_color_space_rgb(
        &self,
        ptr: *mut c_void,
        lumens: &Lumens,
    ) -> Result<Rgb> {
        if *self == Self::Hex {
            let hex = unsafe { CStr::from_ptr(ptr as *const _) }
                .to_string_lossy()
                .into_owned();

            return Ok(Rgb::try_from(Hex(hex))?);
        }

        if *self == Self::Ansi16 || *self == Self::Ansi256 {
            let ansi = Ansi(get_color_slice_from_ptr::<u8, 1>(ptr)?[0]);
            return Ok(Rgb::try_from(ansi)?);
        }

        let rgb = match self {
            Self::Rgb => construct_rgb_from_slice::<3, Rgb, u8>(ptr),
            Self::Cymk => construct_rgb_from_slice::<4, Cymk, f64>(ptr),
            Self::Hsl => construct_rgb_from_slice::<3, Hsl, f64>(ptr),
            Self::Hsv => construct_rgb_from_slice::<3, Hsv, f64>(ptr),
            Self::Hwb => construct_rgb_from_slice::<3, Hwb, f64>(ptr),
            Self::YCbCr => construct_rgb_from_slice::<3, Ycbcr, u8>(ptr),
            Self::Yuv => construct_rgb_from_slice::<3, Yuv, f64>(ptr),
            Self::Xyz => construct_rgb_from_xyz_slice::<3, Xyz, f64>(ptr, lumens),
            Self::Argb => construct_rgb_from_xyz_slice::<3, Argb, f64>(ptr, lumens),
            Self::Hcl => construct_rgb_from_xyz_slice::<3, Hcl, f64>(ptr, lumens),
            Self::Hlab => construct_rgb_from_xyz_slice::<3, Hlab, f64>(ptr, lumens),
            Self::Lab => construct_rgb_from_xyz_slice::<3, Lab, f64>(ptr, lumens),
            Self::LchLab => construct_rgb_from_xyz_slice::<3, Lchlab, f64>(ptr, lumens),
            Self::Lchuv => construct_rgb_from_xyz_slice::<3, Lchuv, f64>(ptr, lumens),
            Self::Luv => construct_rgb_from_xyz_slice::<3, Luv, f64>(ptr, lumens),
            Self::Oklab => construct_rgb_from_xyz_slice::<3, OkLab, f64>(ptr, lumens),
            Self::Oklch => construct_rgb_from_xyz_slice::<3, OkLch, f64>(ptr, lumens),
            Self::REC709 => construct_rgb_from_xyz_slice::<3, Rec709, f64>(ptr, lumens),
            Self::REC2020 => construct_rgb_from_xyz_slice::<3, Rec2020, f64>(ptr, lumens),
            Self::REC2100 => construct_rgb_from_xyz_slice::<3, Rec2100, f64>(ptr, lumens),
            Self::SRGB => construct_rgb_from_xyz_slice::<3, Srgb, f64>(ptr, lumens),
            Self::Xyy => construct_rgb_from_xyz_slice::<3, Xyy, f64>(ptr, lumens),
            _ => unreachable!(),
        };

        Ok(rgb)
    }

    /// Wrap the lymui color conversion function for the given color mapping.
    ///
    /// # Arguments
    ///
    /// * `self` - The color mapping to wrap the lymui color conversion function for.
    /// * `rgb` - The RGB color to convert.
    /// * `lumens` - The lumens to use for the color conversion.
    ///
    /// # Returns
    ///
    /// Returns a [`ColorConversionResult`] containing the converted color.
    pub(crate) fn wrap_lymui_convert_space(
        &self,
        rgb: Rgb,
        lumens: Option<Kind>,
    ) -> Result<ColorConversionResult> {
        // Special case for RGB color space, which returns the color as-is.
        if *self == Self::Rgb {
            return Ok(ColorConversionResult::Color(
                rgb.as_slice().map(f64::from).to_vec(),
            ));
        }

        if *self == Self::YCbCr {
            return Ok(ColorConversionResult::Color(
                lymui::convert_rgb_subcolor::<Rgb, Ycbcr>(rgb)
                    .as_slice()
                    .map(f64::from)
                    .to_vec(),
            ));
        }

        // Special case for Hex color space, which returns a string instead of a color vector.
        if *self == Self::Hex {
            return Ok(ColorConversionResult::Hex(
                lymui::convert_rgb_subcolor::<Rgb, Hex>(rgb).0,
            ));
        }

        // Convert the RGB color to the appropriate color space and return the result
        // and returns a vector of color values (easier for further processing).
        let color_vec = match self {
            Self::Cymk => convert_rgb_subcolor::<Rgb, Cymk>(rgb).to_vec(),
            Self::Hsl => convert_rgb_subcolor::<Rgb, Hsl>(rgb).to_vec(),
            Self::Hsv => convert_rgb_subcolor::<Rgb, Hsv>(rgb).to_vec(),
            Self::Hwb => convert_rgb_subcolor::<Rgb, Hwb>(rgb).to_vec(),
            Self::Yuv => convert_rgb_subcolor::<Rgb, Yuv>(rgb).to_vec(),
            Self::Xyz => from_rgb_space_to_xyz_space::<Rgb, Xyz>(rgb, lumens).to_vec(),
            Self::Argb => from_rgb_space_to_xyz_space::<Rgb, Argb>(rgb, lumens).to_vec(),
            Self::Hcl => from_rgb_space_to_xyz_space::<Rgb, Hcl>(rgb, lumens).to_vec(),
            Self::Hlab => from_rgb_space_to_xyz_space::<Rgb, Hlab>(rgb, lumens).to_vec(),
            Self::Lab => from_rgb_space_to_xyz_space::<Rgb, Lab>(rgb, lumens).to_vec(),
            Self::LchLab => from_rgb_space_to_xyz_space::<Rgb, Lchlab>(rgb, lumens).to_vec(),
            Self::Lchuv => from_rgb_space_to_xyz_space::<Rgb, Lchuv>(rgb, lumens).to_vec(),
            Self::Luv => from_rgb_space_to_xyz_space::<Rgb, Luv>(rgb, lumens).to_vec(),
            Self::Oklab => from_rgb_space_to_xyz_space::<Rgb, OkLab>(rgb, lumens).to_vec(),
            Self::Oklch => from_rgb_space_to_xyz_space::<Rgb, OkLch>(rgb, lumens).to_vec(),
            Self::REC709 => from_rgb_space_to_xyz_space::<Rgb, Rec709>(rgb, lumens).to_vec(),
            Self::REC2020 => from_rgb_space_to_xyz_space::<Rgb, Rec2020>(rgb, lumens).to_vec(),
            Self::REC2100 => from_rgb_space_to_xyz_space::<Rgb, Rec2100>(rgb, lumens).to_vec(),
            Self::SRGB => from_rgb_space_to_xyz_space::<Rgb, Srgb>(rgb, lumens).to_vec(),
            Self::Xyy => from_rgb_space_to_xyz_space::<Rgb, Xyy>(rgb, lumens).to_vec(),
            Self::Ansi16 => {
                vec![Ansi::from_rgb(rgb, AnsiKind::C16).0 as f64]
            }
            Self::Ansi256 => {
                vec![Ansi::from_rgb(rgb, AnsiKind::C256).0 as f64]
            }
            _ => unreachable!(),
        };

        Ok(ColorConversionResult::Color(color_vec))
    }
}
