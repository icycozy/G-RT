use crate::hit::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Option<Rc<dyn Material>>) -> Self {
        Sphere {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin().clone();
        let a = r.direction().squared_length();
        let h = Vec3::dot(r.direction(), oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.mat = self.mat.clone();
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}