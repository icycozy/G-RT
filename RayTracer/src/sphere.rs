use crate::hit::{Hittable, HitRecord, HittableClone};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;
use std::sync::Arc;
use crate::onb::ONB;
use crate::rtweekend::random_double;

#[derive(Clone)]
pub struct Sphere {
    center1: Vec3,
    radius: f64,
    mat: Option<Arc<dyn Material + Send + Sync>>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: AABB,
}

impl Sphere {
    // Stationary Sphere
    pub fn new(center: Vec3, radius: f64, mat: Option<Arc<dyn Material + Send + Sync>>) -> Self {
        let center_vec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - center_vec, center + center_vec);   
        Self {
            center1: center,
            radius: f64::max(0.0, radius),
            mat,
            is_moving: false,
            center_vec,
            bbox,
        }
    }

    // Moving Sphere
    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Option<Arc<dyn Material + Send + Sync>>) -> Self {
        let center_vec = center2 - center1;
        let bbox = AABB::from_aabbs(
            &AABB::from_points(center1 - Vec3::new(radius, radius, radius), center1 + Vec3::new(radius, radius, radius)),
            &AABB::from_points(center2 - Vec3::new(radius, radius, radius), center2 + Vec3::new(radius, radius, radius))
        );
        Self {
            center1,
            radius: f64::max(0.0, radius),
            mat,
            is_moving: true,
            center_vec,
            bbox,
        }
    }

    fn sphere_center(&self, time: f64) -> Vec3 {
        self.center1 + self.center_vec * time.clone()
    }

    pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double(0.0, 1.0);
        let r2 = random_double(0.0, 1.0);
        let z = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / distance_squared) - 1.0);

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = f64::cos(phi) * f64::sqrt(1.0 - z * z);
        let y = f64::sin(phi) * f64::sqrt(1.0 - z * z);

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center1
        };
        let oc = center - r.origin().clone();
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
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(outward_normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        // This method only works for stationary spheres.

        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(origin.clone(), direction.clone(), 0.0), Interval::with_values(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let cos_theta_max = (1.0 - self.radius * self.radius / (self.center1 - *origin).squared_length()).sqrt();
        let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center1 - origin.clone();
        let distance_squared = direction.squared_length();
        let uvw = ONB::build_from_w(&direction);
        uvw.local_vec3(&Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

impl HittableClone for Sphere {
    fn clone_box(&self) -> Arc<dyn Hittable + Sync + Send> {
        Arc::new(self.clone())
    }
}