use napi::bindgen_prelude::*;
use napi_derive::napi;
use anyhow::{Result, anyhow};
use lymui::prelude::*;
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
    Ycbcr
}

impl Mapping {
    /// Convert a JS object into an RGB. This will allows us to convert it to an other type T
    /// 
    /// # Arguments
    /// 
    /// * `object` - Object
    pub fn get_rgb_from_mapping(&self, object: Object) -> Result<Rgb> {
        // @TODO handle special case ?
        
        let rgb = match self {
            Self::Cymk => Ok(Rgb::from(Cymk::from_js_object(object)?)),
            Self::Hex => {
                let hex = Hex::from_js_object(object)?;
                Rgb::try_from(hex).map_err(|err| anyhow!(err.to_string()))
            },
            Self::Hsl => Ok(Rgb::from(Hsl::from_js_object(object)?)),
            Self::Hsv => Ok(Rgb::from(Hsv::from_js_object(object)?)),
            Self::Rgb => Rgb::from_js_object(object),
            Self::Yuv => Ok(Rgb::from(Yuv::from_js_object(object)?)),
            Self::Hwb => Ok(Rgb::from(Hwb::from_js_object(object)?)),
            Self::Ycbcr => Ok(Rgb::from(Ycbcr::from_js_object(object)?)),
            _ => Ok(Rgb::default()),
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
            Self::Ycbcr => Ycbcr::from(rgb).as_js_object(env)
        }
    }
}
