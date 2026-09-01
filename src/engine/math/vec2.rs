use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec2{
    pub x: f32,
    pub y: f32
}

impl Vec2{
    #[allow(dead_code)]
    pub fn new() -> Vec2{
        Vec2 { x: 0.0f32, y: 0.0f32 }
    }
    #[allow(dead_code)]
    pub fn dot(self, other: Vec2) -> f32 {
        self.x*other.x + self.y*other.y
    }
    #[allow(dead_code)]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
    #[allow(dead_code)]
    pub fn vec_vec_angle(self, other: Vec2) -> f32 {
        self.dot(other)/(self.magnitude()*other.magnitude())
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}