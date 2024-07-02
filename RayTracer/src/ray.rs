use crate::vec3::Vec3;
type Point3 = Vec3;

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }

    pub fn ray_color(r: &Ray) -> [u8; 3] {
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0) * 255.0;
        let blue = Vec3::new(0.5, 0.7, 1.0) * 255.0;
        let c = white * (1.0 - a) + blue * a;
        [c.x() as u8, c.y() as u8, c.z() as u8]
    }
}
