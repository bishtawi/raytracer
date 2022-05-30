#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::rc::Rc;


use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::Material;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn main() -> Result<(), std::io::Error> {
    // Image

    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = utils::float_to_int_truncate(f64::from(image_width) / aspect_ratio);
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World

    let world = random_scene();

    // Camera

    let look_from = Point3::new_with_values(13.0, 2.0, 3.0);
    let look_at = Point3::new_with_values(0.0, 0.0, 0.0);
    let vup = Vec3::new_with_values(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render

    let path = Path::new("output.ppm");
    let file = File::create(&path)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + utils::random_float()) / f64::from(image_width - 1);
                let v = (f64::from(j) + utils::random_float()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_color += &ray_color(&r, &world, max_depth);
            }
            write_color(&mut buf_writer, &pixel_color, samples_per_pixel)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

fn write_color(
    writer: &mut BufWriter<File>,
    pixel_color: &Color,
    samples_per_pixel: i32,
) -> Result<(), std::io::Error> {
    let scale = 1.0 / f64::from(samples_per_pixel);
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();
    let ir = utils::float_to_int_truncate(256.0 * utils::clamp(r, 0.0, 0.999));
    let ig = utils::float_to_int_truncate(256.0 * utils::clamp(g, 0.0, 0.999));
    let ib = utils::float_to_int_truncate(256.0 * utils::clamp(b, 0.0, 0.999));

    writeln!(writer, "{} {} {}", ir, ig, ib)
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new(0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color::new_with_values(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new_with_values(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_float();
            let center = Point3::new_with_values(
                f64::from(a) + 0.9 * utils::random_float(),
                0.2,
                f64::from(b) + 0.9 * utils::random_float(),
            );

            if (center - Point3::new_with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    Rc::new(Lambertian::new(Color::random() * Color::random()))
                } else if choose_mat < 0.95 {
                    // metal
                    Rc::new(Metal::new(
                        Color::random_range(0.5, 1.0),
                        utils::random_float_range(0.0, 0.5),
                    ))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new_with_values(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new_with_values(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new_with_values(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new_with_values(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new_with_values(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
