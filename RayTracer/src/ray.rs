use std::f64::INFINITY;

use crate::hit_list::HittableList;
use crate::interval::Interval;
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::hit::{HitRecord, Hittable};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, tm: f64) -> Self {
        Ray {
            origin,
            direction,
            tm,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }
}

impl Ray {
    pub fn ray_color(&self, background: Vec3, depth: u32, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        
        // let mut rec = HitRecord::default();
        // if world.hit(&self, Interval::with_values(0.001, INFINITY), &mut rec) {
        //     let direction = rec.normal + Vec3::random_unit_vector();
        //     let r = Ray::new(rec.p, direction);
        //     return r.ray_color(depth - 1, &world) * 0.1;
        // }

        let mut rec = HitRecord::default();
        if world.hit(&self, Interval::with_values(0.001, INFINITY), &mut rec) {
            if let Some(mat) = rec.mat.as_ref() {
                let color_from_emission = mat.emitted(rec.u, rec.v, &rec.p);
                if let Some((attenuation, scattered)) = mat.scatter(&self, &rec) {
                    return attenuation * scattered.ray_color(background, depth - 1, world) + color_from_emission;
                }
                return color_from_emission;
            }
        }

        background
    }
}
