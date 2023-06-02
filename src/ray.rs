use crate::vec::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
    #[inline(always)]
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
