mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use std::ops::Deref;

use hitable::*;
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Vec3};

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.0, 1000.0) {
        return 0.5
            * Vec3::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            );
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3");
    println!("{} {}", &nx, &ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let veritcal = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world: HitableList = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)) as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)) as Box<dyn Hitable>,
    ]
    .into_iter()
    .collect();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * veritcal);

            let col = color(&r, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
