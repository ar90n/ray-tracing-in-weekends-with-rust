use std::f32::consts::PI;

use crate::ray::Ray;

use rand::random;

use super::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_weight = aspect * half_height;

        let origin = lookfrom.clone();
        let w = unit_vector(&(*lookfrom - *lookat));
        let u = unit_vector(&cross(vup, &w));
        let v = cross(&w, &u);

        let lower_left_corner =
            origin - half_weight * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_weight * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical
                - (self.origin + offset),
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random::<f32>(), random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if 1.0 < dot(&p, &p) {
            return p;
        }
    }
}
