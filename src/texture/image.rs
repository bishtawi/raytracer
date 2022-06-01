use super::Texture;
use crate::vec3::{Color, Point3};

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
