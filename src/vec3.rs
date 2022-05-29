use std::fmt;
use std::ops;

pub type Color = Vec3;
// pub type Point3 = Vec3; // TODO: Enable when needed

pub struct Vec3 {
    elements: [f64; 3],
}

impl Vec3 {
    pub fn new_with_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            elements: [e0, e1, e2],
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

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.elements[0] * self.elements[0]
            + self.elements[1] * self.elements[1]
            + self.elements[2] * self.elements[2]
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
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            elements: [0.0, 0.0, 0.0],
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

impl ops::Add<&Vec3> for &Vec3 {
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

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.elements[0] += other.elements[0];
        self.elements[1] += other.elements[1];
        self.elements[2] += other.elements[2];
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
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

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.elements[0] -= rhs.elements[0];
        self.elements[1] -= rhs.elements[1];
        self.elements[2] -= rhs.elements[2];
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
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

impl ops::Mul<f64> for &Vec3 {
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
