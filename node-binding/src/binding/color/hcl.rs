use lymui::prelude::Hcl;
use napi::bindgen_prelude::Object;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Hcl {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("h", self.h)?;
        res.set("c", self.c)?;
        res.set("l", self.l)?;

        Ok(res)
    }
}

impl FromObject for Hcl {
    fn from_js_object(object: Object) -> Result<Hcl>  {
        let h: f64 = object.get("h")?.ok_or_else(|| anyhow!("Unable to found h"))?;
        let c: f64 = object.get("c")?.ok_or_else(|| anyhow!("Unable to found c"))?;
        let l: f64 = object.get("l")?.ok_or_else(|| anyhow!("Unable to found l"))?;

        Ok(Hcl { h, c, l })
    }
}
