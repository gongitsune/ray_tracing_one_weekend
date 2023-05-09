use crate::{ray::Ray, vec::Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let mint = (self.min[a] - r.origin()[a]) / r.direction()[a];
            let maxt = (self.max[a] - r.origin()[a]) / r.direction()[a];
            let t0 = mint.min(maxt);
            let t1 = mint.max(maxt);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Vec3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let big = Vec3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );

    Aabb {
        min: small,
        max: big,
    }
}
