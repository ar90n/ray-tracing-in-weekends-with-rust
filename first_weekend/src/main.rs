mod ray;
mod vec3;

use ray::Ray;
use vec3::{dot, unit_vector, Vec3};

fn hit_spere(center: &Vec3, radius: f32, r: &Ray) -> Option<f32> {
    let oc = *r.origin() - *center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(&oc, r.direction());
    let c = dot(&oc,&oc) - radius * radius;
    let discriminant = b * b  -4.0 * a * c;
    if 0.0 < discriminant {
        Some(-(b + discriminant.sqrt()) / (2.0 * a))
    } else {
        None
    }
}

fn color(r: &Ray) -> Vec3 {
    if let Some(t) = hit_spere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        let N = unit_vector(&(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Vec3::new(N.x() +1.0 ,N.y() + 1.0, N.z() + 1.0);
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * veritcal);

            let col = color(&r);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
