use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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