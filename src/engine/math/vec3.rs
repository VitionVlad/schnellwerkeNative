use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::engine::math::vec4::Vec4;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3{
    #[allow(dead_code)]
    pub fn new() -> Vec3{
        Vec3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 }
    }
    pub fn cross(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn min(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    pub fn max(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
    pub fn abs(self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    #[allow(dead_code)]
    pub fn dot(self, other: Vec3) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    #[allow(dead_code)]
    pub fn length_squared(self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    #[allow(dead_code)]
    pub fn to_quat(&self) -> Vec4 {
        let cx = (self.x * 0.5).cos();
        let sx = (self.x * 0.5).sin();

        let cy = (self.y * 0.5).cos();
        let sy = (self.y * 0.5).sin();

        let cz = (self.z * 0.5).cos();
        let sz = (self.z * 0.5).sin();

        Vec4 {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz,
        }
    }
    #[allow(dead_code)]
    pub fn normalize(&self) -> Vec3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            Vec3 { x: 0.0, y: 0.0, z: 0.0 }
        } else {
            Vec3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}