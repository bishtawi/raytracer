use crate::{ray::Ray, vec3::Point3};

#[derive(Clone)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb {
            minimum: min,
            maximum: max,
        }
    }

    pub fn min(&self) -> &Point3 {
        &self.minimum
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum
    }

    pub fn hit(&self, r: &Ray, time_min: f64, time_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.minimum[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }

            if time_max.min(t1) <= time_min.max(t0) {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );
        Aabb::new(small, big)
    }
}
