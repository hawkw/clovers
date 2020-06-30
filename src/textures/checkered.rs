use super::Texture;
use crate::{color::Color, Float, Vec3};

pub struct Checkered {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
    density: Float,
}

impl Checkered {
    pub fn new(
        texture1: Box<dyn Texture>,
        texture2: Box<dyn Texture>,
        density: Float,
    ) -> Checkered {
        Checkered {
            even: texture1,
            odd: texture2,
            density,
        }
    }
}

impl Texture for Checkered {
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
