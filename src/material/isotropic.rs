use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture},
    vec3::{Color, Vec3},
};

use super::Material;

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: Color) -> Isotropic {
        Isotropic {
            albedo: Box::new(SolidColor::new(c)),
        }
    }

    pub fn new_texture(albedo: Box<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
