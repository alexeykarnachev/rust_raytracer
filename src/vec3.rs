use std::{
    fmt::Display,
    ops::{Add, AddAssign, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};

const MAX_UNIT_EPS: f32 = 1.0e-5;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { data: [x, y, z] }
    }

    pub fn zeros() -> Self {
        Vec3 { data: [0.0; 3] }
    }

    pub fn ones() -> Self {
        Vec3 { data: [1.0; 3] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }
    pub fn y(&self) -> f32 {
        self.data[1]
    }
    pub fn z(&self) -> f32 {
        self.data[2]
    }
    pub fn r(&self) -> i32 {
        self.data[0] as i32
    }
    pub fn g(&self) -> i32 {
        self.data[1] as i32
    }
    pub fn b(&self) -> i32 {
        self.data[2] as i32
    }

    pub fn scale(&self, k: f32) -> Self {
        Vec3::new(self.x() * k, self.y() * k, self.z() * k)
    }

    pub fn squared_length(&self) -> f32 {
        let mut len: f32 = 0.0;
        for i in 0..3 {
            let val = self.data[i];
            len += val * val;
        }
        len
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        let mut res: f32 = 0.0;
        for i in 0..3 {
            res += self.data[i] * other.data[i];
        }
        res
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            -(self.x() * other.z() - self.z() * other.x()),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn get_unit(&self) -> Self {
        let len = self.length();
        self.scale(1.0 / len)
    }

    pub fn get_abs(&self) -> Self {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn is_unit(&self) -> bool {
        (self.length() - 1.0).abs() <= MAX_UNIT_EPS
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4} {:.4} {:.4}", self.x(), self.y(), self.z())
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.data[0], -self.data[1], -self.data[2])
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.data[i] *= rhs.data[i];
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.data[i] /= rhs.data[i];
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.data[i] += rhs.data[i];
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.data[i]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fields(vec: &Vec3) {
        assert_eq!(vec.x() as i32, vec.r());
        assert_eq!(vec.y() as i32, vec.g());
        assert_eq!(vec.z() as i32, vec.b());
    }

    #[test]
    fn test_fields() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
        assert_fields(&vec);
    }

    #[test]
    fn test_operators() {
        let vec_1 = Vec3::new(-1.0, -2.0, -3.0);
        let mut vec_1 = -vec_1;
        assert_eq!(vec_1.x(), 1.0);
        assert_eq!(vec_1.y(), 2.0);
        assert_eq!(vec_1.z(), 3.0);

        let vec_2 = Vec3::new(1.0, 2.0, 3.0);
        vec_1 += vec_2.clone();
        assert_eq!(vec_1.x(), 2.0);
        assert_eq!(vec_1.y(), 4.0);
        assert_eq!(vec_1.z(), 6.0);

        let vec_3 = vec_1 + vec_2;
        assert_eq!(vec_3.x(), 3.0);
        assert_eq!(vec_3.y(), 6.0);
        assert_eq!(vec_3.z(), 9.0);

        let mut vec_3 = vec_1 - vec_2;
        assert_eq!(vec_3.x(), 1.0);
        assert_eq!(vec_3.y(), 2.0);
        assert_eq!(vec_3.z(), 3.0);
        vec_3 += vec_2;

        vec_1 -= vec_2.clone();
        assert_eq!(vec_1.x(), 1.0);
        assert_eq!(vec_1.y(), 2.0);
        assert_eq!(vec_1.z(), 3.0);

        vec_1 *= vec_2.clone();
        assert_eq!(vec_1.x(), 1.0);
        assert_eq!(vec_1.y(), 4.0);
        assert_eq!(vec_1.z(), 9.0);

        vec_1 /= vec_2.clone();
        assert_eq!(vec_1.x(), 1.0);
        assert_eq!(vec_1.y(), 2.0);
        assert_eq!(vec_1.z(), 3.0);

        assert_eq!(vec_1, Vec3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_methods() {
        let mut vec = Vec3::new(0.5, 1.0, 1.0);
        vec = vec.scale(2.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 2.0);

        assert_eq!(vec.length(), 3.0);
        assert_eq!(vec.squared_length(), 9.0);
        assert_eq!(format!("{}", vec), "1.0000 2.0000 2.0000");
        assert_eq!(vec.dot(&vec), 9.0);

        vec = vec.get_unit();
        assert_eq!(vec.length(), 1.0);

        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(vec1.cross(&vec2), Vec3::new(-3.0, 6.0, -3.0));
    }
}
