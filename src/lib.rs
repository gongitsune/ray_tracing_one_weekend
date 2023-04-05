mod color;
mod hittable;
mod ray;
mod sphere;
mod vec;

use anyhow::{Ok, Result};
use color::write_color;
use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use ray::Ray;
use sphere::Sphere;
use std::{
    f64::INFINITY,
    io::{BufWriter, Write},
    sync::Arc,
};
use vec::{Color, Vec3};

fn ray_color<H: Hittable>(ray: &Ray, world: &H) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = ray.direction().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / a
    }
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

    // World
    let world = HittableList::new(vec![
        Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

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
            let pixel_color = ray_color(&ray, &world);
            write_color(writer, pixel_color)?;
        }
    }

    pb.finish();

    Ok(())
}
