use crate::interval::Interval;
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::rtweekend;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new() -> Self {
        AABB {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }

    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        let mut new = AABB { x, y, z };
        new.pad_to_minimums();
        new
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::with_values(a.x, b.x)
        } else {
            Interval::with_values(b.x, a.x)
        };

        let y = if a.y <= b.y {
            Interval::with_values(a.y, b.y)
        } else {
            Interval::with_values(b.y, a.y)
        };

        let z = if a.z <= b.z {
            Interval::with_values(a.z, b.z)
        } else {
            Interval::with_values(b.z, a.z)
        };

        let mut new = AABB { x, y, z };
        new.pad_to_minimums();
        new
    }

    pub fn from_aabbs(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        AABB { x, y, z }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ray_orig_axis = match axis {
                0 => ray_orig.x,
                1 => ray_orig.y,
                _ => ray_orig.z,
            };
            let ray_dir_axis = match axis {
                0 => ray_dir.x,
                1 => ray_dir.y,
                _ => ray_dir.z,
            };
            let adinv = 1.0 / ray_dir_axis;

            let t0 = (ax.min - ray_orig_axis) * adinv;
            let t1 = (ax.max - ray_orig_axis) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.

        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}
