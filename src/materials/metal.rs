use super::{random_in_unit_sphere, reflect, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
#[derive(Clone)]
pub struct Metal<'a> {
    albedo: &'a dyn Texture<'a>,
    fuzz: Float,
}

impl<'a> Material<'a> for Metal<'a> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        let scattered: Ray = Ray::new(
            hit_record.position,
            reflected + self.fuzz * random_in_unit_sphere(rng),
            ray.time,
        );
        let attenuation: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl<'a> Metal<'a> {
    pub fn new(albedo: &'a dyn Texture, fuzz: Float) -> dyn Texture<'a> + 'a {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}
