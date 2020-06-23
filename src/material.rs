use crate::{Float, HitRecord, Ray, ThreadRng, Vec3};
use rand::prelude::*;

// Internal helper
pub fn random_in_unit_sphere(mut rng: ThreadRng) -> Vec3 {
  let mut position: Vec3;
  loop {
    position = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    if position.magnitude_squared() >= 1.0 {
      return position;
    }
  }
}

pub trait Material: Sync + Send + Sized {
  /// Returns `None`, if the ray gets absorbed.
  /// Returns `Some(scattered, attenuation)`, if the ray gets scattered
  fn scatter(&self, ray: Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
  albedo: Vec3,
}

impl Material for Lambertian {
  fn scatter(&self, mut ray: Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Vec3)> {
    let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
    ray.origin = hit_record.position;
    ray.direction = target - hit_record.position;
    // scattered, attenuation
    Some((ray, self.albedo))
  }
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Self {
    Lambertian { albedo }
  }
}

pub struct Metal {
  albedo: Vec3,
}

// Reflects a vector based on a given normal, returning the new vector.
fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
  vector - 2.0 * vector.dot(&normal) * normal
}

impl Material for Metal {
  fn scatter(
    &self,
    mut ray: Ray,
    hit_record: &HitRecord,
    _rng: ThreadRng,
  ) -> Option<(Ray, Vec3)> {
    let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
    // scatter
    ray.origin = hit_record.position;
    ray.direction = reflected;
    if ray.direction.dot(&hit_record.normal) > 0.0 {
      // scattered, attenuation
      Some((ray, self.albedo))
    } else {
      None
    }
  }
}

impl Metal {
  pub fn new(albedo: Vec3) -> Self {
    Metal { albedo }
  }
}

fn refract(vector: Vec3, normal: Vec3, ni_over_nt: Float) -> Option<Vec3> {
  let uv: Vec3 = vector.normalize();
  let dt: Float = uv.dot(&normal);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    let refracted: Vec3 = ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt();
    Some(refracted)
  } else {
    None
  }
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
  let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
  let r0 = r0 * r0;
  r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

pub struct Dielectric {
  refractive_index: Float,
}

impl Material for Dielectric {
  fn scatter(
    &self,
    mut ray: Ray,
    hit_record: &HitRecord,
    mut rng: ThreadRng,
  ) -> Option<(Ray, Vec3)> {
    let attenuation: Vec3 = Vec3::new(1.0, 1.0, 1.0); // Glass does not attenuate
    let ni_over_nt: Float;
    let reflect_probability: Float;
    let cosine: Float;
    let outward_normal: Vec3;
    let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0); // TODO: fix this, shouldn't be zero. see below
    
    // TODO: understand and annotate this mess of if-else clauses
    // TODO: cleanup
    if ray.direction.dot(&hit_record.normal) > 0.0 {
      outward_normal = -hit_record.normal;
      ni_over_nt = self.refractive_index;
      cosine = self.refractive_index * ray.direction.dot(&hit_record.normal)
      / ray.direction.norm();
    } else {
      outward_normal = hit_record.normal;
      ni_over_nt = 1.0 / self.refractive_index;
      cosine = -(ray.direction.dot(&hit_record.normal)) / ray.direction.norm();
    }
    if let Some(new_refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
      refracted = new_refracted;
      reflect_probability = schlick(cosine, self.refractive_index);
    } else {
      reflect_probability = 1.0;
    }
    if rng.gen::<Float>() < reflect_probability {
      ray.origin = hit_record.position;
      ray.direction = reflect(ray.direction, hit_record.normal);
    } else {
      ray.origin = hit_record.position;
      ray.direction = refracted; // TODO: fix this. should be refracted. see above
    }
    Some((ray, attenuation))
  }
}

impl Dielectric {
  pub fn new(refractive_index: Float) -> Self {
    Dielectric { refractive_index }
  }
}
