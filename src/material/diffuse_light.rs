use crate::{
    texture::{solid_color::SolidColor, Texture},
    vec3::{Color, Point3},
};

use super::Material;

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(color: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Box::new(SolidColor::new(color)),
        }
    }

    /* Enable once needed
    pub fn new_texture(texture: Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight {
            emit: texture,
        }
    }*/
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &crate::ray::Ray,
        _rec: &crate::hittable::HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut crate::ray::Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
