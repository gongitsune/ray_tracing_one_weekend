use std::io::{BufWriter, Write};

use anyhow::{Ok, Result};
use indicatif::ParallelProgressIterator;
use nalgebra::Vector3;
use rayon::prelude::*;

pub fn draw<W: Write>(
    (img_width, img_height): (i32, i32),
    writer: &mut BufWriter<W>,
) -> Result<()> {
    // Render
    writeln!(writer, "P3\n{} {}\n255", img_width, img_height)?;
    let img = (0..img_height)
        .into_par_iter()
        .progress_count(img_height as u64)
        .rev()
        .flat_map(|y| {
            (0..img_width)
                .flat_map(|x| {
                    let col = Vector3::new(
                        y as f32 / img_width as f32,
                        x as f32 / img_height as f32,
                        0.25,
                    );
                    vec![
                        (255.999 * col[0]) as u8,
                        (255.999 * col[1]) as u8,
                        (255.999 * col[2]) as u8,
                    ]
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for col in img.chunks(3) {
        writeln!(writer, "{} {} {}", col[0], col[1], col[2])?;
    }
    Ok(())
}
