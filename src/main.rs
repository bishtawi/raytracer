#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

mod ray;
mod vec3;

fn main() -> Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    #[allow(clippy::cast_possible_truncation)] // Truncation is fine
    let image_height = (f64::from(image_width) / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0.0);
    let horizontal = vec3::Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - vec3::Vec3::new_with_values(0.0, 0.0, focal_length);

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
            let r = ray::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(&mut buf_writer, &pixel_color)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

fn write_color(
    writer: &mut BufWriter<File>,
    pixel_color: &vec3::Color,
) -> std::result::Result<(), std::io::Error> {
    #[allow(clippy::cast_possible_truncation)] // Truncation is fine
    let ir = (255.999 * pixel_color.x()) as i64;
    #[allow(clippy::cast_possible_truncation)]
    let ig = (255.999 * pixel_color.y()) as i64;
    #[allow(clippy::cast_possible_truncation)]
    let ib = (255.999 * pixel_color.z()) as i64;

    writeln!(writer, "{} {} {}", ir, ig, ib)
}

fn ray_color(r: &ray::Ray) -> vec3::Color {
    if hit_sphere(&vec3::Point3::new_with_values(0.0, 0.0, -1.0), 0.5, r) {
        return vec3::Color::new_with_values(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0) + t * vec3::Color::new_with_values(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &vec3::Point3, radius: f64, r: &ray::Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
