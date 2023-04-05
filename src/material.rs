use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{random_in_unit_sphere, Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.point, target - hit.point);

        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray.direction().normalize(), &hit.normal);
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.point, reflected);
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
