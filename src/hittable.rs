use crate::{materials::Material, ray::Ray, vec::Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (_, hit_anything) = self.objects.iter().fold((t_max, None), |mut a, obj| {
            if let Some(hit) = obj.hit(ray, t_min, a.0) {
                a.0 = hit.t;
                a.1 = Some(hit);
            }
            a
        });

        hit_anything
    }
}
