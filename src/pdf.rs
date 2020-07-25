use crate::{hitable::Hitable, onb::ONB, random::random_cosine_direction, Float, Vec3, PI};
use rand::prelude::*;
use std::sync::Arc;

pub enum PDF<'a> {
    CosinePDF(CosinePDF),
    HitablePDF(HitablePDF<'a>),
    MixturePDF(MixturePDF<'a>),
    ZeroPDF(ZeroPDF),
}

impl<'a> PDF<'a> {
    pub fn value(&self, direction: Vec3, time: Float, rng: ThreadRng) -> Float {
        match self {
            PDF::CosinePDF(p) => p.value(direction, time, rng),
            PDF::HitablePDF(p) => p.value(direction, time, rng),
            PDF::MixturePDF(p) => p.value(direction, time, rng),
            PDF::ZeroPDF(p) => p.value(direction, time, rng),
        }
    }
    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(rng),
            PDF::HitablePDF(p) => p.generate(rng),
            PDF::MixturePDF(p) => p.generate(rng),
            PDF::ZeroPDF(p) => p.generate(rng),
        }
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl<'a> CosinePDF {
    pub fn new(w: Vec3) -> PDF<'a> {
        PDF::CosinePDF(CosinePDF {
            uvw: ONB::build_from_w(w),
        })
    }

    pub fn value(&self, direction: Vec3, _time: Float, _rng: ThreadRng) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        let pdf_value: Float;
        if cosine <= 0.0 {
            pdf_value = 0.0;
        } else {
            pdf_value = cosine / PI;
        }
        // DEBUG
        if pdf_value.is_nan() {
            println!("CosinePDF::value was NaN");
        }
        pdf_value
    }

    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

pub struct HitablePDF<'a> {
    origin: Vec3,
    hitable: &'a Hitable,
}

impl<'a> HitablePDF<'a> {
    pub fn new(hitable: &'a Hitable, origin: Vec3) -> PDF {
        PDF::HitablePDF(HitablePDF { origin, hitable })
    }

    pub fn value(&self, direction: Vec3, time: Float, rng: ThreadRng) -> Float {
        let value = self.hitable.pdf_value(self.origin, direction, time, rng);
        if value.is_nan() {
            dbg!(&value);
            match self.hitable {
                Hitable::Boxy(_) => println!("Boxy.value returned NaN"),
                Hitable::ConstantMedium(_) => println!("ConstantMedium.value returned NaN"),
                Hitable::MovingSphere(_) => println!("MovingSphere.value returned NaN"),
                Hitable::XZRect(_) => println!("XZRect.value returned NaN"),
                Hitable::XYRect(_) => println!("XYRect.value returned NaN"),
                Hitable::YZRect(_) => println!("YZRect.value returned NaN"),
                Hitable::RotateY(_) => println!("RotateY.value returned NaN"),
                Hitable::Sphere(_) => println!("Sphere.value returned NaN"),
                Hitable::Translate(_) => println!("Translate.value returned NaN"),
                Hitable::BVHNode(_) => println!("BVHNode.value returned NaN"),
                Hitable::HitableList(_) => println!("HitableList.value returned NaN"),
                Hitable::FlipFace(_) => println!("FlipFace.value returned NaN"),
            };
        }
        value
    }

    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        self.hitable.random(self.origin, rng)
    }
}

pub struct MixturePDF<'a> {
    // Arc to prevent infinite size
    pdf1: Arc<PDF<'a>>,
    pdf2: Arc<PDF<'a>>,
}

impl<'a> MixturePDF<'a> {
    pub fn new(pdf1: PDF<'a>, pdf2: PDF<'a>) -> PDF<'a> {
        PDF::MixturePDF(MixturePDF {
            pdf1: Arc::new(pdf1),
            pdf2: Arc::new(pdf2),
        })
    }

    pub fn value(&self, direction: Vec3, time: Float, rng: ThreadRng) -> Float {
        let value = 0.5 * self.pdf1.value(direction, time, rng)
            + 0.5 * self.pdf2.value(direction, time, rng);
        if value.is_nan() {
            dbg!(&value);
        }
        value
    }

    pub fn generate(&self, mut rng: ThreadRng) -> Vec3 {
        if rng.gen::<bool>() {
            self.pdf1.generate(rng)
        } else {
            self.pdf2.generate(rng)
        }
    }
}

// TODO: this is an ugly hack due to tutorial saying `srec.pdf_ptr = 0;` in 12.2 Handling Specular for Metal
pub struct ZeroPDF {}

impl<'a> ZeroPDF {
    pub fn new() -> PDF<'a> {
        PDF::ZeroPDF(ZeroPDF {})
    }

    pub fn value(&self, _direction: Vec3, _time: Float, _rng: ThreadRng) -> Float {
        0.0
    }

    pub fn generate(&self, _rng: ThreadRng) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
