use crate::hit::{HitRecord, Hittable, HittableClone};
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::material::Material;
use crate::aabb::AABB;
use std::rc::Rc;
use crate::ray::Ray;
use crate::interval::Interval;

#[derive(Clone)]
pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Option<Rc<dyn Material>>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Option<Rc<dyn Material>>,) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        let D = normal.dot(Q);
        let w = 1.0 / n.length() * n;

        let bbox_diagonal1 = AABB::from_points(Q, Q + u + v);
        let bbox_diagonal2 = AABB::from_points(Q + u, Q + v);
        let bbox = AABB::from_aabbs(&bbox_diagonal1, &bbox_diagonal2);

        Quad {
            Q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            D,
        }
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::with_values(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }

}

impl Hittable for Quad {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(*r.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.D - self.normal.dot(*r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }
}

impl HittableClone for Quad {
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}