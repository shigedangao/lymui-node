use anyhow::{anyhow, Result};
use lymui::js::prelude::*;
use lymui::prelude::*;
use lymui::rgb::FromRgb;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum RgbMapping {
    Cymk,
    Hex,
    Hsl,
    Hsv,
    Hwb,
    Rgb,
    Yuv,
    Ycbcr,
    XyzD65,
    XyzD50,
    XyzAdobe,
    Ansi16,
    Ansi256,
}

impl RgbMapping {
    /// Convert a JS object into an RGB. This will allows us to convert it to an other type T
    ///
    /// # Arguments
    ///
    /// * `object` - Object
    pub(crate) fn get_rgb_from_mapping(&self, object: Object) -> Result<Rgb> {
        let rgb = match self {
            Self::Cymk => Rgb::from(Cymk::from_js_object(object)?),
            Self::Hex => {
                let hex = Hex::from_js_object(object)?;
                Rgb::try_from(hex).map_err(|err| anyhow!(err.to_string()))?
            }
            Self::Hsl => Rgb::from(Hsl::from_js_object(object)?),
            Self::Hsv => Rgb::from(Hsv::from_js_object(object)?),
            Self::Rgb => Rgb::from_js_object(object)?,
            Self::Yuv => Rgb::from(Yuv::from_js_object(object)?),
            Self::Hwb => Rgb::from(Hwb::from_js_object(object)?),
            Self::Ycbcr => Rgb::from(Ycbcr::from_js_object(object)?),
            Self::XyzD65 => Xyz::from_js_object(object)?.as_rgb(lymui::xyz::Kind::D65),
            Self::XyzD50 => Xyz::from_js_object(object)?.as_rgb(lymui::xyz::Kind::D50),
            Self::XyzAdobe => Xyz::from_js_object(object)?.as_rgb(lymui::xyz::Kind::Adobe),
            Self::Ansi16 | Self::Ansi256 => Rgb::try_from(Ansi::from_js_object(object)?)?,
        };

        Ok(rgb)
    }

    /// Create a color from an RGB input
    ///
    /// # Arguments
    ///
    /// * `rgb` - Rgb
    /// * `env` - Env
    pub(crate) fn create_color_from_rgb_input(&self, rgb: Rgb, env: Env) -> Result<Object> {
        let res = match self {
            Self::Cymk => Cymk::from(rgb).into_js_object(env),
            Self::Hex => Hex::from(rgb).into_js_object(env),
            Self::Hsl => Hsl::from(rgb).into_js_object(env),
            Self::Hsv => Hsv::from(rgb).into_js_object(env),
            Self::Rgb => rgb.into_js_object(env),
            Self::Yuv => Yuv::from(rgb).into_js_object(env),
            Self::Hwb => Hwb::from(rgb).into_js_object(env),
            Self::Ycbcr => Ycbcr::from(rgb).into_js_object(env),
            Self::XyzD65 => Xyz::from_rgb(rgb, lymui::xyz::Kind::D65).into_js_object(env),
            Self::XyzD50 => Xyz::from_rgb(rgb, lymui::xyz::Kind::D50).into_js_object(env),
            Self::XyzAdobe => Xyz::from_rgb(rgb, lymui::xyz::Kind::Adobe).into_js_object(env),
            Self::Ansi16 => Ansi::from_rgb(rgb, AnsiKind::C16).into_js_object(env),
            Self::Ansi256 => Ansi::from_rgb(rgb, AnsiKind::C256).into_js_object(env),
        }?;

        Ok(res)
    }
}
