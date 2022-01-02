use std::{fs::File, io::Write};

use rand::Rng;
use rust_raytracer::{
    camera::Camera,
    scatters::get_color,
    surfaces::{Material, Sphere, Surfaces},
    vec3::Vec3,
};

fn main() {
    let camera = Camera::default();
    let surfaces = Surfaces::new(vec![
        Box::new(Sphere::new(
            Vec3::new(3.5, 0.23, -2.5),
            0.8,
            Material::Metal(0.25),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.5, -0.15, -2.2),
            0.4,
            Material::Metal(0.7),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.85, -0.2, -1.25),
            0.3,
            Material::Metal(0.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Metal(0.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(0.5),
        )),
    ]);
    let mut out_file = File::create("render.ppm").unwrap();

    let nx = 800;
    let ny = 400;
    let ns = 100;
    out_file
        .write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();
    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::zero();
            // Anti aliasing:
            for _ in 0..ns {
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &surfaces, 0);
            }
            color = color.scale(1.0 / ns as f32);
            color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());
            color = color.scale(255.99);

            out_file
                .write(format!("{} {} {}\n", color.r(), color.g(), color.b()).as_bytes())
                .unwrap();
        }
    }
}
