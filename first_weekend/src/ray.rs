use super::vec3::*;

#[derive(Clone, Debug)]
pub struct Ray {
    A: Vec3,
    B: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { A: a, B: b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.A
    }

    pub fn direction(&self) -> &Vec3 {
        &self.B
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.A + t * self.B
    }
}
