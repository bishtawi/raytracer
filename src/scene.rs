use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::texture::Checker;
use crate::utils;
use crate::vec3::{Color, Point3, Vec3};

pub struct Scene {
    pub world: HittableList,
    pub cam: Camera,
}

pub fn get(id: i32, aspect_ratio: f64) -> Scene {
    let world;
    let look_from;
    let look_at;
    let vfov;
    let aperture;

    match id {
        1 => {
            world = random_scene();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
        }
        _ => panic!("Unrecognized scene id {}", id),
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new_with_time(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene { world, cam }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Box::new(Checker::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new_with_texture(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_float();
            let center = Point3::new(
                f64::from(a) + 0.9 * utils::random_float(),
                0.2,
                f64::from(b) + 0.9 * utils::random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let center2 = center + Vec3::new(0.0, utils::random_float_range(0.0, 0.5), 0.0);
                    let mat = Arc::new(Lambertian::new(Color::random() * Color::random()));
                    world.add(Arc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, mat,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let mat = Arc::new(Metal::new(
                        Color::random_range(0.5, 1.0),
                        utils::random_float_range(0.0, 0.5),
                    ));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let checker = Box::new(Checker::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let mat = Arc::new(Lambertian::new_with_texture(checker));

    HittableList::new(&[
        Arc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, mat.clone())),
        Arc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat)),
    ])
}
