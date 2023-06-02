use rand::{rngs::SmallRng, Rng};

use crate::{ray::Ray, vec::Vec3};

fn random_in_unit_disk(rng: &mut SmallRng) -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - unit;
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = focus_dist * f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - focus_dist * w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    #[inline(always)]
    pub fn get_ray(&self, u: f32, v: f32, rng: &mut SmallRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
