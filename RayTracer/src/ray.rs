use std::f64::INFINITY;

use crate::hit_list::HittableList;
use crate::interval::Interval;
use crate::material::ScatterRecord;
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::hit::{HitRecord, Hittable};
use crate::pdf::{Pdf, CosinePdf, HittablePdf, MixturePdf};
use std::sync::Arc;
use crate::rtweekend::random_double;

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
    pub fn ray_color(&self, background: Vec3, depth: u32, world: &HittableList, lights: &HittableList) -> Vec3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // If the ray hits nothing, return the background color.
        if !world.hit(self, Interval::with_values(0.001, INFINITY), &mut rec) {
            return background;
        }

        let mat = rec.mat.as_ref().unwrap();
        let mut srec = ScatterRecord::default();
        let color_from_emission = mat.emitted(self, &rec, rec.u, rec.v, &rec.p);

        if !mat.scatter(self, &rec, &mut srec) {
            // println!("{} {} {}", color_from_emission.x, color_from_emission.y, color_from_emission.z);
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation * srec.skip_pdf_ray.ray_color(background, depth - 1, world, lights);
        }

        let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
        let p = MixturePdf::new(light_ptr, srec.pdf_ptr);

        let scattered = Ray::new(rec.p, p.generate(), self.time());
        let pdf_val = p.value(scattered.direction());

        let scattering_pdf = mat.scattering_pdf(self, &rec, &scattered);

        let sample_color = scattered.ray_color(background, depth - 1, world, lights);
        let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf_val;

        color_from_emission + color_from_scatter
    }
}
