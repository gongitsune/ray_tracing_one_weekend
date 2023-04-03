mod color;
mod vec;

use anyhow::{Ok, Result};
use color::write_color;
use indicatif::ProgressBar;
use nalgebra::Vector3;
use std::io::{BufWriter, Write};

pub fn draw<W: Write>(
    (img_width, img_height): (i32, i32),
    writer: &mut BufWriter<W>,
) -> Result<()> {
    // Progress
    let pb = ProgressBar::new(img_height as u64);

    // Render
    writeln!(writer, "P3\n{} {}\n255", img_width, img_height)?;

    for j in (0..img_height).rev() {
        pb.inc(1);
        for i in 0..img_width {
            let pixel_color: Vector3<f64> = Vector3::new(
                i as f64 / (img_width - 1) as f64,
                j as f64 / (img_height - 1) as f64,
                0.25,
            );
            write_color(writer, pixel_color)?;
        }
    }

    pb.finish();

    Ok(())
}
