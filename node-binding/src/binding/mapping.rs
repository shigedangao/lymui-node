use napi::bindgen_prelude::*;
use napi_derive::napi;
use anyhow::{Result, anyhow};
use lymui::{
    prelude::*,
    xyz::Kind::D65, rgb::FromRgb
};
use super::color::*;

#[napi]
pub enum Mapping {
    Cymk,
    Hex,
    Hsl,
    Hsv,
    Hue,
    Hwb,
    Rgb,
    Yuv,
    Ycbcr,
    Xyz,
    Argb,
    Hcl,
    Hlab
}

impl Mapping {
    /// Convert a JS object into an RGB. This will allows us to convert it to an other type T
    /// 
    /// # Arguments
    /// 
    /// * `object` - Object
    pub fn get_rgb_from_mapping(&self, object: Object) -> Result<Rgb> {        
        let rgb = match self {
            Self::Cymk => Ok(Rgb::from(Cymk::from_js_object(object)?)),
            Self::Hex => {
                let hex = Hex::from_js_object(object)?;
                Rgb::try_from(hex).map_err(|err| anyhow!(err.to_string()))
            },
            Self::Hsl => Ok(Rgb::from(Hsl::from_js_object(object)?)),
            Self::Hue => Ok(Rgb::default()),
            Self::Hsv => Ok(Rgb::from(Hsv::from_js_object(object)?)),
            Self::Rgb => Rgb::from_js_object(object),
            Self::Yuv => Ok(Rgb::from(Yuv::from_js_object(object)?)),
            Self::Hwb => Ok(Rgb::from(Hwb::from_js_object(object)?)),
            Self::Ycbcr => Ok(Rgb::from(Ycbcr::from_js_object(object)?)),
            Self::Xyz => Ok(Xyz::from_js_object(object)?.as_rgb(D65)),
            Self::Argb => Ok(Xyz::from(Argb::from_js_object(object)?).as_rgb(D65)),
            Self::Hcl => Ok(Xyz::from(Hcl::from_js_object(object)?).as_rgb(D65)),
            Self::Hlab => Ok(Xyz::from(Hlab::from_js_object(object)?).as_rgb(D65))
        }?;

        Ok(rgb)
    }
    /// Create a color from an RGB input
    /// 
    /// # Arguments
    /// 
    /// * `rgb` - Rgb
    /// * `env` - Env
    pub fn create_color_from_rgb_input(&self, rgb: Rgb, env: Env) -> Result<Object> {
        match self {
            Self::Cymk => Cymk::from(rgb).as_js_object(env),
            Self::Hex => Hex::from(rgb).as_js_object(env),
            Self::Hsl => Hsl::from(rgb).as_js_object(env),
            Self::Hsv => Hsv::from(rgb).as_js_object(env),
            Self::Rgb => rgb.as_js_object(env),
            Self::Yuv => Yuv::from(rgb).as_js_object(env),
            Self::Hue => Hue::from(rgb).as_js_object(env),
            Self::Hwb => Hwb::from(rgb).as_js_object(env),
            Self::Ycbcr => Ycbcr::from(rgb).as_js_object(env),
            Self::Xyz => Xyz::from_rgb(rgb, D65).as_js_object(env),
            Self::Argb => Argb::from(Xyz::from_rgb(rgb, D65)).as_js_object(env),
            Self::Hcl => Hcl::from(Xyz::from_rgb(rgb, D65)).as_js_object(env),
            Self::Hlab => Hlab::from(Xyz::from_rgb(rgb, D65)).as_js_object(env)
        }
    }
}
