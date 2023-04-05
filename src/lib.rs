mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec;

use anyhow::{Ok, Result};
use camera::Camera;
use color::write_color;
use hittable::Hittable;
use indicatif::ProgressBar;
use rand::Rng;
use ray::Ray;
use scene::random_scene;
use sphere::Sphere;
use std::{
    f64::INFINITY,
    io::{BufWriter, Write},
};
use vec::{Color, Vec3};

fn ray_color<H: Hittable>(ray: &Ray, world: &H, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            return attenuation.zip_map(&ray_color(&scattered, world, depth - 1), |l, r| l * r);
        }
        return Color::zeros();
    }

    let unit_dir = ray.direction().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn draw<W: Write>(
    aspect_ratio: f64,
    img_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    writer: &mut BufWriter<W>,
) -> Result<()> {
    let mut rng = rand::thread_rng();

    // Image
    let img_height = (img_width as f64 / aspect_ratio) as usize;

    // Progress
    let pb = ProgressBar::new(img_height as u64);

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = (look_from - look_at).magnitude();
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // Render
    writeln!(writer, "P3\n{} {}\n255", img_width, img_height)?;

    for j in (0..img_height).rev() {
        pb.inc(1);
        for i in 0..img_width {
            let pixel_color = (0..samples_per_pixel).fold(Vec3::new(0.0, 0.0, 0.0), |color, _| {
                let u = (i as f64 + rng.gen::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (img_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                color + ray_color(&ray, &world, max_depth)
            });
            write_color(writer, pixel_color, samples_per_pixel)?;
        }
    }

    pb.finish();

    Ok(())
}
