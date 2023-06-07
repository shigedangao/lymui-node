use napi::{bindgen_prelude::Object, Env};
use anyhow::Result;

pub mod rgb;
pub mod hex;
pub mod yuv;
pub mod cymk;
pub mod hsl;
pub mod hsv;
pub mod hue;
pub mod hwb;
pub mod ycbcr;

pub(crate) trait FromObject {
    fn from_js_object(object: Object) -> Result<Self> where Self: Sized;
}

pub(crate) trait IntoObject {
    fn as_js_object(&self, env: Env) -> Result<Object> where Self: Sized;
}
