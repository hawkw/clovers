use super::Texture;
use crate::{color::Color, Float, Vec3};

pub struct Checkered<'a> {
    even: &'a dyn Texture,
    odd: &'a dyn Texture,
    density: Float,
}

impl<'a> Checkered<'a> {
    pub fn new(
        texture1: &'a dyn Texture,
        texture2: &'a dyn Texture,
        density: Float,
    ) -> Checkered<'a> {
        Checkered {
            even: texture1,
            odd: texture2,
            density,
        }
    }
}

impl<'a> Texture for Checkered<'a> {
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        let sines = (self.density * position.x).sin()
            * (self.density * position.y).sin()
            * (self.density * position.z).sin();
        if sines < 0.0 {
            return self.odd.color(u, v, position);
        } else {
            return self.even.color(u, v, position);
        }
    }
}
