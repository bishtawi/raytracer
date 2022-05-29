#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

mod camera;
mod hittable;
mod hittable_list;
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
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn main() -> Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = utils::float_to_int_truncate(f64::from(image_width) / aspect_ratio);
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let world = HittableList::new(&[
        Rc::new(Sphere::new(Point3::new_with_values(0.0, 0.0, -1.0), 0.5)),
        Rc::new(Sphere::new(
            Point3::new_with_values(0.0, -100.5, -1.0),
            100.0,
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

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}
