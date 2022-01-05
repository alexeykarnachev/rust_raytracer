use rust_raytracer::{camera::Camera, image::Image, surfaces::Surfaces, vec3::Vec3};

fn main() {
    let n_threads = 12;
    let nx = 400;
    let ny = 200;
    let vfov = 30.0;
    let lookfrom = Vec3::new(15.0, 5.0, 5.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        nx as f32 / ny as f32,
        0.2,
        (lookfrom - lookat).length(),
    );

    let surfaces = Surfaces::generate();
    let mut image = Image::new(nx, ny);
    image.render(&camera, &surfaces, n_threads);
    image.to_ppm("./render.ppm".to_string());
}
