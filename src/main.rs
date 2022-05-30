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

use anyhow::Result;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3};

fn main() -> Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = utils::float_to_int_truncate(f64::from(image_width) / aspect_ratio);
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let material_ground = Rc::new(Lambertian::new(Color::new_with_values(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new_with_values(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new_with_values(0.8, 0.6, 0.2), 0.0));

    let world = HittableList::new(&[
        Rc::new(Sphere::new(
            Point3::new_with_values(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Rc::new(Sphere::new(
            Point3::new_with_values(0.0, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Rc::new(Sphere::new(
            Point3::new_with_values(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new_with_values(-1.0, 0.0, -1.0),
            -0.4,
            material_left,
        )),
        Rc::new(Sphere::new(
            Point3::new_with_values(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ]);

    // Camera

    let cam = Camera::default();

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
) -> std::result::Result<(), std::io::Error> {
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
