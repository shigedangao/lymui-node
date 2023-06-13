use mapping::{
    rgb::RgbMapping,
    xyz::XyzMapping,
    grayscale::GrayscaleMapping
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

mod mapping;

#[napi]
pub fn get_any_rgb_compatible_color(obj: Object, input: RgbMapping, out: RgbMapping, env: Env) -> Result<Object> {
    let rgb = input.get_rgb_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let generated = out.create_color_from_rgb_input(rgb, env)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(generated)
}

#[napi]
pub fn get_any_xyz_compatible_color(obj: Object, input: XyzMapping, out: XyzMapping, env: Env) -> Result<Object> {
    let xyz = input.get_xyz_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let generated = out.create_color_from_xyz_input(xyz, env)?;

    Ok(generated)
}

#[napi]
pub fn get_grayscale_from_rgb(obj: Object, kind: GrayscaleMapping, env: Env) -> Result<Object> {
    let gscale = kind.into_grayscale(obj, env)?;

    Ok(gscale)
}
