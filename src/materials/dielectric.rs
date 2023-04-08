use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};
use rand::Rng;

use super::{reflect, refract, schlick, Material};

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ref_idx: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            let cosine =
                self.ref_idx * ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (-hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = -ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };
        if let Some(refracted) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            let refract_prob = schlick(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f64>() >= refract_prob {
                let scattered = Ray::new(hit.point, refracted);
                return Some((scattered, attenuation));
            }
        }

        let reflected = reflect(&ray.direction(), &hit.normal);
        let scattered = Ray::new(hit.point, reflected);
        Some((scattered, attenuation))
    }
}
