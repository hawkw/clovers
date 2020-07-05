use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{DiffuseLight, Lambertian, Material},
    objects::Sphere,
    objects::XYRect,
    textures::{SolidColor, Texture},
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn load(width: u32, height: u32, rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: Arc<Hitable>List = HitableList::new();

    let texture: Texture = SolidColor::new(Color::new(0.3, 0.2, 0.1));
    let texture2: Texture = SolidColor::new(Color::new(0.1, 0.2, 0.3));

    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(texture),
    )));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(texture2),
    )));

    let difflight: Material = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));
    world
        .hitables
        .push(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let camera_position: Vec3 = Vec3::new(20.0, 5.0, 2.0);
    let camera_target: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 20.0;
    let aspect_ratio: Float = width as Float / height as Float;

    let aperture: Float = 0.0;
    let focus_distance: Float = 1.0;
    let camera = Camera::new(
        camera_position,
        camera_target,
        camera_up,
        fov,
        aspect_ratio,
        aperture,
        focus_distance,
        time_0,
        time_1,
    );

    let background: Color = Color::new(0.0, 0.0, 0.0); // Black background = only lit by the light, no ambient

    Scene::new(world, camera, time_0, time_1, background, rng)
}
