use super::{random_unit_vector, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Vec3};
use rand::prelude::ThreadRng;

#[derive(Clone)]
pub struct Lambertian<'a> {
    albedo: &'a dyn Texture<'a>,
}

impl<'a> Material<'a> for Lambertian<'a> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = hit_record.normal + random_unit_vector(rng);
        let scattered = Ray::new(hit_record.position, scatter_direction, ray.time);
        let attenuation = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        Some((scattered, attenuation))

        // old backup
        //     let target = hit_record.position + hit_record.normal + random_unit_vector(rng);
        //     let scattered: Ray = Ray::new(hit_record.position, target - hit_record.position, ray.time);
        //     let attenuation: Color = self
        //         .albedo
        //         .color(hit_record.u, hit_record.v, hit_record.position);
        //     Some((scattered, attenuation))
    }
}

impl<'a> Lambertian<'a> {
    pub fn new(albedo: &'a dyn Texture) -> dyn Texture<'a> + 'a {
        Lambertian { albedo }
    }
}
