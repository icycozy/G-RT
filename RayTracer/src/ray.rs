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
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }
}

impl Ray {
    pub fn ray_color(&self, depth: u32, world: &HittableList) -> Vec3 {
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
            if let Some((attenuation, scattered)) = rec.mat.as_ref().unwrap().scatter(&self, &rec) {
                return attenuation * scattered.ray_color(depth - 1, world);
            }
            return Vec3::zero();
        }

        let unit_direction = self.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        let c = white * (1.0 - a) + blue * a;
        c
    }
}
