use crate::hit::{Hittable, HitRecord, HittableClone};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;
use std::sync::Arc;
use crate::rtweekend::random_int;
use crate::vec3::Vec3;
type Point3 = Vec3;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new(),
        }
    }

    pub fn hittable_list(object: Arc<dyn Hittable + Send + Sync>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.bbox = AABB::from_aabbs(&self.bbox,&object.bounding_box());
        self.objects.push(object);   
    }
    pub fn addlist(&mut self, list: HittableList) {
        for object in list.objects {
            self.add(object);
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::with_values(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len();
        self.objects[random_int(0, int_size as i32 - 1) as usize].random(origin)
    }
}

impl HittableClone for HittableList {
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}
