use std::f32::consts::PI;

use rand::Rng;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).get_unit();
        let u = vup.cross(&w).get_unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let corner = origin
            - u.scale(half_width * focus_dist)
            - v.scale(half_height * focus_dist)
            - w.scale(focus_dist);
        let horizontal = u.scale(2.0 * half_width * focus_dist);
        let vertical = v.scale(2.0 * half_height * focus_dist);

        Self {
            corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            u,
            v,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = rnd_vec_in_disc().scale(self.lens_radius);
        let offset = self.u.scale(rd.x()) + self.v.scale(rd.y());

        let horizontal = self.horizontal.scale(u);
        let vertical = self.vertical.scale(v);
        let origin = self.origin + offset;
        let direction = (self.corner + horizontal + vertical - self.origin - offset).get_unit();
        Ray::new(origin, direction)
    }
}

fn rnd_vec_in_disc() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    let ones = Vec3::new(1.0, 1.0, 0.0);
    loop {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0);
        p -= ones;
        if p.length() < 1.0 {
            break p;
        }
    }
}
