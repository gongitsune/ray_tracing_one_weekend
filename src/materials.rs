use rand::rngs::SmallRng;

use crate::{
    hittable::HitRecord,
    vec::{random_vec, Color, Vec3},
    Ray,
};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

#[inline(always)]
fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * random_vec(rng, 0.0..=1.0) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

#[inline(always)]
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

#[inline(always)]
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

#[inline(always)]
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = {
        let t = (1.0 - ref_idx) / (1.0 + ref_idx);
        t * t
    };
    r0 + (1.0 - r0) * {
        let t = 1.0 - cosine;
        t * t * t * t * t
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Color)>;
}
