use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

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

pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new_single(1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct Image {
    data: Vec<u8>,
    _width: i32,
    _height: i32,
    _bytes_per_scanline: i32,
    _bytes_per_pixel: i32,
}

impl Image {
    pub fn new(_file_path: &str) -> Image {
        // TODO: Load image file into memory
        Image {
            data: Vec::new(),
            _width: 0,
            _height: 0,
            _bytes_per_scanline: 0,
            _bytes_per_pixel: 3,
        }
    }
}

impl Texture for Image {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        /*let uu = utils::clamp(u, 0.0, 1.0);
        let vv = 1.0 - utils::clamp(v, 0.0, 1.0);

        let mut i = utils::float_to_int_truncate(uu * f64::from(self.width));
        let mut j = utils::float_to_int_truncate(vv * f64::from(self.height));

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height-1;
        }

        let color_scale = 1.0 / 255.0;*/

        todo!()
    }
}
