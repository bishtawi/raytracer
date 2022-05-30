use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir,
            time: 0.0,
        }
    }

    pub fn new_with_time(orig: Point3, dir: Vec3, time: f64) -> Ray {
        Ray { orig, dir, time }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
