pub mod checkered;
pub mod noise_texture;
pub mod solid_color;

pub use checkered::*;
pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};

pub trait Texture<'a>: Sync + Send + 'a {
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color;
}
