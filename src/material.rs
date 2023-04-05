use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
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

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt) = if ray.direction().dot(&hit.normal) > 0.0 {
            (-hit.normal, self.index_of_refraction)
        } else {
            (hit.normal, 1.0 / self.index_of_refraction)
        };
        if let Some(refracted) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            let scattered = Ray::new(hit.point, refracted);
            Some((scattered, attenuation))
        } else {
            let reflected = reflect(&ray.direction(), &hit.normal);
            let scattered = Ray::new(hit.point, reflected);
            Some((scattered, attenuation))
        }
    }
}
