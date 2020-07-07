use crate::{
    color::Color,
    colorize::colorize,
    hitable::HitableList,
    materials::{Dielectric, DiffuseLight},
    objects::{Sphere, XZRect},
    ray::Ray,
    scenes,
    textures::SolidColor,
    Float, Vec3,
};
use image::{ImageBuffer, ImageResult, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

/// The main drawing function, returns an `ImageResult`.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
) -> ImageResult<ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    let rng = rand::thread_rng();
    let scene = scenes::cornell_book3_final::load(width, height, rng);
    let background_color: Color = scene.background;

    // Progress bar
    let pixels = (width * height) as u64;
    let bar = ProgressBar::new(pixels);
    bar.set_draw_delta(pixels / 1000);
    bar.set_style(ProgressStyle::default_bar().template(
        "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
    ));

    // TODO: remove temporary
    let small_light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));
    let small_light_obj = XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, small_light);
    let sphere = Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, Dielectric::new(1.5));
    let mut lights = HitableList::new();
    lights.add(small_light_obj);
    lights.add(sphere);
    let lights = lights.into_hitable(); // TODO: fixme, silly
    let lights = Arc::new(lights);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            let mut u: Float;
            let mut v: Float;
            let mut ray: Ray;

            // Multisampling for antialiasing
            for _sample in 0..samples {
                u = (x as Float + rng.gen::<Float>()) / width as Float;
                v = (y as Float + rng.gen::<Float>()) / height as Float;
                ray = scene.camera.get_ray(u, v, rng);
                color += colorize(
                    &ray,
                    background_color,
                    &scene.world,
                    Arc::clone(&lights), // TODO: fixme, this is silly
                    0,
                    max_depth,
                    rng,
                );
            }
            color /= samples as Float;

            color = color.gamma_correction(gamma);
            *pixel = color.to_rgb_u8();

            bar.inc(1);
        });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    Ok(img)
}
