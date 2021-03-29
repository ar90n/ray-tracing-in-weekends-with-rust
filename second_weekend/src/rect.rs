use std::rc::Rc;

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
        self.0.hit(r, t_min, t_max).map(|mut hit_record| {
            hit_record.normal = -hit_record.normal;
            hit_record
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        self.0.bounding_box(t0, t1)
    }
}

pub struct Translate {
    offset: Vec3,
    hitable: Box<dyn Hitable>,
}

impl Translate {
    pub fn new(offset: Vec3, hitable: Box<dyn Hitable>) -> Self {
        Self { offset, hitable }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time);
        self.hitable
            .hit(&moved_r, t_min, t_max)
            .map(|rec| HitRecord {
                t: rec.t,
                p: rec.p + self.offset,
                u: rec.u,
                v: rec.v,
                normal: rec.normal,
                material: rec.material,
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        let box_ = self.hitable.bounding_box(t0, t1);
        AABB::new(box_.min + self.offset, box_.max + self.offset)
    }
}

pub struct RotateY {
    hitable: Box<dyn Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {
    pub fn new(angle: f32, hitable: Box<dyn Hitable>) -> Self {
        let radians = std::f32::consts::PI * angle / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let aabb_box = hitable.bounding_box(0.0, 1.0);
        let mut min = [std::f32::MAX, std::f32::MAX, std::f32::MAX];
        let mut max = [-std::f32::MAX, -std::f32::MAX, -std::f32::MAX];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * aabb_box.min.x() + (1 - i) as f32 * aabb_box.max.x();
                    let y = j as f32 * aabb_box.min.y() + (1 - j) as f32 * aabb_box.max.y();
                    let z = k as f32 * aabb_box.min.z() + (1 - k) as f32 * aabb_box.max.z();
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = tester[c].min(min[c]);
                        max[c] = tester[c].max(max[c]);
                    }
                }
            }
        }
        let bbox = AABB::new(
            Vec3::new(min[0], min[1], min[2]),
            Vec3::new(max[0], max[1], max[2]),
        );

        Self {
            hitable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rotated_origin_x = self.cos_theta * r.origin().x() + -self.sin_theta * r.origin().z();
        let rotated_origin_z = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();
        let rotated_direction_x =
            self.cos_theta * r.direction().x() + -self.sin_theta * r.direction().z();
        let rotated_direction_z =
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();
        let rotated_r = Ray::new(
            Vec3::new(rotated_origin_x, r.origin().y(), rotated_origin_z),
            Vec3::new(rotated_direction_x, r.direction().y(), rotated_direction_z),
            r.time,
        );
        self.hitable.hit(&rotated_r, t_min, t_max).map(|rec| {
            let rotated_p_x = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
            let rotated_p_z = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();
            let rotated_normal_x =
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            let rotated_normal_z =
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

            HitRecord {
                t: rec.t,
                p: Vec3::new(rotated_p_x, rec.p.y(), rotated_p_z),
                u: rec.u,
                v: rec.v,
                normal: Vec3::new(rotated_normal_x, rec.normal.y(), rotated_normal_z),
                material: rec.material,
            }
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        self.bbox.clone()
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
