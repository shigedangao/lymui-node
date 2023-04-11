use color::{Color, TargetColor};
use lymui::rgb::Rgb;

pub mod color;

pub extern "C" fn convert_color_from_rgb<T>(from: Color) -> TargetColor<T> where T: From<Rgb> {
    TargetColor::from_kind(from, Rgb::default())
}