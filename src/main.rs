#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

mod aabb;
mod camera;
mod hittable;
mod material;
mod perlin;
mod ray;
mod scene;
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
    let scene = scene::get(&scene::Type::Final);

    let pixels: Vec<Vec<Color>> = (0..scene.image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..scene.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::default();
                    for _ in 0..scene.samples_per_pixel {
                        let u = (f64::from(i) + utils::random_float())
                            / f64::from(scene.image_width - 1);
                        let v = (f64::from(j) + utils::random_float())
                            / f64::from(scene.image_height - 1);
                        let r = scene.cam.get_ray(u, v);
                        pixel_color +=
                            &ray_color(&r, &scene.background, &scene.world, scene.max_depth);
                    }
                    Color::new(
                        (pixel_color.x() * scene.scale).sqrt(),
                        (pixel_color.y() * scene.scale).sqrt(),
                        (pixel_color.z() * scene.scale).sqrt(),
                    )
                })
                .collect()
        })
        .collect();

    let path = Path::new("output.ppm");
    let file = File::create(&path)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(
        buf_writer,
        "P3\n{} {}\n255",
        scene.image_width, scene.image_height
    )?;

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

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if !rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return emitted;
        }

        emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
    } else {
        *background
    }
}
