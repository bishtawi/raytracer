use super::Texture;
use crate::{
    perlin::Perlin,
    vec3::{Axis, Color, Point3},
};

pub struct Noise {
    noise: Perlin,
    scale: f64,
    axis: Axis,
}

impl Noise {
    pub fn new(scale: f64, axis: Axis) -> Noise {
        Noise {
            noise: Perlin::default(),
            scale,
            axis,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let s = self.scale * p;
        Color::new_single(1.0)
            * 0.5
            * (1.0 + (s.axis(&self.axis) + 10.0 * self.noise.turb(&s, 7)).sin())
    }
}
