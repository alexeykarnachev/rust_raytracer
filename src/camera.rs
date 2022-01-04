use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let horizontal = self.horizontal.scale(u);
        let vertical = self.vertical.scale(v);
        let direction = (self.corner + horizontal + vertical - self.origin).get_unit();
        Ray::new(self.origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::zeros(),
        }
    }
}
