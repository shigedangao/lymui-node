use lymui::prelude::Hlab;
use napi::bindgen_prelude::Object;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hlab {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("l", self.l)?;
        res.set("a", self.a)?;
        res.set("b", self.b)?;

        Ok(res)
    }
}

impl FromObject for Hlab {
    fn from_js_object(object: Object) -> Result<Hlab> {
        let l: f64 = object.get("l")?.ok_or_else(|| anyhow!("Unable to found l"))?;
        let a: f64 = object.get("a")?.ok_or_else(|| anyhow!("Unable to found a"))?;
        let b: f64 = object.get("b")?.ok_or_else(|| anyhow!("Unable to found b"))?;

        Ok(Hlab { l, a, b })
    }
}
