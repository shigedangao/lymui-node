use napi::{bindgen_prelude::Object};
use napi::{Env};
use lymui::hsv::Hsv;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hsv {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("h", self.h)?;
        res.set("s", self.s)?;
        res.set("v", self.v)?;

        Ok(res)   
    }
}

impl FromObject for Hsv {
    fn from_js_object(object: Object) -> Result<Hsv> {
        let h: f64 = object.get("h")?.ok_or_else(|| anyhow!("Unable to found h"))?;
        let s: f64 = object.get("s")?.ok_or_else(|| anyhow!("Unable to found s"))?;
        let v: f64 = object.get("v")?.ok_or_else(|| anyhow!("Unable to found v"))?;

        Ok(Hsv { h, s, v })
    }
}
