use lymui::rgb::Rgb;

#[derive(Debug)]
#[repr(C)]
pub enum Color {
    Hex,
}

#[derive(Debug)]
#[repr(C)]
pub struct TargetColor<T> where T: From<Rgb> {
    c: T
}

impl<T> TargetColor<T> where T: From<Rgb> {
    pub fn from_kind(c: Color, rgb: Rgb) -> Self {
        match c {
            Color::Hex => {
                TargetColor {
                    c: T::from(rgb)
                }
            }
        }
    }
}