use super::Texture;
use crate::{color::Color, Float, Vec3};

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl<'a> SolidColor {
    pub fn new(color: Color) -> dyn Texture<'a> + 'a {
        SolidColor { color }
    }
}

impl<'a> Texture<'a> for SolidColor {
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        self.color
    }
}
