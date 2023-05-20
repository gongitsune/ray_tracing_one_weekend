use anyhow::{Ok, Result};
use clap::Parser;
use ray_tracing_one_weekend::{cli::Cli, draw};
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = Path::new("image.png");
    let image = draw(cli.height, cli.width, cli.samples, cli.depth)?;

    image.save_with_format(path, image::ImageFormat::Png)?;
    Ok(())
}
