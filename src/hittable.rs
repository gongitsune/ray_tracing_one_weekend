pub mod aabb;
pub mod bvh;
pub mod sphere;

use crate::{materials::Material, ray::Ray, vec::Vec3};

use self::aabb::Aabb;

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}
