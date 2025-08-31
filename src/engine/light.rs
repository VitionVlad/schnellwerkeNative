use core::f32;

use super::{camera::Camera, math::vec3::Vec3, physics::PhysicsObject};

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum LightType{
    Directional,
    Spot,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Light{
    pub light_type: LightType,
    pub pos: Vec3,
    pub rot: Vec3,
    pub color: Vec3,
    pub shadow: bool,
    cameras: Camera,
}

impl Light{
    #[allow(dead_code)]
    pub fn new(light_type: LightType) -> Light{
        Light{
            light_type: light_type,
            pos: Vec3::new(),
            rot: Vec3::new(),
            color: Vec3::newdefined(1f32, 1f32, 1f32),
            shadow: true,
            cameras: Camera{ physic_object: PhysicsObject::new(vec![Vec3::new(), Vec3::new()], true), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: light_type == LightType::Directional, rotation_colision_calc: false },
        }
    }
    #[allow(dead_code)]
    pub fn getvec(&mut self) -> Vec<f32>{
        self.cameras.physic_object.pos = self.pos;
        self.cameras.physic_object.rot = self.rot;
        self.cameras.is_orthographic = self.light_type == LightType::Directional; 
        return self.cameras.get_projection(1f32).mat.to_vec();
    }
}