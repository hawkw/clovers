use crate::{
    color::Color,
    pdf::{HitablePDF, MixturePDF},
    ray::Ray,
    scenes::Scene,
    Float, SHADOW_EPSILON,
};
use rand::prelude::*;

/// The main coloring function
pub fn colorize(ray: &Ray, scene: &Scene, depth: u32, max_depth: u32, rng: ThreadRng) -> Color {
    if depth > max_depth {
        // Ray bounce limit reached, return background_color
        return scene.background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match scene.objects.hit(&ray, SHADOW_EPSILON, Float::MAX, rng) {
        // If the ray hits nothing, return the background color.
        None => scene.background_color,

        // Hit something
        Some(hit_record) => {
            let emitted: Color = hit_record.material.emit(
                ray,
                &hit_record,
                hit_record.u,
                hit_record.v,
                hit_record.position,
            );

            // DEBUG
            if emitted.r.is_nan() {
                println!("emitted.r was NaN");
            }
            if emitted.g.is_nan() {
                println!("emitted.g was NaN");
            }
            if emitted.b.is_nan() {
                println!("emitted.b was NaN");
            }

            // Do we scatter?
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // No scatter, emit only
                None => emitted,
                // Got a scatter
                Some(scatter_record) => {
                    match scatter_record.material_type {
                        // If we hit a specular, return a specular ray
                        crate::materials::MaterialType::Specular => {
                            let recurse = colorize(
                                &scatter_record.specular_ray.unwrap(), // should always have a ray at this point
                                scene,
                                depth + 1,
                                max_depth,
                                rng,
                            );
                            // DEBUG
                            if recurse.r.is_nan() {
                                println!("recurse.r was NaN")
                            }
                            if recurse.g.is_nan() {
                                println!("recurse.g was NaN")
                            }
                            if recurse.b.is_nan() {
                                println!("recurse.b was NaN")
                            }

                            scatter_record.attenuation * recurse
                        }
                        crate::materials::MaterialType::Diffuse => {
                            // Use a probability density function to figure out where to scatter a new ray
                            let light_ptr =
                                HitablePDF::new(&scene.priority_objects, hit_record.position);
                            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

                            let scattered =
                                Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
                            // DEBUG
                            if scattered.direction.x.is_nan() {
                                println!("scattered.direction.x was NaN");
                            }
                            if scattered.direction.y.is_nan() {
                                println!("scattered.direction.y was NaN");
                            }
                            if scattered.direction.z.is_nan() {
                                println!("scattered.direction.z was NaN");
                            }

                            let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);
                            // DEBUG
                            if pdf_val.is_nan() {
                                println!("pdf_val was NaN"); // TODO: seems to be at least one of the NaN sources
                            }

                            // recurse
                            let recurse = colorize(&scattered, scene, depth + 1, max_depth, rng);

                            // DEBUG
                            // if recurse.r.is_nan() {
                            //     println!("recurse.r was NaN")
                            // }
                            // if recurse.g.is_nan() {
                            //     println!("recurse.g was NaN")
                            // }
                            // if recurse.b.is_nan() {
                            //     println!("recurse.b was NaN")
                            // }

                            // DEBUG
                            if scatter_record.attenuation.r.is_nan() {
                                println!("scatter_record.attenuation.r was NaN")
                            }
                            if scatter_record.attenuation.g.is_nan() {
                                println!("scatter_record.attenuation.g was NaN")
                            }
                            if scatter_record.attenuation.b.is_nan() {
                                println!("scatter_record.attenuation.b was NaN")
                            }

                            let scattering_pdf = hit_record.material.scattering_pdf(
                                ray,
                                &hit_record,
                                &scattered,
                                rng,
                            );
                            // DEBUG
                            if scattering_pdf.is_nan() {
                                println!("scattering_pdf was NaN")
                            }

                            // Blend it all together
                            emitted
                                + scatter_record.attenuation * scattering_pdf * recurse / pdf_val
                        }
                    }
                }
            }
        }
    }
}
