use std::rc::Rc;

use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::*;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = 2.0 * dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if 0.0 < discriminant {
            let near_t = -(b + discriminant.sqrt()) / (2.0 * a);
            if (t_min < near_t) && (near_t < t_max) {
                let t = near_t;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
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
                let normal = (p - self.center) / self.radius;
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
}
