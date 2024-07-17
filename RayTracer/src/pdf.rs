use crate::onb::ONB;
use crate::vec3::Vec3;
use crate::hit_list::HittableList;
use crate::hit::{Hittable};
type Point3 = Vec3;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl SpherePdf {
    pub fn new() -> Self {
        SpherePdf
    }
}

impl Pdf for SpherePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: ONB,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        let uvw = ONB::build_from_w(&w);
        CosinePdf { uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.unit().dot(self.uvw.w());
        f64::max(0.0, cosine_theta / std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec3(&Vec3::random_cosine_direction())
    }
}

#[derive(Clone)]
pub struct HittablePdf {
    objects: HittableList,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: HittableList, origin: Point3) -> Self {
        HittablePdf { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, &direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p0: Arc<dyn Pdf + Sync + Send>,
    p1: Option<Arc<dyn Pdf + Sync + Send>>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf + Sync + Send>, p1: Option<Arc<dyn Pdf + Sync + Send>>) -> Self {
        MixturePdf { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        match &self.p1 {
            Some(p1) => 0.5 * self.p0.value(direction) + 0.5 * p1.value(direction),
            None => self.p0.value(direction),
        }
    }

    fn generate(&self) -> Vec3 {
        if rand::random::<f64>() < 0.5 {
            self.p0.generate()
        } else {
            match &self.p1 {
                Some(p1) => p1.generate(),
                None => self.p0.generate(),
            }
        }
    }
}