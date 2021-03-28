use std::rc::Rc;

use crate::{hitable, hitable_list};

use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct FlipNormal(Box<dyn Hitable>);
impl FlipNormal {
    pub fn new(hitable: Box<dyn Hitable>) -> Self {
        Self(hitable)
    }
}

impl Hitable for FlipNormal {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.0.hit(r, t_min, t_max).map(|mut hit_record|{
            hit_record.normal = - hit_record.normal;
            hit_record
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        self.0.bounding_box(t0, t1)
    }
}

pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Rc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t_max < t {
            return None;
        }

        let p = r.point_at_parameter(t);
        if p.x() < self.x0 || self.x1 < p.x() || p.y() < self.y0 || self.y1 < p.y() {
            return None;
        }

        let u = (p.x() - self.x0) / (self.x1 - self.x0);
        let v = (p.y() - self.y0) / (self.y1 - self.y0);
        Some(HitRecord {
            u,
            v,
            t,
            p,
            material: Rc::clone(&self.material),
            normal: Vec3::new(0.0, 0.0, 1.0),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        )
    }
}

pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Rc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for XZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t_max < t {
            return None;
        }

        let p = r.point_at_parameter(t);
        if p.x() < self.x0 || self.x1 < p.x() || p.z() < self.z0 || self.z1 < p.z() {
            return None;
        }

        let u = (p.x() - self.x0) / (self.x1 - self.x0);
        let v = (p.z() - self.z0) / (self.z1 - self.z0);
        Some(HitRecord {
            u,
            v,
            t,
            p,
            material: Rc::clone(&self.material),
            normal: Vec3::new(0.0, 1.0, 0.0),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        )
    }
}

pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Rc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for YZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t_max < t {
            return None;
        }

        let p = r.point_at_parameter(t);
        if p.y() < self.y0 || self.y1 < p.y() || p.z() < self.z0 || self.z1 < p.z() {
            return None;
        }

        let u = (p.y() - self.y0) / (self.y1 - self.y0);
        let v = (p.z() - self.z0) / (self.z1 - self.z0);
        Some(HitRecord {
            u,
            v,
            t,
            p,
            material: Rc::clone(&self.material),
            normal: Vec3::new(1.0, 0.0, 0.0),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        )
    }
}
