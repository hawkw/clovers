use super::{reflect, Material};
use crate::{
    color::Color, hitable::HitRecord, random::random_in_unit_sphere, ray::Ray, textures::Texture,
    Float, Vec3,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Metal {
    albedo: Texture,
    fuzz: Float,
}

impl Metal {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
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

    pub fn new(albedo: Texture, fuzz: Float) -> Material {
        Material::Metal(Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        })
    }
}
