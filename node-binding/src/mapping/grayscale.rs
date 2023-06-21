use lymui::{
    grayscale::Kind,
    js::{FromJsObject, IntoJsObject},
    prelude::*,
    rgb::FromRgb,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum GrayscaleMapping {
    Lightness,
    Average,
    Luminosity,
    BT709,
    BT2100,
}

impl GrayscaleMapping {
    /// Convert a js rgb object into a grayscale
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `obj - Object
    /// * `env` - Env`
    pub(crate) fn into_grayscale(self, obj: Object, env: Env) -> Result<Object> {
        let rgb = Rgb::from_js_object(obj)?;

        let grayscale = match self {
            Self::Lightness => GrayScale::from_rgb(rgb, Kind::Lightness),
            Self::Average => GrayScale::from_rgb(rgb, Kind::Average),
            Self::Luminosity => GrayScale::from_rgb(rgb, Kind::Luminosity),
            Self::BT709 => GrayScale::from_rgb(rgb, Kind::BT709),
            Self::BT2100 => GrayScale::from_rgb(rgb, Kind::BT2100),
        };

        grayscale.into_js_object(env)
    }
}
