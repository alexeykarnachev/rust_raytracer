use crate::{ray::Ray, surfaces::HitRes, vec3::Vec3};
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
