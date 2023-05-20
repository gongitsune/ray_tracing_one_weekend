use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "ray tracing one weekend",
    about = "Rust implementation of `Ray tracing in one weekend`"
)]
pub struct Cli {
    #[clap(default_value = "256", help = "image width", short)]
    pub width: u32,
    #[clap(default_value = "128", help = "image height", short)]
    pub height: u32,
    #[clap(default_value = "500", help = "samples par pixel", short)]
    pub samples: u32,
    #[clap(default_value = "50", help = "max depth", short)]
    pub depth: u32,
}
