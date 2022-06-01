use super::Texture;
use crate::vec3::{Color, Point3};

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }

    /* Enable once needed
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor {
            color: Color::new(red, green, blue),
        }
    }*/
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}
