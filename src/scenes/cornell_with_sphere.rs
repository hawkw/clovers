use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    materials::{Dielectric, DiffuseLight, Lambertian, Material},
    objects::Sphere,
    rect::{XYRect, XZRect, YZRect},
    textures::SolidColor,
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;
pub fn load<'a>(rng: ThreadRng) -> Scene<'a> {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    // Cornell box

    world.hitables.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(
            0.12, 0.45, 0.15,
        ))))),
    )));
    world.hitables.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(
            0.65, 0.05, 0.05,
        ))))),
    )));
    world.hitables.push(Box::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::new(DiffuseLight::new(Box::new(SolidColor::new(Color::new(
            7.0, 7.0, 7.0,
        ))))),
    )));
    world.hitables.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(
            0.73, 0.73, 0.73,
        ))))),
    )));
    world.hitables.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(
            0.73, 0.73, 0.73,
        ))))),
    )));
    world.hitables.push(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(
            0.73, 0.73, 0.73,
        ))))),
    )));

    // glass sphere
    let sphere: Box<dyn Hitable> = Box::new(Sphere::new(
        Vec3::new(278.0, 278.0, 278.0),
        120.0,
        Box::new(Dielectric::new(1.5)),
    ));
    world.hitables.push(sphere);

    let camera_position: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    let camera_target: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 40.0;
    let aspect_ratio: Float = WIDTH as Float / HEIGHT as Float;
    let aperture: Float = 0.0;
    let focus_distance: Float = 10.0;
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
    return Scene::new(world, camera, time_0, time_1, background, rng);
}
