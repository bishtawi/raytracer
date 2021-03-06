use std::fmt;
use std::ops;

use crate::utils;

pub type Color = Vec3;
pub type Point3 = Vec3;

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    elements: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            elements: [e0, e1, e2],
        }
    }

    pub fn new_single(v: f64) -> Vec3 {
        Vec3 {
            elements: [v, v, v],
        }
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }

    pub fn y(&self) -> f64 {
        self.elements[1]
    }

    pub fn z(&self) -> f64 {
        self.elements[2]
    }

    pub fn axis(&self, axis: &Axis) -> f64 {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
            Axis::Z => self.z(),
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.elements[0] * other.elements[0]
            + self.elements[1] * other.elements[1]
            + self.elements[2] * other.elements[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.elements[1] * other.elements[2] - self.elements[2] * other.elements[1],
                self.elements[2] * other.elements[0] - self.elements[0] * other.elements[2],
                self.elements[0] * other.elements[1] - self.elements[1] * other.elements[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.elements[0].abs() < s && self.elements[1].abs() < s && self.elements[2].abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - &(2.0 * self.dot(n) * n)
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + &(cos_theta * n));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            utils::random_float(),
            utils::random_float(),
            utils::random_float(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            utils::random_float_range(min, max),
            utils::random_float_range(min, max),
            utils::random_float_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                utils::random_float_range(-1.0, 1.0),
                utils::random_float_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
        }
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] + other.elements[0],
                self.elements[1] + other.elements[1],
                self.elements[2] + other.elements[2],
            ],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] + other.elements[0],
                self.elements[1] + other.elements[1],
                self.elements[2] + other.elements[2],
            ],
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.elements[0] += other.elements[0];
        self.elements[1] += other.elements[1];
        self.elements[2] += other.elements[2];
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] - other.elements[0],
                self.elements[1] - other.elements[1],
                self.elements[2] - other.elements[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] - other.elements[0],
                self.elements[1] - other.elements[1],
                self.elements[2] - other.elements[2],
            ],
        }
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.elements[0] -= rhs.elements[0];
        self.elements[1] -= rhs.elements[1];
        self.elements[2] -= rhs.elements[2];
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] * other.elements[0],
                self.elements[1] * other.elements[1],
                self.elements[2] * other.elements[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self.elements[0] * other.elements[0],
                self.elements[1] * other.elements[1],
                self.elements[2] * other.elements[2],
            ],
        }
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self * other.elements[0],
                self * other.elements[1],
                self * other.elements[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            elements: [
                self * other.elements[0],
                self * other.elements[1],
                self * other.elements[2],
            ],
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        other * self
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        other * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.elements[0] *= rhs;
        self.elements[1] *= rhs;
        self.elements[2] *= rhs;
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.elements[0], self.elements[1], self.elements[2]
        )
    }
}
