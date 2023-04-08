use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{Ok, Result};
use clap::Parser;
use ray_tracing_one_weekend::{cli::Cli, draw};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = Path::new("image.ppm");
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)?;
    let mut writer = BufWriter::new(file);

    draw(3.0 / 2.0, cli.width, cli.samples, cli.depth, &mut writer)?;

    writer.flush()?;
    Ok(())
}
