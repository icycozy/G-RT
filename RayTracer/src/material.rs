use std::any::Any;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
type Color = Vec3;
use crate::rtweekend::random_double;
use crate::texture::{Texture, SolidColor};
use std::sync::Arc;
// use crate::onb::ONB;
use crate::pdf::{Pdf, CosinePdf, SpherePdf};

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf + Send + Sync>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        ScatterRecord {
            attenuation: Color::zero(),
            pdf_ptr: Arc::new(SpherePdf::new()),
            skip_pdf: false,
            skip_pdf_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
        }
    }
}

pub trait Material: Any {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _scatter_rec: &mut ScatterRecord) -> bool {
        false
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Color {
        Color::zero()
    }
    fn as_any(&self) -> &dyn Any;
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    albedo: Color,
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo,
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn with_texture(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0), // Placeholder value, replace with desired albedo
            tex,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        scatter_rec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        scatter_rec.pdf_ptr = Arc::new(CosinePdf::new(rec.normal));
        scatter_rec.skip_pdf = false;
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * std::f64::consts::PI)
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        let reflected = r_in.direction().reflect(rec.normal);
        let reflected = reflected.unit() + (self.fuzz * Vec3::random_unit_vector());

        scatter_rec.attenuation = self.albedo;
        scatter_rec.skip_pdf = true;
        scatter_rec.skip_pdf_ray = Ray::new(rec.p, reflected, r_in.time());

        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        scatter_rec.attenuation = Color::new(1.0, 1.0, 1.0);
        scatter_rec.skip_pdf = true;
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit();
        let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta) > random_double(0.0, 1.0) {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }

        scatter_rec.skip_pdf_ray = Ray::new(rec.p, direction, r_in.time());
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        DiffuseLight { tex }
    }
    pub fn with_color(color: Color) -> Self {
        DiffuseLight {
            tex: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Color {
        if !rec.front_face {
            return Color::zero();
        }
        self.tex.value(u, v, p)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Isotropic {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Isotropic {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn with_texture(tex: Arc<dyn Texture + Sync + Send>) -> Self {
        Isotropic {
            tex,
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        scatter_rec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        scatter_rec.pdf_ptr = Arc::new(SpherePdf::new());
        scatter_rec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
