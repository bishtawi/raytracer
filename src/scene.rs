use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable::b0x::B0x;
use crate::hittable::bvh_node::BVHNode;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::htlist::HittableList;
use crate::hittable::moving_sphere::MovingSphere;
use crate::hittable::rotate_y::RotateY;
use crate::hittable::sphere::Sphere;
use crate::hittable::translate::Translate;
use crate::hittable::xyrect::XYRect;
use crate::hittable::xzrect::XZRect;
use crate::hittable::yzrect::YZRect;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::utils;
use crate::vec3::{Color, Point3, Vec3};

#[allow(dead_code)]
pub enum Scene {
    Random,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
    CornellSmoke,
}

pub fn get(scene: &Scene, aspect_ratio: f64) -> (HittableList, Camera, Color) {
    let world;
    let look_from;
    let look_at;
    let vfov;
    let aperture;
    let background;

    match scene {
        Scene::Random => {
            world = random_scene();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.1;
            background = Color::new(0.70, 0.80, 1.00);
        }
        Scene::TwoSpheres => {
            world = two_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
        }
        Scene::TwoPerlinSpheres => {
            world = two_perlin_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
        }
        Scene::Earth => {
            world = earth();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
        }
        Scene::SimpleLight => {
            world = simple_light();
            background = Color::default();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        Scene::CornellBox => {
            world = cornell_box();
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
        Scene::CornellSmoke => {
            world = cornell_smoke();
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
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

    (world, cam, background)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Box::new(Checker::new(
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

    HittableList::single(Arc::new(BVHNode::new_from_list(&world, 0.0, 1.0)))
}

fn two_spheres() -> HittableList {
    let checker = Box::new(Checker::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let mat = Arc::new(Lambertian::new_with_texture(checker));

    HittableList::new(&[
        Arc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, mat.clone())),
        Arc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat)),
    ])
}

fn two_perlin_spheres() -> HittableList {
    let perlin = Box::new(Noise::new(4.0));
    let mat = Arc::new(Lambertian::new_with_texture(perlin));

    HittableList::new(&[
        Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        )),
        Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)),
    ])
}

fn earth() -> HittableList {
    let earth_texture = Box::new(Image::new("resources/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::default(), 2.0, earth_surface));

    HittableList::single(globe)
}

fn simple_light() -> HittableList {
    let perlin = Box::new(Noise::new(4.0));
    let mat = Arc::new(Lambertian::new_with_texture(perlin));

    HittableList::new(&[
        Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        )),
        Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)),
        Arc::new(XYRect::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0))),
        )),
    ])
}

fn cornell_box() -> HittableList {
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    HittableList::new(&[
        Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
        Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)),
        Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
        Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(B0x::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(165.0, 330.0, 165.0),
                    white.clone(),
                )),
                15.0,
            )),
            Vec3::new(265.0, 0.0, 295.0),
        )),
        Arc::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(B0x::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(165.0, 165.0, 165.0),
                    white,
                )),
                -18.0,
            )),
            Vec3::new(130.0, 0.0, 65.0),
        )),
    ])
}

fn cornell_smoke() -> HittableList {
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    HittableList::new(&[
        Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
        Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light)),
        Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
        Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(ConstantMedium::new(
            Box::new(Translate::new(
                Box::new(RotateY::new(
                    Box::new(B0x::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(165.0, 330.0, 165.0),
                        white.clone(),
                    )),
                    15.0,
                )),
                Vec3::new(265.0, 0.0, 295.0),
            )),
            0.01,
            Color::default(),
        )),
        Arc::new(ConstantMedium::new(
            Box::new(Translate::new(
                Box::new(RotateY::new(
                    Box::new(B0x::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(165.0, 165.0, 165.0),
                        white,
                    )),
                    -18.0,
                )),
                Vec3::new(130.0, 0.0, 65.0),
            )),
            0.01,
            Color::new(1.0, 1.0, 1.0),
        )),
    ])
}
