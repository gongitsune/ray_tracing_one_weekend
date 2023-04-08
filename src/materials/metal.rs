use crate::{hittable::HitRecord, ray::Ray, vec::Color};

use super::{random_in_unit_sphere, reflect, Material};

pub struct Metal {
    albedo: Color,
    fuzzy: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f64) -> Self {
        Self {
            albedo,
            fuzzy: fuzzy.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray.direction().normalize(), &hit.normal);
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.point, reflected + self.fuzzy * random_in_unit_sphere());
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
