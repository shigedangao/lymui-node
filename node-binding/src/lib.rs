use lymui::{
    generator::GeneratorOps,
    generator::{shade::Shade, tint::Tint},
    js::IntoJsObject,
};
use mapping::{grayscale::GrayscaleMapping, rgb::RgbMapping, xyz::XyzMapping};
use napi::bindgen_prelude::*;
use napi_derive::napi;

mod mapping;

#[napi]
pub fn get_any_rgb_compatible_color(
    obj: Object,
    input: RgbMapping,
    out: RgbMapping,
    env: Env,
) -> Result<Object> {
    let rgb = input
        .get_rgb_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let generated = out
        .create_color_from_rgb_input(rgb, env)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(generated)
}

#[napi]
pub fn get_any_xyz_compatible_color(
    obj: Object,
    input: XyzMapping,
    out: XyzMapping,
    env: Env,
) -> Result<Object> {
    let xyz = input
        .get_xyz_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let generated = out.create_color_from_xyz_input(xyz, env)?;

    Ok(generated)
}

#[napi]
pub fn get_grayscale_from_rgb(obj: Object, kind: GrayscaleMapping, env: Env) -> Result<Object> {
    let gscale = kind.into_grayscale(obj, env)?;

    Ok(gscale)
}

#[napi]
pub fn generate_shade_from_rgb_compatible(
    obj: Object,
    input: RgbMapping,
    factor: f64,
    env: Env,
) -> Result<Object> {
    let rgb = input
        .get_rgb_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let shade = Shade::compute(rgb, factor)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    shade.into_js_object(env)
}

#[napi]
pub fn generate_tint_from_rgb_compatible(
    obj: Object,
    input: RgbMapping,
    factor: f64,
    env: Env,
) -> Result<Object> {
    let rgb = input
        .get_rgb_from_mapping(obj)
        .map_err(|err| Error::new(Status::InvalidArg, err.to_string()))?;

    let tint = Tint::compute(rgb, factor)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    tint.into_js_object(env)
}
