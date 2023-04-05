use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;

            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { point, normal, t });
            }
            let t = (-b + sqrt_discriminant) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { point, normal, t });
            }
        }

        None
    }
}
