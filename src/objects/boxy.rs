use crate::{
    hitable::{HitRecord, Hitable, HitableList, AABB},
    materials::Material,
    ray::Ray,
    rect::{XYRect, XZRect, YZRect},
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

// Avoid keyword clash
pub struct Boxy<'a> {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: HitableList,
    material: &'a dyn Material,
}

impl<'a> Boxy<'a> {
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Box<dyn Material>) -> Boxy<'a> {
        let mut sides = HitableList::new();
        sides.hitables.push(Box::new(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_1.z, &*material,
        )));
        sides.hitables.push(Box::new(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_0.z, &*material,
        )));

        sides.hitables.push(Box::new(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_1.y, &*material,
        )));
        sides.hitables.push(Box::new(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_0.y, &*material,
        )));

        sides.hitables.push(Box::new(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_1.x, &*material,
        )));
        sides.hitables.push(Box::new(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_0.x, &*material,
        )));

        Boxy {
            corner_0,
            corner_1,
            sides,
            material: &*material,
        }
    }
}

impl<'a> Hitable for Boxy<'a> {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max, rng)
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
