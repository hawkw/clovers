use super::{random_in_unit_sphere, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture};
use rand::prelude::ThreadRng;
pub struct Isotropic<'a> {
    albedo: &'a dyn Texture,
}

impl<'a> Isotropic<'a> {
    pub fn new(emission: &'a dyn Texture) -> Self
    where
        Self: 'a,
    {
        Isotropic { albedo: emission }
    }
}

impl<'a> Material<'a> for Isotropic<'a> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let scattered: Ray = Ray::new(hit_record.position, random_in_unit_sphere(rng), ray.time);
        let attenuation: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        Some((scattered, attenuation))
    }
}
