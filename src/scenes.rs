use crate::{
    camera::Camera,
    color::Color,
    hitable::{BVHNode, HitableList},
    Float,
};
use rand::prelude::*;
use std::marker::PhantomData;

// pub mod cornell;
// pub mod cornell_with_boxes;
// pub mod cornell_with_smoke;
pub mod cornell_with_sphere;
// pub mod cornell_with_subsurface_sphere;
// pub mod final_scene;
// pub mod glass_spheres;
// pub mod metal_spheres;
// pub mod random_scene;
// pub mod simple_light_lambertian;
// pub mod simple_light_perlin;
// pub mod two_perlin_spheres;
// pub mod two_spheres;

pub struct Scene<'a> {
    pub world: BVHNode<'a>,
    pub camera: Camera,
    pub background: Color, // TODO: make into Texture or something?
}

impl<'a> Scene<'a> {
    fn new(
        world: HitableList<'a>,
        camera: Camera,
        time_0: Float,
        time_1: Float,
        background: Color,
        rng: ThreadRng,
    ) -> Scene<'a> {
        Scene {
            world: world.into_bvh(time_0, time_1, rng),
            camera,
            background,
        }
    }
}
