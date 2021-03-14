use super::vec3::*;
use super::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    veritcal: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            veritcal: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.veritcal - self.origin)
    }
}