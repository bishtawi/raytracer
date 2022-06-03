use std::sync::Arc;

use crate::{
    material::{isotropic::Isotropic, Material},
    texture::Texture,
    utils,
    vec3::{Color, Vec3},
};

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_func: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, color: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_func: Arc::new(Isotropic::new(color)),
        }
    }

    #[allow(dead_code)]
    pub fn new_texture(
        boundary: Box<dyn Hittable>,
        density: f64,
        texture: Box<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_func: Arc::new(Isotropic::new_texture(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * utils::random_float().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;

                return Some(HitRecord {
                    p: r.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    material: self.phase_func.clone(),
                    t,
                    u: 0.0,
                    v: 0.0,
                    front_face: true,
                });
            }
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
