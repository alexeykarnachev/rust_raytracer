use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        if !direction.is_unit() {
            panic!("direction must be unit!");
        }
        Ray { origin, direction }
    }

    pub fn get_point(&self, distance: f32) -> Vec3 {
        self.origin + self.direction.scale(distance)
    }
}
