use binding::mapping::Mapping;
use napi::bindgen_prelude::*;
use napi_derive::napi;

mod binding;

#[napi]
pub fn get_any_color(obj: Object, input: Mapping, out: Mapping, env: Env) -> Result<Object> {
    let rgb = input.get_rgb_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let generated = out.create_color_from_rgb_input(rgb, env)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(generated)
}
