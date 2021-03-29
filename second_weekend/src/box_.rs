use std::rc::Rc;

use super::aabb::AABB;
use super::ray::Ray;
use super::rect::{FlipNormal, XYRect, XZRect, YZRect};
use super::vec3::Vec3;
use crate::{
    hitable::{HitRecord, Hitable},
    hitable_list::HitableList,
    material::Material,
};

pub struct Box_ {
    pmin: Vec3,
    pmax: Vec3,
    panels: HitableList,
}

impl Box_ {
    pub fn new(p0: Vec3, p1: Vec3, material: Rc<dyn Material>) -> Self {
        let panels = {
            let panels: Vec<Box<dyn Hitable>> = vec![
                Box::new(XYRect::new(
                    p0.x(),
                    p1.x(),
                    p0.y(),
                    p1.y(),
                    p1.z(),
                    Rc::clone(&material),
                )),
                Box::new(FlipNormal::new(Box::new(XYRect::new(
                    p0.x(),
                    p1.x(),
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    Rc::clone(&material),
                )))),
                Box::new(XZRect::new(
                    p0.x(),
                    p1.x(),
                    p0.z(),
                    p1.z(),
                    p1.y(),
                    Rc::clone(&material),
                )),
                Box::new(FlipNormal::new(Box::new(XZRect::new(
                    p0.x(),
                    p1.x(),
                    p0.z(),
                    p1.z(),
                    p0.y(),
                    Rc::clone(&material),
                )))),
                Box::new(YZRect::new(
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    p1.z(),
                    p1.x(),
                    Rc::clone(&material),
                )),
                Box::new(FlipNormal::new(Box::new(YZRect::new(
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    p1.z(),
                    p0.x(),
                    Rc::clone(&material),
                )))),
            ];
            panels.into_iter().collect::<HitableList>()
        };
        Self {
            pmin: p0,
            pmax: p1,
            panels,
        }
    }
}

impl Hitable for Box_ {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.panels.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        AABB::new(self.pmin, self.pmax)
    }
}
