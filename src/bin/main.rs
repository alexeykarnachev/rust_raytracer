use rust_raytracer::{
    camera::Camera,
    image::Image,
    surfaces::{Material, Sphere, Surfaces},
    vec3::Vec3,
};

fn main() {
    let nx = 800;
    let ny = 400;
    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        nx as f32 / ny as f32,
        0.0,
        (lookfrom - lookat).length(),
    );

    let surfaces = Surfaces::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian {
                attenuation: Vec3::new(0.1, 0.2, 0.5),
            },
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian {
                attenuation: Vec3::new(0.8, 0.8, 0.0),
            },
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal {
                attenuation: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.2,
            },
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::Dielectric { ref_k: 2.0 },
        )),
    ]);

    let mut image = Image::new(nx, ny);
    image.render(&camera, &surfaces, 12);
    image.to_ppm("./render.ppm".to_string());
}
