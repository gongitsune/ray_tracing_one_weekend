use std::io::{BufWriter, Write};

use anyhow::{Ok, Result};

use crate::vec::Color;

/**
 * Write color
 */
pub fn write_color<W: Write>(writer: &mut BufWriter<W>, pixel_color: Color) -> Result<()> {
    let translate = |x: f64| (255.999 * x) as usize;
    writeln!(
        writer,
        "{} {} {}",
        translate(pixel_color.x),
        translate(pixel_color.y),
        translate(pixel_color.z)
    )?;

    Ok(())
}
