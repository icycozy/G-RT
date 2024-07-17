use std::ops::{AddAssign, MulAssign};
use crate::rtweekend::random_double;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if self.dot(on_unit_sphere) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    pub fn cross(&self, b: Self) -> Self {
        Self {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn dot(&self, b: Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        self.clone() - 2.0 * self.clone().dot(n) * n
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-1.0 * self.clone()).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self.clone() + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.squared_length()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_cosine_direction() -> Vec3 {
        let r1 = random_double(0.0, 1.0);
        let r2 = random_double(0.0, 1.0);

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        let z = (1.0 - r2).sqrt();

        Vec3::new(x, y, z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

// impl SubAssign for Vec3 {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//         };
//     }
// }

// impl SubAssign<f64> for Vec3 {
//     fn add_assign(&mut self, other: f64) {
//         *self = Self {
//             x: self.x - other,
//             y: self.y - other,
//             z: self.z - other,
//         };
//     }
// }

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }*/

    /*
    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }
    */

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0 as f64);
    }

    /*
    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }
    */
}
