use crate::{ray::Ray, vec3::Vec3};

pub trait Hit {
    fn hit(&self, ray: &Ray, t_max: f32, t_min: f32) -> Option<HitRes>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian {
        attenuation: Vec3,
    },
    Metal {
        attenuation: Vec3,
        fuzz: f32,
    },
    Dielectric {
        ref_k: f32,
    },
}

pub struct HitRes {
    pub point: Vec3,
    pub distance: f32,
    pub norm: Vec3,
    pub material: Material,
}

impl HitRes {
    pub fn new(point: Vec3, distance: f32, norm: Vec3, material: Material) -> Self {
        if !norm.is_unit() {
            panic!("norm must be unit!")
        }
        HitRes {
            point,
            distance,
            norm,
            material,
        }
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRes> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;

        if d > 0.0 {
            let distances = [(-b - d.sqrt()) / 2.0 * a, (-b + d.sqrt()) / 2.0 * a];
            for distance in distances {
                if (distance < t_max) && (distance > t_min) {
                    let point = ray.get_point(distance);
                    let norm = (point - self.center).get_unit();
                    return Some(HitRes::new(point, distance, norm, self.material));
                }
            }
        };
        None
    }
}

pub struct Surfaces {
    surfaces: Vec<Box<dyn Hit + Sync>>,
}

impl Surfaces {
    pub fn new(surfaces: Vec<Box<dyn Hit + Sync>>) -> Self {
        Self { surfaces }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRes> {
        self.surfaces
            .iter()
            .filter_map(|sphere| sphere.hit(&ray, t_min, t_max))
            .min_by_key(|hit_res| (hit_res.distance * 100000.0) as i32)
    }
}
