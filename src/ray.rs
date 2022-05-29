use crate::vec3;

#[derive(Default)]
pub struct Ray {
    orig: vec3::Point3,
    dir: vec3::Vec3,
}

impl Ray {
    pub fn new(orig: vec3::Point3, dir: vec3::Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &vec3::Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.orig + t * self.dir
    }
}
