use lymui::xyz::Xyz;
use anyhow::{Result, anyhow};
use napi::bindgen_prelude::Object;
use super::{IntoObject, FromObject};

impl IntoObject for Xyz {
    fn as_js_object(&self, env: napi::Env) -> Result<Object> {
        let mut res = env.create_object()?;
        res.set("x", self.x)?;
        res.set("y", self.y)?;
        res.set("z", self.z)?;

        Ok(res)
    }
}

impl FromObject for Xyz {
    fn from_js_object(object: Object) -> Result<Xyz> {
        let x: f64 = object.get("x")?.ok_or_else(|| anyhow!("Unable to found x"))?;
        let y: f64 = object.get("y")?.ok_or_else(|| anyhow!("Unable to found y"))?;
        let z: f64 = object.get("z")?.ok_or_else(|| anyhow!("Unable to found z"))?;

        Ok(Xyz { x, y, z })
    }
}
