mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use rand::random;

use camera::Camera;
use hitable::*;
use hitable_list::HitableList;
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Vec3};

fn color(r: &Ray, world: &dyn Hitable, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, 1000.0) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            if depth < 50 {
                return attenuation * color(&scattered, world, depth + 1);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HitableList {
    let n = 500;
    let mut world: Vec<Box<dyn Hitable>> = vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ))];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            let choice = random::<f32>();
            let material: Rc<dyn Material> = if choice < 0.8 {
                Rc::new(Lambertian::new(Vec3::new(
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                )))
            } else if choice < 0.95 {
                Rc::new(Metal::new(
                    Vec3::new(
                        0.5 * (1.0 + random::<f32>()),
                        0.5 * (1.0 + random::<f32>()),
                        0.5 * (1.0 - random::<f32>()),
                    ),
                    0.5 * random::<f32>(),
                ))
            } else {
                Rc::new(Dielectric::new(1.5))
            };
            world.push(Box::new(Sphere::new(center, 0.2, material)));
        }
    }
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world.into_iter().collect()
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    println!("P3");
    println!("{} {}", &nx, &ny);
    println!("255");

    let world = random_scene();
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).lenght();
    let aperture = 2.0;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..ns {
                    let u = (i as f32 + random::<f32>()) / nx as f32;
                    let v = (j as f32 + random::<f32>()) / ny as f32;
                    let r = cam.get_ray(u, v);
                    col += color(&r, &world, 0);
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
