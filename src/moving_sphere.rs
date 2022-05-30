use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct MovingSphere {
    center_start: Point3,
    center_end: Point3,
    time_start: f64,
    time_end: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center_start: Point3,
        center_end: Point3,
        time_start: f64,
        time_end: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - &self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            p: r.at(root),
            t: root,
            material: self.material.clone(),
            normal: Vec3::default(),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let rad = Vec3::new_single(self.radius);
        let box0 = Aabb::new(self.center(time0) - rad, self.center(time0) + rad);
        let box1 = Aabb::new(self.center(time1) - rad, self.center(time1) + rad);
        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
