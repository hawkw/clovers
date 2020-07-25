use crate::{Float, Vec3};

/// A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
#[derive(Copy, Clone)]
pub struct Ray {
    /// The origin position of the ray
    pub origin: Vec3,
    /// The direction of the ray
    pub direction: Vec3,
    /// The moment in time the ray exists in
    pub time: Float,
}

impl Ray {
    /// Creates a single Ray. A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
    pub fn new(origin: Vec3, direction: Vec3, time: Float) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn point_at_parameter(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}
