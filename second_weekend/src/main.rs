mod aabb;
mod box_;
mod camera;
mod constant_medium;
mod hitable;
mod hitable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rect;
mod sphere;
mod texture;
mod vec3;

use core::f32;
use std::rc::Rc;

use image::io::Reader as ImageReader;
use rand::random;

use box_::Box_;
use camera::Camera;
use constant_medium::ConstantMedium;
use hitable::*;
use hitable_list::HitableList;
use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use moving_sphere::MovingSphere;
use ray::Ray;
use rect::{FlipNormal, RotateY, Translate, XYRect, XZRect, YZRect};
use sphere::Sphere;
use texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use vec3::Vec3;

fn color(r: &Ray, world: &dyn Hitable, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, 1000.0) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
        let scattered = match rec.material.scatter(r, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                attenuation * color(&scattered, world, depth + 1)
            }
            _ => Vec3::default(),
        };
        return emitted + scattered;
    }

    Vec3::default()
}

fn random_scene() -> HitableList {
    let checker = CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    );
    let mut world: Vec<Box<dyn Hitable>> = vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Box::new(checker))),
    ))];
    for a in -15..15 {
        for b in -15..15 {
            let radius = 0.1 + 0.25 * random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                radius,
                b as f32 + 0.9 * random::<f32>(),
            );
            let material_choice = random::<f32>();
            let material: Rc<dyn Material> = if material_choice < 0.8 {
                Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                )))))
            } else if material_choice < 0.95 {
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

            let obj_choice = random::<f32>();
            if obj_choice < 0.3 {
                world.push(Box::new(Sphere::new(center, radius, material)));
            } else {
                let center0 = center;
                let center1 = center0 + Vec3::new(0.0, 1.5, 0.0) * random::<f32>();
                world.push(Box::new(MovingSphere::new(
                    center0, center1, 0.0, 1.0, radius, material,
                )));
            }
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
        Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world.into_iter().collect()
}

fn simple_scene() -> HitableList {
    let img = ImageReader::open("./assets/texture.jpg")
        .map(|reader| reader.decode().unwrap())
        .map(|img| img.to_rgb8())
        .unwrap();
    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(Lambertian::new(Box::new(NoiseTexture::new(12.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Rc::new(Lambertian::new(Box::new(NoiseTexture::new(12.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 2.0),
            1.0,
            Rc::new(Lambertian::new(Box::new(ImageTexture::new(img)))),
        )),
    ];

    world.into_iter().collect()
}

fn simple_light() -> HitableList {
    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(Lambertian::new(Box::new(NoiseTexture::new(12.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Rc::new(Lambertian::new(Box::new(NoiseTexture::new(12.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 7.0, 0.0),
            2.0,
            Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
                Vec3::new(4.0, 4.0, 4.0),
            )))),
        )),
        Box::new(XYRect::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
                Vec3::new(4.0, 4.0, 4.0),
            )))),
        )),
    ];

    world.into_iter().collect()
}

fn cornel_box() -> HitableList {
    let red: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.65, 0.05, 0.05),
    ))));
    let gray: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.73, 0.73, 0.73),
    ))));
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(1.0, 1.0, 1.0),
    ))));
    let black: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.0, 0.0, 0.0),
    ))));
    let green: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.12, 0.45, 0.15),
    ))));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
        Vec3::new(15.0, 15.0, 15.0),
    ))));
    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipNormal::new(Box::new(YZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Rc::clone(&green),
        )))),
        Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::clone(&red))),
        Box::new(XZRect::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            Rc::clone(&light),
        )),
        Box::new(FlipNormal::new(Box::new(XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Rc::clone(&gray),
        )))),
        Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::clone(&gray))),
        Box::new(FlipNormal::new(Box::new(XYRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Rc::clone(&gray),
        )))),
        Box::new(ConstantMedium::new(
            0.01,
            Rc::clone(&black),
            Box::new(Translate::new(
                Vec3::new(130.0, 0.0, 65.0),
                Box::new(RotateY::new(
                    -18.0,
                    Box::new(Box_::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(165.0, 165.0, 165.0),
                        Rc::clone(&white),
                    )),
                )),
            )),
        )),
        Box::new(ConstantMedium::new(
            0.01,
            Rc::clone(&white),
            Box::new(Translate::new(
                Vec3::new(265.0, 0.0, 295.0),
                Box::new(RotateY::new(
                    15.0,
                    Box::new(Box_::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(165.0, 330.0, 165.0),
                        Rc::clone(&black),
                    )),
                )),
            )),
        )),
    ];

    world.into_iter().collect()
}

fn second_weekend_final() -> HitableList {
    let img = ImageReader::open("./assets/texture.jpg")
        .map(|reader| reader.decode().unwrap())
        .map(|img| img.to_rgb8())
        .unwrap();
    let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.48, 0.83, 0.53),
    ))));
    let brown: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.7, 0.3, 0.1),
    ))));
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(
        Vec3::new(0.73, 0.73, 0.73),
    ))));
    let ground_boxes = (0..400)
        .into_iter()
        .map(|idx| {
            let i = idx / 20;
            let j = idx % 20;
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f32 * w;
            let x1 = x0 + w;
            let y1 = 100.0 * (random::<f32>() + 0.01);
            let z1 = z0 + w;
            Box::new(Box_::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Rc::clone(&ground),
            )) as Box<dyn Hitable>
        })
        .collect::<HitableList>();

    let group_spheres = (0..1000)
        .into_iter()
        .map(|_| {
            let x = 165.0 * random::<f32>();
            let y = 165.0 * random::<f32>();
            let z = 165.0 * random::<f32>();
            Box::new(Sphere::new(Vec3::new(x, y, z), 10.0, Rc::clone(&white))) as Box<dyn Hitable>
        })
        .collect::<HitableList>();

    let light: Rc<dyn Material> = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
        Vec3::new(7.0, 7.0, 7.0),
    ))));
    let center = Vec3::new(400.0, 400.0, 200.0);

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(ground_boxes),
        Box::new(XZRect::new(
            123.0,
            423.0,
            147.0,
            412.0,
            554.0,
            Rc::clone(&light),
        )),
        Box::new(MovingSphere::new(
            center,
            center + Vec3::new(30.0, 0.0, 0.0),
            0.0,
            1.0,
            50.0,
            Rc::clone(&brown),
        )),
        Box::new(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(360.0, 160.0, 145.0),
            70.0,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(ConstantMedium::new(
            0.2,
            Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.2, 0.4, 0.9,
            ))))),
            Box::new(Sphere::new(
                Vec3::new(360.0, 160.0, 145.0),
                70.0,
                Rc::new(Dielectric::new(1.5)),
            )),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            5000.0,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(ConstantMedium::new(
            0.0001,
            Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                1.0, 1.0, 1.0,
            ))))),
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                5000.0,
                Rc::new(Dielectric::new(1.5)),
            )),
        )),
        Box::new(Sphere::new(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            Rc::new(Lambertian::new(Box::new(ImageTexture::new(img)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(220.0, 200.0, 300.0),
            80.0,
            Rc::new(Lambertian::new(Box::new(NoiseTexture::new(0.1)))),
        )),
        Box::new(Translate::new(Vec3::new(-100.0, 270.0, 395.0), Box::new(RotateY::new(15.0, Box::new(group_spheres)))))
    ];
    world.into_iter().collect()
}

fn main() {
    let nx = 640;
    let ny = 640;
    let ns = 8192;

    println!("P3");
    println!("{} {}", &nx, &ny);
    println!("255");

    //let world = random_scene();
    //let world = simple_scene();
    //let world = simple_light();
    //let world = cornel_box();
    let world = second_weekend_final();
    let lookfrom = Vec3::new(428.0, 278.0, -800.0);
    //let lookfrom = Vec3::new(278.0, 278.0, 278.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
                Vec3::new(
                    col.x().sqrt().min(1.0),
                    col.y().sqrt().min(1.0),
                    col.z().sqrt().min(1.0),
                )
            };

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
