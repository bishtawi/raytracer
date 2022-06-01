use super::{solid_color::SolidColor, Texture};
use crate::vec3::{Color, Point3};

pub struct Checker {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checker {
    pub fn new(c1: Color, c2: Color) -> Checker {
        Checker {
            even: Box::new(SolidColor::new(c1)),
            odd: Box::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
