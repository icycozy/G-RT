use crate::vec3::Vec3;

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn new() -> Self {
        Self {
            axis: [Vec3::zero(), Vec3::zero(), Vec3::zero()],
        }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }

    pub fn local_vec3(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    pub fn build_from_w(w: &Vec3) -> Self {
        let unit_w = w.unit();
        let a = if unit_w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = unit_w.cross(a).unit();
        let u = unit_w.cross(v);
        let mut onb = Self::new();
        onb.axis[0] = u;
        onb.axis[1] = v;
        onb.axis[2] = unit_w;
        onb
    }
}