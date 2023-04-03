use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();

        let root = (-b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            let root = (-b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord {
            point,
            normal: (point - self.center) / self.radius,
            t: root,
        })
    }
}
