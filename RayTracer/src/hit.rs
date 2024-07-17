use crate::ray::Ray;
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::interval::Interval;
use crate::material::Material;
use std::f64::INFINITY;
use crate::aabb::AABB;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material + Send + Sync>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: None,
        }
    }
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal.clone()) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.clone() * (-1.0)
        };
    }
}

pub trait HittableClone {
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync>;
}
pub trait Hittable: HittableClone {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}


#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Translate { object, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl HittableClone for Translate {
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}
impl RotateY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new( INFINITY,  INFINITY,  INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin.x = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin.z = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction.x = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction.z = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_r = Ray::new(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where)
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Change the intersection point from object space to world space
        let mut p = rec.p;
        p.x = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
        p.z = -1.0 * self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

        // Change the normal from object space to world space
        let mut normal = rec.normal;
        normal.x = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.z = -1.0 * self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
impl HittableClone for RotateY {
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}
