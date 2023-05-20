use anyhow::{Ok, Result};
use ray_tracing_one_weekend::draw;

fn main() -> Result<()> {
    draw(128, 256, 512, 50)?;
    Ok(())
}
