use crate::{ray::Ray, surfaces::HitRes, vec3::Vec3};
use rand::Rng;

fn rnd_vec_in_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    let ones = Vec3::ones();
    loop {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()).scale(2.0);
        p -= ones;
        if p.length() < 1.0 {
            break p;
        }
    }
}

pub fn scatter_lambertian(hit_res: &HitRes) -> Option<Ray> {
    let direction = (hit_res.norm + rnd_vec_in_sphere()).get_unit();
    let scattered = Ray::new(hit_res.point, direction);
    Some(scattered)
}

pub fn scatter_metal(inp_ray: &Ray, hit_res: &HitRes, fuzz: f32) -> Option<Ray> {
    let reflected = reflect(&inp_ray.direction, &hit_res.norm);
    let direction = (reflected.get_unit() + rnd_vec_in_sphere().scale(fuzz)).get_unit();

    let scattered = Ray::new(hit_res.point, direction);
    if scattered.direction.dot(&hit_res.norm) > 0.0 {
        return Some(scattered);
    }
    None
}

pub fn scatter_dielectric(inp_ray: &Ray, hit_res: &HitRes, ref_k: f32) -> Option<Ray> {
    let ni_over_nt: f32;
    let outward_norm: Vec3;
    let cosine: f32;
    let reflect_prob: f32;
    let reflected = reflect(&inp_ray.direction, &hit_res.norm);

    if inp_ray.direction.dot(&hit_res.norm) > 0.0 {
        outward_norm = -hit_res.norm;
        ni_over_nt = ref_k;
        cosine = ref_k * inp_ray.direction.dot(&hit_res.norm) / inp_ray.direction.length();
    } else {
        outward_norm = hit_res.norm;
        ni_over_nt = 1.0 / ref_k;
        cosine = -inp_ray.direction.dot(&hit_res.norm) / inp_ray.direction.length();
    }

    if let Some(refracted) = refract(&inp_ray.direction, &outward_norm, ni_over_nt) {
        reflect_prob = schlick(cosine, ref_k);
        let mut rng = rand::thread_rng();
        if reflect_prob > rng.gen::<f32>() {
            return Some(Ray::new(hit_res.point, reflected.get_unit()));
        } else {
            return Some(Ray::new(hit_res.point, refracted.get_unit()));
        }
    } else {
        return Some(Ray::new(hit_res.point, reflected.get_unit()));
    };
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - n.scale(2.0 * v.dot(n))
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.get_unit();
    let dt = uv.dot(n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        let refracted = (uv - n.scale(dt)).scale(ni_over_nt) - n.scale(d.sqrt());
        return Some(refracted);
    } else {
        return None;
    };
}

fn schlick(cosine: f32, ref_k: f32) -> f32 {
    let mut r0 = (1.0 - ref_k) / (1.0 + ref_k);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
