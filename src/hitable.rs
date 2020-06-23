use crate::{Float, Material, Ray, Vec3};

pub struct HitRecord<'a> {
    pub distance: Float,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hitable: Sync + Send {
    /// The main function for checking whether an object is hit by a ray. If an object is hit, returns Some(HitRecord)
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord>;
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in self.hitables.iter() {
            if let Some(record) = hitable.hit(&ray, distance_min, closest) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
