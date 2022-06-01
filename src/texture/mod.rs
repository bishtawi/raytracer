pub mod checker;
pub mod image;
pub mod noise;
pub mod solid_color;

use crate::vec3::{Color, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
