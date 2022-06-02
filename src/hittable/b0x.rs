use std::sync::Arc;

use super::{
    htlist::HittableList, xyrect::XYRect, xzrect::XZRect, yzrect::YZRect, HitRecord, Hittable,
};
use crate::{aabb::Aabb, material::Material, ray::Ray, vec3::Point3};

pub struct B0x {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl B0x {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> B0x {
        let sides = HittableList::new(&[
            Arc::new(XYRect::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p1.z(),
                material.clone(),
            )),
            Arc::new(XYRect::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p0.z(),
                material.clone(),
            )),
            Arc::new(XZRect::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p1.y(),
                material.clone(),
            )),
            Arc::new(XZRect::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p0.y(),
                material.clone(),
            )),
            Arc::new(YZRect::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p1.x(),
                material.clone(),
            )),
            Arc::new(YZRect::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p0.x(),
                material,
            )),
        ]);

        B0x {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for B0x {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
