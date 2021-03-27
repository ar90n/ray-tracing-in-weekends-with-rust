use std::rc::Rc;

use super::ray::Ray;
use super::vec3::Vec3;
use super::material::Material;
use super::aabb::AABB;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB;
}
