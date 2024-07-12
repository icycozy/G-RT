use crate::hit::{HitRecord, Hittable, HittableClone};
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::material::Material;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hit_list::HittableList;
use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Option<Arc<dyn Material + Send + Sync>>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Option<Arc<dyn Material + Send + Sync>>) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);
        let w = (1.0 / n.squared_length()) * n;

        let bbox_diagonal1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::from_points(q + u, q + v);
        let bbox = AABB::from_aabbs(&bbox_diagonal1, &bbox_diagonal2);

        Quad {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
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
        let t = (self.d - self.normal.dot(*r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
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
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}

pub fn make_box(a: Point3, b: Point3, mat: Option<Arc<dyn Material + Send + Sync>>) -> HittableList {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new(
        f64::min(a.x(), b.x()),
        f64::min(a.y(), b.y()),
        f64::min(a.z(), b.z()),
    );
    let max = Point3::new(
        f64::max(a.x(), b.x()),
        f64::max(a.y(), b.y()),
        f64::max(a.z(), b.z()),
    );

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    ))); // front
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -1.0 * dz,
        dy,
        mat.clone(),
    ))); // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -1.0 * dx,
        dy,
        mat.clone(),
    ))); // back
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    ))); // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -1.0 * dz,
        mat.clone(),
    ))); // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    ))); // bottom

    sides
}