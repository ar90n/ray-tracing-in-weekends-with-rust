use super::vec3::*;

#[derive(Clone, Debug)]
pub struct Ray {
    A: Vec3,
    B: Vec3,
    pub time: f32
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f32) -> Self {
        Self { A: a, B: b, time }
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
