use napi::bindgen_prelude::Object;
use napi::Env;
use lymui::ycbcr::Ycbcr;
use anyhow::{Result, anyhow};
use super::{IntoObject, FromObject};

impl IntoObject for Ycbcr {
    fn as_js_object(&self, env: Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("y", self.y)?;
        res.set("cb", self.cb)?;
        res.set("cr", self.cr)?;

        Ok(res)
    }
}

impl FromObject for Ycbcr {
    fn from_js_object(object: Object) -> Result<Self> {
        let y: u8 = object.get("y")?.ok_or_else(|| anyhow!("Unable to found y"))?;
        let cb: u8 = object.get("cb")?.ok_or_else(|| anyhow!("Unable to found cb"))?;
        let cr: u8 = object.get("cr")?.ok_or_else(|| anyhow!("Unable to found cr"))?;

        Ok(Ycbcr { y, cb, cr })
    }
}
