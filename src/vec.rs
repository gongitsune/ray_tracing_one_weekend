use nalgebra::Vector3;
use rand::Rng;

pub type Color = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
