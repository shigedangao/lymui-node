use napi::{bindgen_prelude::Object};
use anyhow::{Result, anyhow};
use lymui::hex::Hex;
use super::{IntoObject, FromObject};

impl IntoObject for Hex {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("hex", self.to_string())?;

        Ok(res)
    }
}

impl FromObject for Hex {
    fn from_js_object(object: Object) -> Result<Hex> {
        let hex: &str = object.get("hex")?.ok_or_else(|| anyhow!("Unable to found hex value"))?;

        Ok(hex.to_string())
    }
}
