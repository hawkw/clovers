use crate::{
    hitable::{HitRecord, Hitable, HitableList, AABB},
    material::Material,
    rect::{XYRect, XZRect, YZRect},
    Vec3,
};

// Avoid keyword clash
pub struct Boxy<'a> {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: HitableList<'a>,
    material: &'a dyn Material,
}

impl<'a> Boxy<'a> {
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: &'a dyn Material) -> Boxy<'a> {
        let mut sides = HitableList::new();
        sides.hitables.push(&XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_1.z, material,
        ));
        sides.hitables.push(&XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_0.z, material,
        ));

        sides.hitables.push(&XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_1.y, material,
        ));
        sides.hitables.push(&XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_0.y, material,
        ));

        sides.hitables.push(&YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_1.x, material,
        ));
        sides.hitables.push(&YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_0.x, material,
        ));

        Boxy {
            corner_0,
            corner_1,
            sides,
            material,
        }
    }
}

impl<'a> Hitable for Boxy<'a> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        distance_min: crate::Float,
        distance_max: crate::Float,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max)
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
