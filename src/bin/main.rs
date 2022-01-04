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

    let mut image = Image::new(800, 400);
    image.render(&camera, &surfaces);
    image.to_ppm("./render.ppm".to_string());
}
