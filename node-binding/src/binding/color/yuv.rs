use napi::{bindgen_prelude::Object};
use napi::{Env};
use lymui::yuv::Yuv;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Yuv {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("y", self.y)?;
        res.set("u", self.u)?;
        res.set("v", self.v)?;

        Ok(res)   
    }
}

impl FromObject for Yuv {
    fn from_js_object(object: Object) -> Result<Yuv> {
        let y: f64 = object.get("y")?.ok_or_else(|| anyhow!("Unable to found r"))?;
        let u: f64 = object.get("u")?.ok_or_else(|| anyhow!("Unable to found g"))?;
        let v: f64 = object.get("v")?.ok_or_else(|| anyhow!("Unable to found b"))?;

        Ok(Yuv { y, u, v })
    }
}
