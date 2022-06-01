use std::{cmp::Ordering, sync::Arc};

use super::{htlist::HittableList, HitRecord, Hittable};
use crate::{aabb::Aabb, utils};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn new_from_list(list: &HittableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::new_from_array(&list.objects, 0, list.objects.len(), time0, time1)
    }

    pub fn new_from_array(
        objects: &[Arc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let axis = utils::random_int(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Unexpected axis {}", axis),
        };

        let span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            // Shallow clone and sort array
            let mut objects = objects.to_vec();
            objects.sort_by(comparator);

            let mid = start + span / 2;
            left = Arc::new(BVHNode::new_from_array(&objects, start, mid, time0, time1));
            right = Arc::new(BVHNode::new_from_array(&objects, mid, end, time0, time1));
        }

        let box_left = left
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor");
        let box_right = right
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor");

        let bbox = Aabb::surrounding_box(&box_left, &box_right);

        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let mut rec = None;
        let mut max = t_max;

        for object in [&self.left, &self.right] {
            if let Some(hr) = object.hit(r, t_min, max) {
                max = hr.t;
                rec = Some(hr);
            }
        }

        rec
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a
        .bounding_box(0.0, 0.0)
        .expect("No bounding box in bvh_node constructor");
    let box_b = b
        .bounding_box(0.0, 0.0)
        .expect("No bounding box in bvh_node constructor");

    let axis_a = box_a.min()[axis];
    let axis_b = box_b.min()[axis];

    if axis_a < axis_b {
        Ordering::Less
    } else if axis_a > axis_b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
