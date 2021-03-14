use std::f32::consts::PI;

use crate::ray::Ray;

use super::vec3::*;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_weight = aspect * half_height;

        let origin = lookfrom.clone();
        let w = unit_vector(&(*lookfrom - *lookat));
        let u = unit_vector(&cross(vup, &w));
        let v = cross(&w, &u);

        let lower_left_corner = origin - half_weight * u - half_height * v - w;
        let horizontal = 2.0 * half_weight * u;
        let vertical = 2.0 * half_height * v;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
