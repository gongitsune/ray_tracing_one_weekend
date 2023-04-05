use std::io::{BufWriter, Write};

use anyhow::{Ok, Result};

use crate::vec::Color;

/**
 * Write color
 */
pub fn write_color<W: Write>(
    writer: &mut BufWriter<W>,
    pixel_color: Color,
    samples_per_pixel: usize,
) -> Result<()> {
    let cast = |x: f64| (256.0 * x.clamp(0.0, 0.999)) as usize;

    let scale = 1.0 / samples_per_pixel as f64;
    writeln!(
        writer,
        "{} {} {}",
        cast((pixel_color.x * scale).sqrt()),
        cast((pixel_color.y * scale).sqrt()),
        cast((pixel_color.z * scale).sqrt())
    )?;

    Ok(())
}
