use std::iter::FromIterator;

use super::hitable::*;
use super::ray::Ray;

pub struct HitableList(Vec<Box<dyn Hitable>>);
impl HitableList {
    pub fn new() -> Self {
        Self(vec![])
    }
}
impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.0.iter().fold(None, |result, obj| {
            let t_far = result.as_ref().map(|obj| obj.t).unwrap_or(t_max);
            obj.hit(r, t_min, t_far).or(result)
        })
    }
}

impl FromIterator<Box<dyn Hitable>> for HitableList {
    fn from_iter<I: IntoIterator<Item=Box<dyn Hitable>>>(iter: I) -> Self {
        HitableList(iter.into_iter().collect::<Vec<_>>())
    }
}