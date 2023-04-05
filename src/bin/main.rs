use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{Ok, Result};
use ray_tracing_one_weekend::draw;

fn main() -> Result<()> {
    let path = Path::new("image.ppm");
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)?;
    let mut writer = BufWriter::new(file);

    draw(3.0 / 2.0, 1200, 500, 50, &mut writer)?;

    writer.flush()?;
    Ok(())
}
