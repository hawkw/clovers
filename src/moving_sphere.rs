use crate::{hitable::AABB, Float, HitRecord, Hitable, Material, Ray, Vec3, PI};
use std::sync::Arc;

pub struct MovingSphere {
    center_0: Vec3,
    center_1: Vec3,
    time_0: Float,
    time_1: Float,
    radius: Float,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        time_0: Float,
        time_1: Float,
        radius: Float,
        material: Arc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: Float) -> Vec3 {
        return self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0))
                * (self.center_1 - self.center_0);
    }

    // Returns the U,V surface coordinates of a hitpoint
    pub fn get_uv(&self, hit_position: Vec3, time: Float) -> (Float, Float) {
        let translated: Vec3 = (hit_position - self.center(time)) / self.radius;
        let phi: Float = hit_position.z.atan2(hit_position.x);
        let theta: Float = hit_position.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // First possible root
            let distance = (-b - discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center(ray.time)) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                    u,
                    v,
                    material: Arc::clone(&self.material),
                });
            }
            // Second possible root
            let distance = (-b + discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center(ray.time)) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                    u,
                    v,
                    material: Arc::clone(&self.material),
                });
            }
        }
        None
    }

    // TODO: might need to return Option<T> ?
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        let box0: AABB = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1: AABB = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}
