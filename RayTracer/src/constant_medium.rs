use std::rc::Rc;
use crate::material::{Isotropic, Material};
use crate::vec3::Vec3;
type Color = Vec3;
use crate::hit::{HitRecord, Hittable, HittableClone};
use crate::texture::Texture;
use crate::interval::{self, Interval};
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::rtweekend::random_double;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, tex: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Isotropic::with_texture(tex)),
        }
    }

    pub fn new_with_albedo(boundary: Box<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Isotropic::new(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enable_debug true.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && random_double(0.0, 1.0) < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, Interval::with_values(rec1.t + 0.0001, f64::INFINITY), &mut rec2) {
            return false;
        }

        if debugging {
            println!("t_min={}, t_max={}", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 1.0).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            println!("hit_distance = {}", hit_distance);
            println!("rec.t = {}", rec.t);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

impl HittableClone for ConstantMedium {
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}