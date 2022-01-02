use crate::{
    ray::Ray,
    surfaces::{HitRes, Material, Surfaces},
    vec3::Vec3,
};
use rand::Rng;

pub fn scatter_lambertian(hit_res: &HitRes) -> Option<Ray> {
    let rnd_vec_in_sphere = {
        let mut rng = rand::thread_rng();
        let ones = Vec3::new(1.0, 1.0, 1.0);
        loop {
            let mut p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()).scale(2.0);
            p -= ones;
            if p.squared_length() < 1.0 {
                break p;
            }
        }
    };

    let direction = (hit_res.norm + rnd_vec_in_sphere).get_unit();
    let scattered = Ray::new(hit_res.point, direction);
    Some(scattered)
}

pub fn scatter_metal(inp_ray: &Ray, hit_res: &HitRes) -> Option<Ray> {
    let reflected = inp_ray.direction - hit_res.norm.scale(inp_ray.direction.dot(&hit_res.norm));
    let scattered = Ray::new(hit_res.point, reflected.get_unit());
    if scattered.direction.dot(&hit_res.norm) > 0.0 {
        return Some(scattered);
    }
    None
}

pub fn get_color(ray: &Ray, surfaces: &Surfaces, depth: i32) -> Vec3 {
    if let Some(hit_res) = surfaces.hit(ray, 0.001, f32::MAX) {
        let attenuation: f32;
        if depth < 50 {
            if let Some(scattered) = match hit_res.material {
                Material::Lambertian(a) => {
                    attenuation = a;
                    scatter_lambertian(&hit_res)
                }
                Material::Metal(a) => {
                    attenuation = a;
                    scatter_metal(ray, &hit_res)
                }
            } {
                return get_color(&scattered, surfaces, depth + 1).scale(attenuation);
            };
        }
        return Vec3::zero();
    } else {
        let t = 0.5 * (ray.direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0).scale(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t);
    }
}
