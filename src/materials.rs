use rand::rngs::SmallRng;

use crate::{
    hittable::HitRecord,
    vec::{random_vec, Color, Vec3},
    Ray,
};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * random_vec(rng, 0.0..=1.0) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Color)>;
}
