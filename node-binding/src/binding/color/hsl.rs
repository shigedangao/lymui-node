use napi::{bindgen_prelude::Object};
use napi::{Env};
use lymui::hsl::Hsl;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hsl {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("h", self.h)?;
        res.set("s", self.s)?;
        res.set("l", self.l)?;

        Ok(res)   
    }
}

impl FromObject for Hsl {
    fn from_js_object(object: Object) -> Result<Hsl> {
        let h: f64 = object.get("h")?.ok_or_else(|| anyhow!("Unable to found h"))?;
        let s: f64 = object.get("s")?.ok_or_else(|| anyhow!("Unable to found s"))?;
        let l: f64 = object.get("l")?.ok_or_else(|| anyhow!("Unable to found l"))?;

        Ok(Hsl { h, s, l })
    }
}
