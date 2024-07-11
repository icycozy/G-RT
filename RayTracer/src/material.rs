use std::any::Any;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
type Color = Vec3;
use crate::rtweekend::random_double;
use crate::texture::{Texture, SolidColor, CheckerTexture};

pub trait Material: Any {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        Color::zero()
    }
    fn as_any(&self) -> &dyn Any;
}

pub struct Lambertian {
    albedo: Color,
    tex: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo,
            tex: Box::new(SolidColor::new(albedo)),
        }
    }
    pub fn with_texture(tex: Box<dyn Texture>) -> Self {
        Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0), // Placeholder value, replace with desired albedo
            tex,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { 
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = _r_in.direction().reflect(rec.normal);
        let reflected = reflected.unit() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected, _r_in.time());
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
    pub fn reflectance(&self, cosine: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit();

        let cos_theta = f64::min((-1.0 * unit_direction).dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta) > random_double(0.0, 1.0){
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }
        let scattered = Ray::new(rec.p, direction, r_in.time());
        Some((attenuation, scattered))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Box<dyn Texture>) -> Self {
        DiffuseLight { tex }
    }
    pub fn with_color(color: Color) -> Self {
        DiffuseLight {
            tex: Box::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.tex.value(u, v, p)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Isotropic {
    tex: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Isotropic {
            tex: Box::new(SolidColor::new(albedo)),
        }
    }
    pub fn with_texture(tex: Box<dyn Texture>) -> Self {
        Isotropic {
            tex,
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_unit_vector(), _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
