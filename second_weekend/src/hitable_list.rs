use std::iter::FromIterator;
use std::rc::Rc;

use ordered_float::OrderedFloat;
use rand::random;

use super::aabb::{surrounding_box, AABB};
use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;

struct BVHNode {
    left: Rc<dyn Hitable>,
    right: Rc<dyn Hitable>,
    box_: AABB,
}

impl BVHNode {
    pub fn new(hittables: &mut [Rc<dyn Hitable>], time0: f32, time1: f32) -> Self {
        let axis = (3.0 * random::<f32>()).floor() as usize;
        hittables.sort_by_key(|hitable| OrderedFloat(hitable.bounding_box(time0, time1).min[axis]));

        let (left, right) = match hittables.len() {
            1 => (Rc::clone(&hittables[0]), Rc::clone(&hittables[0])),
            2 => (Rc::clone(&hittables[0]), Rc::clone(&hittables[1])),
            _ => {
                let mid = hittables.len() / 2;
                (
                    Rc::new(BVHNode::new(&mut hittables[0..mid], time0, time1)) as Rc<dyn Hitable>,
                    Rc::new(BVHNode::new(&mut hittables[mid..], time0, time1)) as Rc<dyn Hitable>,
                )
            }
        };
        let box_ = surrounding_box(
            &left.bounding_box(time0, time1),
            &right.bounding_box(time0, time1),
        );
        Self { left, right, box_ }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.box_.hit(&r, t_min, t_max).and_then(|()| {
            let left_hit = self.left.hit(&r, t_min, t_max);
            let right_hit = self.right.hit(&r, t_min, t_max);
            match (left_hit, right_hit) {
                (Some(lh), Some(rh)) => {
                    if lh.t < rh.t {
                        Some(lh)
                    } else {
                        Some(rh)
                    }
                }
                (Some(lh), None) => Some(lh),
                (None, Some(rh)) => Some(rh),
                _ => None,
            }
        })
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        surrounding_box(
            &self.left.bounding_box(t0, t1),
            &self.right.bounding_box(t0, t1),
        )
    }
}

pub struct HitableList(BVHNode);
impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.0.hit(&r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        self.0.bounding_box(t0, t1)
    }
}

impl FromIterator<Box<dyn Hitable>> for HitableList {
    fn from_iter<I: IntoIterator<Item = Box<dyn Hitable>>>(iter: I) -> HitableList {
        let mut inner = iter.into_iter().map(Rc::from).collect::<Vec<_>>();
        let root = BVHNode::new(&mut inner, 0.0, 1.0);
        HitableList(root)
    }
}
