#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utils;
mod vec3;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use rayon::prelude::*;

use hittable::Hittable;
use ray::Ray;
use vec3::Color;

fn main() -> Result<(), std::io::Error> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = utils::float_to_int_truncate(f64::from(image_width) / aspect_ratio);
    let samples_per_pixel = 100;
    let max_depth = 50;
    let scale = 1.0 / f64::from(samples_per_pixel);

    // Scene

    let (world, cam) = scene::get(&scene::Scene::Random, aspect_ratio);

    // Render

    let pixels: Vec<Vec<Color>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::default();
                    for _ in 0..samples_per_pixel {
                        let u = (f64::from(i) + utils::random_float()) / f64::from(image_width - 1);
                        let v =
                            (f64::from(j) + utils::random_float()) / f64::from(image_height - 1);
                        let r = cam.get_ray(u, v);
                        pixel_color += &ray_color(&r, &world, max_depth);
                    }
                    Color::new(
                        (pixel_color.x() * scale).sqrt(),
                        (pixel_color.y() * scale).sqrt(),
                        (pixel_color.z() * scale).sqrt(),
                    )
                })
                .collect()
        })
        .collect();

    let path = Path::new("output.ppm");
    let file = File::create(&path)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "P3\n{} {}\n255", image_width, image_height)?;

    for v in pixels {
        for p in v {
            write_color(&mut buf_writer, &p)?;
        }
    }

    Ok(())
}

fn write_color(writer: &mut BufWriter<File>, pixel_color: &Color) -> Result<(), std::io::Error> {
    let ir = utils::float_to_int_truncate(256.0 * utils::clamp(pixel_color.x(), 0.0, 0.999));
    let ig = utils::float_to_int_truncate(256.0 * utils::clamp(pixel_color.y(), 0.0, 0.999));
    let ib = utils::float_to_int_truncate(256.0 * utils::clamp(pixel_color.z(), 0.0, 0.999));

    writeln!(writer, "{} {} {}", ir, ig, ib)
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
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

        return Color::default();
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new_single(1.0) + t * Color::new(0.5, 0.7, 1.0)
}
