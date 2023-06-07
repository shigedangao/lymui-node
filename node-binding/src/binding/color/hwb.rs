use napi::bindgen_prelude::Object;
use napi::Env;
use lymui::hwb::Hwb;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hwb {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("h", self.h)?;
        res.set("w", self.w)?;
        res.set("b", self.b)?;

        Ok(res)
    }
}

impl FromObject for Hwb {
    fn from_js_object(object: Object) -> Result<Self> {
        let h: f64 = object.get("h")?.ok_or_else(|| anyhow!("Unable to found h"))?;
        let w: f64 = object.get("w")?.ok_or_else(|| anyhow!("Unable to found w"))?;
        let b: f64 = object.get("b")?.ok_or_else(|| anyhow!("Unable to found b"))?;

        Ok(Hwb { h, w, b })
    }
}
