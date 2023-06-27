use anyhow::Result;
use lymui::generator::{shade::Shade, tint::Tint, GeneratorOps};
use lymui::rgb::Rgb;

#[derive(Debug)]
#[repr(C)]
pub struct ColorMixture {
    pub len: usize,
    pub ptr: *const RgbMixture,
}

#[derive(Debug)]
#[repr(C)]
pub struct RgbMixture {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Rgb> for RgbMixture {
    fn from(v: Rgb) -> Self {
        RgbMixture {
            r: v.r,
            g: v.g,
            b: v.b,
        }
    }
}

impl ColorMixture {
    /// Create a new color mixture
    ///
    /// # Arguments
    ///
    /// * `rgb` - Rgb
    /// * `factor` - f64
    /// * `shade` - bool
    pub fn new(rgb: Rgb, factor: f64, shade: bool) -> Result<Self> {
        let generated: Vec<Rgb> = match shade {
            true => Shade::compute(rgb, factor)?.0,
            false => Tint::compute(rgb, factor)?.0,
        };

        let mut mixture = generated
            .into_iter()
            .map(RgbMixture::from)
            .collect::<Vec<_>>();

        mixture.shrink_to_fit();
        assert!(mixture.len() == mixture.capacity());

        let len = mixture.len();
        let ptr = mixture.as_ptr();

        std::mem::forget(mixture);

        Ok(ColorMixture { len, ptr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_generate_shade() {
        let shade = ColorMixture::new(
            Rgb {
                r: 10,
                g: 10,
                b: 255,
            },
            0.1,
            true,
        );

        assert!(shade.is_ok());
    }

    #[test]
    fn expect_to_generate_tint() {
        let tint = ColorMixture::new(
            Rgb {
                r: 10,
                g: 10,
                b: 255,
            },
            0.1,
            false,
        );

        assert!(tint.is_ok());
    }
}
