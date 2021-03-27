use std::rc::Rc;

use super::aabb::{surrounding_box, AABB};
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::*;

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center(r.time);
        let a = dot(r.direction(), r.direction());
        let b = 2.0 * dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if 0.0 < discriminant {
            let near_t = -(b + discriminant.sqrt()) / (2.0 * a);
            if (t_min < near_t) && (near_t < t_max) {
                let t = near_t;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Rc::clone(&self.material),
                });
            }

            let far_t = -(b - discriminant.sqrt()) / (2.0 * a);
            if (t_min < far_t) && (far_t < t_max) {
                let t = far_t;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Rc::clone(&self.material),
                });
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        let t0_aabb = AABB::new(self.center(t0) - radius_vec, self.center(t0) + radius_vec);
        let t1_aabb = AABB::new(self.center(t1) - radius_vec, self.center(t1) + radius_vec);
        surrounding_box(&t0_aabb, &t1_aabb)
    }
}
