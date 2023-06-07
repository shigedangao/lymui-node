use napi::{bindgen_prelude::Object};
use napi::{Env};
use lymui::rgb::Rgb;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Rgb {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("r", self.r)?;
        res.set("g", self.g)?;
        res.set("b", self.b)?;

        Ok(res)   
    }
}

impl FromObject for Rgb {
    fn from_js_object(object: Object) -> Result<Rgb> {
        let r: u8 = object.get("r")?.ok_or_else(|| anyhow!("Unable to found r"))?;
        let g: u8 = object.get("g")?.ok_or_else(|| anyhow!("Unable to found g"))?;
        let b: u8 = object.get("b")?.ok_or_else(|| anyhow!("Unable to found b"))?;

        Ok(Rgb { r, g, b })
    }
}
