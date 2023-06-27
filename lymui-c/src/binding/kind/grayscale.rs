use lymui::{
    grayscale::{GrayScale, Kind},
    rgb::{FromRgb, Rgb},
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum GrayscaleKind {
    Lightness,
    Average,
    Luminosity,
    BT709,
    BT2100,
}

impl From<GrayscaleKind> for Kind {
    fn from(k: GrayscaleKind) -> Self {
        match k {
            GrayscaleKind::Lightness => Kind::Lightness,
            GrayscaleKind::Average => Kind::Average,
            GrayscaleKind::Luminosity => Kind::Luminosity,
            GrayscaleKind::BT709 => Kind::BT709,
            GrayscaleKind::BT2100 => Kind::BT2100,
        }
    }
}

/// Get a grayscale from an rgb value and it's kind
///
/// # Arguments
///
/// * `rgb` - Rgb
/// * `kind` - GrayscaleKind
pub(crate) fn get_grayscale_from_rgb(rgb: Rgb, kind: GrayscaleKind) -> GrayScale {
    let g_kind = Kind::from(kind);

    GrayScale::from_rgb(rgb, g_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_rgb_to_grayscale() {
        let gscale = get_grayscale_from_rgb(
            Rgb {
                r: 10,
                g: 10,
                b: 10,
            },
            GrayscaleKind::BT709,
        );

        assert!(gscale.0 > 0);
    }
}
