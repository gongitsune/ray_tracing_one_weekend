mod color;
mod ray;
mod vec;

use anyhow::{Ok, Result};
use color::write_color;
use indicatif::ProgressBar;
use nalgebra::Vector3;
use ray::Ray;
use std::io::{BufWriter, Write};
use vec::{Color, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let unit_dir = ray.direction().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn draw<W: Write>(
    aspect_ratio: f64,
    img_width: usize,
    writer: &mut BufWriter<W>,
) -> Result<()> {
    // Image
    let img_height = (img_width as f64 / aspect_ratio) as usize;

    // Progress
    let pb = ProgressBar::new(img_height as u64);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    writeln!(writer, "P3\n{} {}\n255", img_width, img_height)?;

    for j in (0..img_height).rev() {
        pb.inc(1);
        for i in 0..img_width {
            let u = i as f64 / (img_width - 1) as f64;
            let v = j as f64 / (img_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);
            write_color(writer, pixel_color)?;
        }
    }

    pb.finish();

    Ok(())
}
