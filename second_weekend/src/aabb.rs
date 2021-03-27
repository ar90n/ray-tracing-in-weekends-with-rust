use super::ray::Ray;
use super::vec3::*;

#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<()> {
        let mut total_t_min = t_min;
        let mut total_t_max = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let (t0, t1) = {
                let t0 = (self.min[a] - r.origin()[a]) * inv_d;
                let t1 = (self.max[a] - r.origin()[a]) * inv_d;
                if inv_d < 0.0 {
                    (t1, t0)
                } else {
                    (t0, t1)
                }
            };

            total_t_min = total_t_min.max(t0);
            total_t_max = total_t_max.min(t1);
            if total_t_max <= total_t_min {
                return None;
            }
        }
        Some(())
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = Vec3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(small, big)
}
