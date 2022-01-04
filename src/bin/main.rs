use rust_raytracer::{
    camera::Camera,
    image::Image,
    surfaces::{Material, Sphere, Surfaces},
    vec3::Vec3,
};

fn main() {
    let camera = Camera::default();
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
            Material::Dielectric {
                refraction_k: 1.5,
            },
        )),
    ]);

    // let mut image = Image::new(1800, 900);
    let mut image = Image::new(800, 400);
    image.render(&camera, &surfaces, 12);
    image.to_ppm("./render.ppm".to_string());
}
