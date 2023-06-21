use lymui::{
    js::{FromJsObject, IntoJsObject},
    prelude::*,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum XyzMapping {
    Argb,
    Hcl,
    Hlab,
    LchLab,
    LchUv,
    Luv,
    OkLab,
    OkLch,
    REC709,
    REC2020,
    REC2100,
    SRgb,
    Xyy,
}

impl XyzMapping {
    /// Create an xyz from the color mapping
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `object` - Object
    pub(crate) fn get_xyz_from_mapping(&self, object: Object) -> Result<Xyz> {
        let xyz = match self {
            Self::Argb => Xyz::from(Xyy::from_js_object(object)?),
            Self::Hcl => Xyz::from(Hcl::from_js_object(object)?),
            Self::Hlab => Xyz::from(Hlab::from_js_object(object)?),
            Self::LchLab => Xyz::from(Lchlab::from_js_object(object)?),
            Self::LchUv => Xyz::from(Lchuv::from_js_object(object)?),
            Self::Luv => Xyz::from(Luv::from_js_object(object)?),
            Self::OkLab => Xyz::from(OkLab::from_js_object(object)?),
            Self::OkLch => Xyz::from(OkLch::from_js_object(object)?),
            Self::REC709 => Xyz::from(Rec709::from_js_object(object)?),
            Self::REC2020 => Xyz::from(Rec2020::from_js_object(object)?),
            Self::REC2100 => Xyz::from(Rec2100::from_js_object(object)?),
            Self::SRgb => Xyz::from(Srgb::from_js_object(object)?),
            Self::Xyy => Xyz::from(Xyy::from_js_object(object)?),
        };

        Ok(xyz)
    }

    /// Create a color from an xyz input
    ///
    /// # Arguments
    ///
    /// * `self` - &Self
    /// * `xyz` - Xyz
    /// * `env` - Env
    pub(crate) fn create_color_from_xyz_input(&self, xyz: Xyz, env: Env) -> Result<Object> {
        let res = match self {
            Self::Argb => Argb::from(xyz).into_js_object(env),
            Self::Hcl => Hcl::from(xyz).into_js_object(env),
            Self::Hlab => Hlab::from(xyz).into_js_object(env),
            Self::LchLab => Lchlab::from(xyz).into_js_object(env),
            Self::LchUv => Lchuv::from(xyz).into_js_object(env),
            Self::Luv => Luv::from(xyz).into_js_object(env),
            Self::OkLab => OkLab::from(xyz).into_js_object(env),
            Self::OkLch => OkLch::from(xyz).into_js_object(env),
            Self::REC709 => Rec709::from(xyz).into_js_object(env),
            Self::REC2020 => Rec2020::from(xyz).into_js_object(env),
            Self::REC2100 => Rec2100::from(xyz).into_js_object(env),
            Self::SRgb => Srgb::from(xyz).into_js_object(env),
            Self::Xyy => Xyy::from(xyz).into_js_object(env),
        }?;

        Ok(res)
    }
}
