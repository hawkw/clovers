use super::Material;
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
pub struct DiffuseLight<'a> {
    emit: &'a dyn Texture,
}

impl<'a> Material<'a> for DiffuseLight<'a> {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: Float, v: Float, position: Vec3) -> Color {
        self.emit.color(u, v, position)
    }
}

impl<'a> DiffuseLight<'a> {
    pub fn new(emission: &'a dyn Texture) -> Self {
        DiffuseLight { emit: emission }
    }
}
