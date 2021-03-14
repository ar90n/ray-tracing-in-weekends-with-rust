mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use camera::Camera;
use hitable::*;
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Vec3};

fn color(r: &Ray, world: &dyn Hitable, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.0, 1000.0) {
        let random_in_unit_sphere = {
            let r = rng.gen::<f32>();
            let theta = 2.0 * 3.14 * rng.gen::<f32>();
            let phi = 2.0 * 3.14 * rng.gen::<f32>();
            Vec3::new(
                r * phi.sin() * theta.cos(),
                r * phi.sin() * theta.sin(),
                r * phi.cos(),
            )
        };
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere;
        return 0.5 * color(&Ray::new(hit_record.p, target - hit_record.p), world, rng);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut rng = rand::thread_rng();

    println!("P3");
    println!("{} {}", &nx, &ny);
    println!("255");

    let world: HitableList = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)) as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)) as Box<dyn Hitable>,
    ]
    .into_iter()
    .collect();
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..ns {
                    let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                    let r = cam.get_ray(u, v);
                    col += color(&r, &world, &mut rng);
                }
                col /= ns as f32;
                Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt())
            };

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
