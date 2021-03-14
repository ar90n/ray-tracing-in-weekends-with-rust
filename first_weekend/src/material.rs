use std::f64::consts::PI;

use rand::random;

use crate::color;

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

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if 0.0 < discriminant {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 * r0
    };
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
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

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_over_nt, cosine_coeff) = if 0.0 < dot(r_in.direction(), &rec.normal)
        {
            (-rec.normal, self.ref_idx, self.ref_idx)
        } else {
            (rec.normal, 1.0 / self.ref_idx, -1.0)
        };

        let scatter_direction = refract(r_in.direction(), &outward_normal, ni_over_nt)
            .and_then(|refracted| {
                let cosine =
                    cosine_coeff * dot(r_in.direction(), &rec.normal) / r_in.direction().lenght();
                let refract_prob = schlick(cosine, self.ref_idx);
                if refract_prob < random::<f32>() {
                    Some(refracted)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| reflect(r_in.direction(), &rec.normal));
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((attenuation, scattered))
    }
}
