use crate::hit::{Hittable, HitRecord, HittableClone};
use crate::hit_list::HittableList;
use crate::aabb::AABB;
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: AABB,
}

impl BVHNode {
    fn box_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index).clone();
        let b_axis_interval = b.bounding_box().axis_interval(axis_index).clone();
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap_or(Ordering::Equal)
    }

    fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        BVHNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        BVHNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        BVHNode::box_compare(a, b, 2)
    }

    pub fn new(list: &mut HittableList) -> Self {
        let start = 0;
        let end = list.objects.len();
        Self::new_recursive(&mut list.objects, start, end)
    }

    pub fn new_recursive(objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::new();
        for object_index in start..end {
            bbox = AABB::from_aabbs(&bbox, &objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();
    
        let comparator = match axis {
            0 => BVHNode::box_x_compare,
            1 => BVHNode::box_y_compare,
            _ => BVHNode::box_z_compare,
        };
    
        let object_span = end - start;
    
        let (left, right): (Arc<dyn Hittable + Send + Sync>, Arc<dyn Hittable + Send + Sync>);
    
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(comparator);
    
            let mid = start + object_span / 2;
            left = Arc::new(BVHNode::new_recursive(objects, start, mid));
            right = Arc::new(BVHNode::new_recursive(objects, mid, end));
        }
    
        BVHNode {
            left,
            right,
            bbox,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut ray_t = ray_t.clone();
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

impl HittableClone for BVHNode {
    fn clone_box(&self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self.clone())
    }
}