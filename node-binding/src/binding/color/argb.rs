use lymui::prelude::Argb;
use napi::bindgen_prelude::Object;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Argb {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("r", self.r)?;
        res.set("g", self.g)?;
        res.set("b", self.b)?;

        Ok(res)
    }
}

impl FromObject for Argb {
    fn from_js_object(object: Object) -> Result<Argb> {
        let r: f64 = object.get("r")?.ok_or_else(|| anyhow!("Unable to found r"))?;
        let g: f64 = object.get("g")?.ok_or_else(|| anyhow!("Unable to found g"))?;
        let b: f64 = object.get("b")?.ok_or_else(|| anyhow!("Unable to found b"))?;

        Ok(Argb { r, g, b })
    }
}
