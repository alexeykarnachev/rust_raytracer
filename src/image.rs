use crossbeam::scope;
use std::{fs::File, io::Write};

use rand::Rng;

use crate::{
    camera::Camera,
    ray::Ray,
    scatters::{scatter_dielectric, scatter_lambertian, scatter_metal},
    surfaces::{Material, Surfaces},
    vec3::Vec3,
};

pub struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        for y in (0..height).rev() {
            for x in 0..width {
                let (u, v) = (x as f32 / width as f32, y as f32 / height as f32);
                pixels.push(Pixel::new(u, v))
            }
        }
        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn render(&mut self, camera: &Camera, surfaces: &Surfaces, n_threads: usize) {
        let chunk_size = self.height * self.width / n_threads;
        scope(|s| {
            for pixels in self.pixels.chunks_mut(chunk_size) {
                s.spawn(|_| {
                    for pixel in pixels.iter_mut() {
                        pixel.render(camera, surfaces);
                    }
                });
            }
        })
        .unwrap();
    }

    pub fn to_ppm(&self, file_path: String) {
        let mut out_file = File::create(file_path).unwrap();

        out_file
            .write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())
            .unwrap();

        for pixel in self.pixels.iter() {
            let color = pixel.color.unwrap();
            out_file
                .write(format!("{} {} {}\n", color.r(), color.g(), color.b()).as_bytes())
                .unwrap();
        }
    }
}

struct Pixel {
    u: f32,
    v: f32,
    color: Option<Vec3>,
}

const N_AA_STEPS: usize = 100;
const AA_STRENGTH: f32 = 0.002;

impl Pixel {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v, color: None }
    }

    pub fn render(&mut self, camera: &Camera, surfaces: &Surfaces) {
        let mut color = Vec3::zeros();
        let mut rng = rand::thread_rng();
        for _ in 0..N_AA_STEPS {
            let v = self.v as f32 + AA_STRENGTH * rng.gen::<f32>();
            let u = self.u as f32 + AA_STRENGTH * rng.gen::<f32>();
            let ray = camera.get_ray(u, v);
            color += get_color(&ray, &surfaces, 0);
        }
        color = color.scale(1.0 / N_AA_STEPS as f32);

        color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());
        color = color.scale(255.99);
        self.color = Some(color);
    }
}

fn get_color(ray: &Ray, surfaces: &Surfaces, depth: i32) -> Vec3 {
    if let Some(hit_res) = surfaces.hit(ray, 0.001, f32::MAX) {
        let att: Vec3;
        if depth < 50 {
            if let Some(scattered) = match hit_res.material {
                Material::Lambertian { attenuation } => {
                    att = attenuation;
                    scatter_lambertian(&hit_res)
                }
                Material::Metal { attenuation, fuzz } => {
                    att = attenuation;
                    scatter_metal(ray, &hit_res, fuzz)
                }
                Material::Dielectric { ref_k } => {
                    att = Vec3::ones();
                    scatter_dielectric(ray, &hit_res, ref_k)
                }
            } {
                return att * get_color(&scattered, surfaces, depth + 1);
            };
        }
        return Vec3::zeros();
    } else {
        let t = 0.5 * (ray.direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0).scale(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t);
    }
}
