use std::rc::Rc;

use rand::random;

use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Box<dyn Hitable>,
    density: f32,
    material: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(density: f32, material: Rc<dyn Material>, boundary: Box<dyn Hitable>) -> Self {
        Self {
            density,
            boundary,
            material,
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(r, -std::f32::MAX, std::f32::MAX)?;
        let rec2 = self.boundary.hit(r, rec1.t + 0.00001, std::f32::MAX)?;
        let t1 = rec1.t.max(t_min);
        let t2 = rec2.t.min(t_max);
        if t2 <= t1 {
            return None;
        }

        let t1 = t1.max(0.0);
        let distance_inside_boundary = (t2 - t1) * r.direction().lenght();
        let hit_distance = -(1.0 / self.density) * random::<f32>().ln();
        if distance_inside_boundary <= hit_distance {
            return None;
        }

        let t = t1 + hit_distance / r.direction().lenght();
        let p = r.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            u: 0.0,
            v: 0.0,
            normal: Vec3::default(),
            material: Rc::clone(&self.material),
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        self.boundary.bounding_box(t0, t1)
    }
}
