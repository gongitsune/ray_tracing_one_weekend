use rand::rngs::ThreadRng;

use crate::{hittable::HitRecord, ray::Ray, vec::Color};

use super::{random_in_unit_sphere, Material};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.point, target - hit.point);

        Some((scattered, self.albedo))
    }
}
