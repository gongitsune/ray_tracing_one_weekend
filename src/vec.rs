use nalgebra::Vector3;
use rand::{distributions::uniform::SampleRange, Rng};

pub type Color = Vector3<f32>;
pub type Vec3 = Vector3<f32>;

#[inline(always)]
pub fn random_vec<R: Rng, S: SampleRange<f32> + Clone>(rng: &mut R, range: S) -> Vec3 {
    Vec3::new(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range),
    )
}
