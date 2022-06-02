use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, ray::Ray, vec3::Vec3};

pub struct Translate {
    hittable: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Box<dyn Hittable>, displacement: Vec3) -> Translate {
        Translate {
            hittable,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray::new(r.origin() - &self.offset, *r.direction(), r.time());
        if let Some(mut rec) = self.hittable.hit(&moved, t_min, t_max) {
            let normal = rec.normal;
            rec.p += &self.offset;
            rec.set_face_normal(&moved, &normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.hittable.bounding_box(time0, time1).map(|output_box| {
            Aabb::new(
                output_box.min() + &self.offset,
                output_box.max() + &self.offset,
            )
        })
    }
}
