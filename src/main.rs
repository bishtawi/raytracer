#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

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
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.25;

            #[allow(clippy::cast_possible_truncation)]
            let ir = (255.999 * r) as i64;
            #[allow(clippy::cast_possible_truncation)]
            let ig = (255.999 * g) as i64;
            #[allow(clippy::cast_possible_truncation)]
            let ib = (255.999 * b) as i64;

            writeln!(buf_writer, "{} {} {}", ir, ig, ib)?;
        }
    }

    println!("\nDone.");

    Ok(())
}
