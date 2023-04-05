use std::sync::Arc;

use rand::Rng;

use crate::{
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Metal},
    vec::{random_vec, Vec3},
    Color, Sphere,
};

pub fn random_scene() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = vec![];
    let mut rng = rand::thread_rng();

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vec(&mut rng, 0.0..=1.0)
                        .zip_map(&random_vec(&mut rng, 0.0..=1.0), |l, r| l * r);
                    objects.push(Arc::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec(&mut rng, 0.5..=1.0);
                    let fuzzy = rng.gen_range(0.0..=0.5);
                    objects.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(albedo, fuzzy),
                    )));
                } else {
                    objects.push(Arc::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    objects.push(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    objects.push(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.push(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    HittableList::new(objects)
}
