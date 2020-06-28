use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        Material,
    },
    objects::{
        boxy::Boxy, constant_medium::ConstantMedium, moving_sphere::MovingSphere, rotate::RotateY,
        sphere::Sphere, translate::Translate,
    },
    perlin::Perlin,
    rect::{XYRect, XZRect, YZRect},
    textures::{noise_texture::NoiseTexture, solid_color::SolidColor, Texture},
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn load(mut rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;

    // Ground: lots of boxes
    let mut boxes1: HitableList = HitableList::new();
    let ground: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(
        Color::new(0.48, 0.83, 0.53),
    ))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w: Float = 100.0;
            let x0: Float = -1000.0 + i as Float * w;
            let z0: Float = -1000.0 + j as Float * w;
            let y0: Float = 0.0;
            let x1: Float = x0 + w;
            let y1: Float = rng.gen_range(1.0, 101.0);
            let z1: Float = z0 + w;

            boxes1.hitables.push(Arc::new(Boxy::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    let mut world: HitableList = HitableList::new();

    world
        .hitables
        .push(Arc::new(boxes1.into_bvh(time_0, time_1, rng)));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        7.0, 7.0, 7.0,
    )))));
    world.hitables.push(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material =
        Lambertian::new(Arc::new(SolidColor::new(Color::new(0.7, 0.3, 0.1))));
    world.hitables.push(Arc::new(MovingSphere::new(
        center1,
        center2,
        time_0,
        time_1,
        50.0,
        Arc::new(moving_sphere_material),
    )));

    // clear glass sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    // half-matte metal sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(
            Arc::new(SolidColor::new(Color::new(0.8, 0.8, 0.9))),
            10.0,
        )),
    )));

    // blue glass sphere
    let boundary: Arc<dyn Hitable> = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.hitables.push(Arc::clone(&boundary));
    world.hitables.push(Arc::new(ConstantMedium::new(
        Arc::clone(&boundary),
        0.2,
        Arc::new(SolidColor::new(Color::new(0.2, 0.4, 0.9))),
    )));
    // overall boundary sphere, big and misty inside
    let boundary2 = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.hitables.push(Arc::new(ConstantMedium::new(
        Arc::new(boundary2),
        0.0001,
        Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    )));

    // noise / marble sphere
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(Perlin::new(256, rng), 2.0));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // Sphere-rasterized pseudo-box
    let mut boxes2: HitableList = HitableList::new();
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(
        Color::new(0.73, 0.73, 0.73),
    ))));
    let num_spheres = 1000;
    for _j in 0..num_spheres {
        boxes2.hitables.push(Arc::new(Sphere::new(
            Vec3::new(
                rng.gen_range(0.0, 165.0),
                rng.gen_range(0.0, 165.0),
                rng.gen_range(0.0, 165.0),
            ),
            10.0,
            Arc::clone(&white),
        )));
    }

    world.hitables.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(boxes2.into_bvh(time_0, time_1, rng)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

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
    Scene::new(world, camera, time_0, time_1, background, rng)
}
