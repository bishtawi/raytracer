use crate::{
    aabb::Aabb,
    ray::Ray,
    utils,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

pub struct RotateY {
    hittable: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: Box<dyn Hittable>, angle: f64) -> RotateY {
        let radians = utils::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        let bbox = hittable.bounding_box(0.0, 1.0).map(|bbox| {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x =
                            f64::from(i) * bbox.max().x() + (1.0 - f64::from(i)) * bbox.min().x();
                        let y =
                            f64::from(j) * bbox.max().y() + (1.0 - f64::from(j)) * bbox.min().y();
                        let z =
                            f64::from(k) * bbox.max().z() + (1.0 - f64::from(k)) * bbox.min().z();

                        let nx = cos_theta * x + sin_theta * z;
                        let nz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(nx, y, nz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            Aabb::new(min, max)
        });

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.hittable.hit(&rotated, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox.clone()
    }
}
