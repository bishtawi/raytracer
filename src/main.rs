#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

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

    // World

    let world = HittableList::new(&[
        Rc::new(Sphere::new(Point3::new_with_values(0.0, 0.0, -1.0), 0.5)),
        Rc::new(Sphere::new(
            Point3::new_with_values(0.0, -100.5, -1.0),
            100.0,
        )),
    ]);

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0);
    let horizontal = Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new_with_values(0.0, 0.0, focal_length);

    // Render

    let path = Path::new("output.ppm");
    let file = File::create(&path)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            write_color(&mut buf_writer, &pixel_color)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

fn write_color(
    writer: &mut BufWriter<File>,
    pixel_color: &Color,
) -> std::result::Result<(), std::io::Error> {
    let ir = utils::float_to_int_truncate(255.999 * pixel_color.x());
    let ig = utils::float_to_int_truncate(255.999 * pixel_color.y());
    let ib = utils::float_to_int_truncate(255.999 * pixel_color.z());

    writeln!(writer, "{} {} {}", ir, ig, ib)
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)
}
