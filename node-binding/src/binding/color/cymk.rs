use napi::{bindgen_prelude::Object};
use anyhow::{Result, anyhow};
use lymui::cymk::Cymk;
use super::{IntoObject, FromObject};

impl IntoObject for Cymk {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("c", self.c)?;
        res.set("y", self.y)?;
        res.set("m", self.m)?;
        res.set("k", self.k)?;

        Ok(res)
    }
}

impl FromObject for Cymk {
    fn from_js_object(object: Object) -> Result<Cymk> {
        let c: f64 = object.get("c")?.ok_or_else(|| anyhow!("Unable to found c value"))?;
        let y: f64 = object.get("y")?.ok_or_else(|| anyhow!("Unable to found y value"))?;
        let m: f64 = object.get("m")?.ok_or_else(|| anyhow!("Unable to found m value"))?;
        let k: f64 = object.get("k")?.ok_or_else(|| anyhow!("Unable to found k value"))?;

        Ok(Cymk { c, y, m, k })
    }
}
