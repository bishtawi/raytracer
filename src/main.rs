#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

mod vec3;

fn main() -> Result<()> {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    let path = Path::new("output.ppm");
    let file = File::create(&path)?;
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = vec3::Vec3::new_with_values(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.25,
            );
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
