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
    pub fn ray_color(&self, world: &HittableList) -> Vec3 {
        let mut rec = HitRecord::default();
        if world.hit(&self, Interval::with_values(0.0, INFINITY), &mut rec) {
            let c = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 255.0;
            return c
        }

        let unit_direction = self.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0) * 255.0;
        let blue = Vec3::new(0.5, 0.7, 1.0) * 255.0;
        let c = white * (1.0 - a) + blue * a;
        c
    }
}
