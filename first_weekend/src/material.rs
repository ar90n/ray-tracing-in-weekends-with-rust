use std::f64::consts::PI;

use rand::random;

use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::*;

fn random_in_unit_sphere() -> Vec3 {
    let r = random::<f32>();
    let theta = 2.0 * PI * random::<f64>();
    let phi = 2.0 * PI * random::<f64>();
    polar_to_cartesian(r, theta as f32, phi as f32)
}

fn polar_to_cartesian(r: f32, theta: f32, phi: f32) -> Vec3 {
    Vec3::new(
        r * phi.sin() * theta.cos(),
        r * phi.sin() * theta.sin(),
        r * phi.cos(),
    )
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(rec.p, target - rec.p)))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if 0.0 < dot(&reflected, &rec.normal) {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
