use napi::bindgen_prelude::Object;
use napi::Env;
use lymui::hue::Hue;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hue {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("hue", *self)?;

        Ok(res)
    }
}

impl FromObject for Hue {
    fn from_js_object(object: Object) -> Result<Self> {
        let hue: f64 = object.get("hue")?.ok_or_else(|| anyhow!("Unable to found hue"))?;

        Ok(hue)
    }
}
