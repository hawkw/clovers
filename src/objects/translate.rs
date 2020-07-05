use crate::{
    hitable::{HitRecord, Hitable, AABB},
    ray::Ray,
    Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

pub struct Translate {
    object: Arc<Hitable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<Hitable>, offset: Vec3) -> Hitable {
        Hitable::Translate(Translate {
            object: Arc::clone(&object),
            offset,
        })
    }

    pub fn hit(
        &self,
        ray: &crate::ray::Ray,
        distance_min: crate::Float,
        distance_max: crate::Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        let moved_ray: Ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&moved_ray, distance_min, distance_max, rng) {
            // Didn't hit anything, return None
            None => None,
            // Hit something, adjust the position and normal
            Some(mut hit_record) => {
                hit_record.position += self.offset;
                hit_record.set_face_normal(&moved_ray, hit_record.normal);
                return Some(hit_record);
            }
        }
    }

    pub fn bounding_box(&self, t0: crate::Float, t1: crate::Float) -> Option<AABB> {
        let object_bounding_box = self.object.bounding_box(t0, t1);
        match object_bounding_box {
            Some(aabb) => return Some(AABB::new(aabb.min + self.offset, aabb.max + self.offset)),
            None => {
                return None;
            }
        }
    }
}
