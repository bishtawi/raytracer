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
use crate::vec3::{Axis, Color, Point3, Vec3};

#[allow(dead_code)]
pub enum Type {
    Random,
    RandomMod,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
    CornellSmoke,
    Final,
}

pub struct Scene {
    pub world: HittableList,
    pub cam: Camera,
    pub background: Color,
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub scale: f64,
}

#[allow(clippy::too_many_lines)]
pub fn get(scene: &Type) -> Scene {
    let world;
    let look_from;
    let look_at;
    let vfov;
    let aperture;
    let background;
    let aspect_ratio;
    let image_width;
    let samples_per_pixel;
    let max_depth;

    match scene {
        Type::Random => {
            world = random_scene();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.1;
            background = Color::new(0.70, 0.80, 1.00);
            aspect_ratio = 3.0 / 2.0;
            image_width = 1200;
            samples_per_pixel = 500;
            max_depth = 50;
        }
        Type::RandomMod => {
            world = random_scene_mod();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.1;
            background = Color::new(0.70, 0.80, 1.00);
            aspect_ratio = 16.0 / 9.0;
            image_width = 600;
            samples_per_pixel = 200;
            max_depth = 50;
        }
        Type::TwoSpheres => {
            world = two_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            max_depth = 50;
        }
        Type::TwoPerlinSpheres => {
            world = two_perlin_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            max_depth = 50;
        }
        Type::Earth => {
            world = earth();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::default();
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.70, 0.80, 1.00);
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            max_depth = 50;
        }
        Type::SimpleLight => {
            world = simple_light();
            background = Color::default();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 400;
            max_depth = 50;
        }
        Type::CornellBox => {
            world = cornell_box();
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 400;
            max_depth = 50;
        }
        Type::CornellSmoke => {
            world = cornell_smoke();
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            max_depth = 50;
        }
        Type::Final => {
            world = final_scene();
            background = Color::default();
            look_from = Point3::new(478.0, 278.0, -600.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            max_depth = 50;
        }
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let time_start = 0.0;
    let time_end = 1.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        time_start,
        time_end,
    );

    Scene {
        world,
        cam,
        background,
        aspect_ratio,
        image_width,
        image_height: utils::float_to_int_truncate(f64::from(image_width) / aspect_ratio),
        samples_per_pixel,
        max_depth,
        scale: 1.0 / f64::from(samples_per_pixel),
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::single(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new_single(0.5))),
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
                    let mat = Arc::new(Lambertian::new(Color::random() * Color::random()));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
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

fn random_scene_mod() -> HittableList {
    let mut world = HittableList::single(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_texture(Box::new(Checker::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        )))),
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
    let perlin = Box::new(Noise::new(4.0, Axis::Z));
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
    let perlin = Box::new(Noise::new(4.0, Axis::Z));
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
            0.01,
            Color::default(),
        )),
        Arc::new(ConstantMedium::new(
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
            0.01,
            Color::new(1.0, 1.0, 1.0),
        )),
    ])
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + f64::from(i) * w;
            let z0 = -1000.0 + f64::from(j) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = utils::random_float_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(B0x::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new(Color::new_single(0.73)));
    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    HittableList::new(&[
        Arc::new(BVHNode::new_from_list(&boxes1, 0.0, 1.0)),
        Arc::new(XZRect::new(
            123.0,
            423.0,
            147.0,
            412.0,
            554.0,
            Arc::new(DiffuseLight::new(Color::new_single(7.0))),
        )),
        Arc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1))),
        )),
        Arc::new(Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectric::new(1.5)),
        )),
        Arc::new(Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
        )),
        boundary.clone(),
        Arc::new(ConstantMedium::new(
            boundary,
            0.2,
            Color::new(0.2, 0.4, 0.9),
        )),
        Arc::new(ConstantMedium::new(
            Arc::new(Sphere::new(
                Point3::default(),
                5000.0,
                Arc::new(Dielectric::new(1.5)),
            )),
            0.0001,
            Color::new_single(1.0),
        )),
        /* TODO: Finish Image texture implementation
        Arc::new(Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            Arc::new(Lambertian::new_with_texture(Box::new(Image::new(
                "resources/earthmap.jpg",
            )))),
        )),*/
        // Replacement sphere until image texture sphere is completed
        Arc::new(Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
        )),
        Arc::new(Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::new_with_texture(Box::new(Noise::new(
                0.1,
                Axis::X,
            )))),
        )),
        Arc::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(BVHNode::new_from_list(&boxes2, 0.0, 1.0)),
                15.0,
            )),
            Vec3::new(-100.0, 270.0, 395.0),
        )),
    ])
}
