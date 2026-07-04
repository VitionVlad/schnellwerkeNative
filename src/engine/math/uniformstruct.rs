use self::{mat4::Mat4, vec2::Vec2, vec3::Vec3, vec4::Vec4};

use super::*;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Usages{
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Uniformstruct{
    pub usage: Usages,
    pub float: f32,
    pub vec2: Vec2,
    pub vec3: Vec3,
    pub vec4: Vec4,
    pub mat: Mat4,
    pub label: String,
}

#[allow(dead_code)]
pub fn createfloat(value: f32, label: &str) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Float,
        float: value,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
        label: label.to_string(),
    }
}

#[allow(dead_code)]
pub fn createvec2(value: Vec2, label: &str) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec2,
        float: 0.0f32,
        vec2: value,
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
        label: label.to_string(),
    }
}

#[allow(dead_code)]
pub fn createvec3(value: Vec3, label: &str) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec3,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: value,
        vec4: Vec4::new(),
        mat: Mat4::new(),
        label: label.to_string(),
    }
}

#[allow(dead_code)]
pub fn createvec4(value: Vec4, label: &str) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec4,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: value,
        mat: Mat4::new(),
        label: label.to_string(),
    }
}

#[allow(dead_code)]
pub fn createmat(value: Mat4, label: &str) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Mat,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: value,
        label: label.to_string(),
    }
}

#[allow(dead_code)]
pub fn getsize(uniforms: &Vec<Uniformstruct>) -> i32{
    let mut size: i32 = 224;
    for i in 0..uniforms.len(){
        match uniforms[i].usage {
            Usages::Float => size += 4,
            Usages::Vec2 => size += 8,
            Usages::Vec3 => size += 12,
            Usages::Vec4 => size += 16,
            Usages::Mat => size += 64,
        }
    }
    return size;
}