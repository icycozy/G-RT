use crate::hit::{Hittable, HitRecord, HittableClone};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new(),
        }
    }

    pub fn hittable_list(object: Box<dyn Hittable>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::from_aabbs(&self.bbox,&object.bounding_box());
        self.objects.push(object);   
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
}

impl HittableClone for HittableList {
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
