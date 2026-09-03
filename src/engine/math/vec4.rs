use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::engine::math::vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4{
    #[allow(dead_code)]
    pub fn new() -> Vec4{
        Vec4 { x: 0.0f32, y: 0.0f32, z: 0.0f32, w: 0.0f32 }
    }
    #[allow(dead_code)]
    pub fn magnitude(self) -> f32{
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if magnitude > f32::EPSILON {
            self.x /= magnitude;
            self.y /= magnitude;
            self.z /= magnitude;
            self.w /= magnitude;
        }
    }
    #[allow(dead_code)]
    pub fn to_euler(&self) -> Vec3 {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);

        let xx = x*x;
        let yy = y*y;
        let zz = z*z;

        let m13 = 2.0*(x*z + w*y);
        let m23 = 2.0*(y*z - w*x);
        let m33 = 1.0 - 2.0*(xx + yy);
        let m12 = 2.0*(x*y - w*z);
        let m11 = 1.0 - 2.0*(yy + zz);
        let m32 = 2.0*(y*z + w*x);
        let m22 = 1.0 - 2.0*(xx + zz);

        let m13_c = m13.clamp(-1.0, 1.0);
        let pitch = m13_c.asin();

        let (roll, yaw) = if m13_c.abs() < 0.9999999 {
            (f32::atan2(-m23, m33), f32::atan2(-m12, m11))
        } else {
            (f32::atan2(m32, m22), 0.0)
        };

        Vec3 { x: roll, y: pitch, z: yaw }
    }
    #[allow(dead_code)]
    pub fn quat_direction(&self) -> Vec3 {
        let x = 2.0 * (self.x * self.z + self.w * self.y);
        let y = 2.0 * (self.y * self.z - self.w * self.x);
        let z = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);

        Vec3 { x, y, z }
    }
    #[allow(dead_code)]
    pub fn quat_direction_up(&self) -> Vec3 {
        let x = 2.0 * (self.x * self.y - self.w * self.z);
        let y = 1.0 - 2.0 * (self.x * self.x + self.z * self.z);
        let z = 2.0 * (self.y * self.z + self.w * self.x);

        Vec3 { x, y, z }
    }
    #[allow(dead_code)]
    pub fn quat_direction_right(&self) -> Vec3 {
        let x = 1.0 - 2.0 * (self.z * self.z + self.y * self.y);
        let y = 2.0 * (self.x * self.y + self.w * self.z);
        let z = 2.0 * (self.x * self.z - self.w * self.y);

        Vec3 { x, y, z }
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Mul for Vec4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl Div for Vec4 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}