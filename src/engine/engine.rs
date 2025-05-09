#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp;

use super::{camera::Camera, light::Light, math::vec3::Vec3, physics::PhysicsObject, render::render::{Control, Render}};

#[derive(Copy, Clone)]
pub struct Engine{
    pub render: Render,
    pub control: Control,
    pub cameras: [Camera; 10],
    pub used_camera_count: u32,
    pub lights: [Light; 100],
    pub used_light_count: u32,
}

impl Engine{
    pub fn new() -> Engine{
        let rn = Render::new();
        Engine { 
            render: rn,
            control: Control::new(rn), 
            cameras: [Camera{ physic_object: PhysicsObject::new(vec![Vec3::newdefined(0.1, 0f32, 0.1), Vec3::newdefined(-0.1, -5f32, -0.1)], false), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false, rotation_colision_calc: false }; 10],
            used_camera_count: 1,
            lights: [Light::new(super::light::LightType::Directional); 100],
            used_light_count: 1,
        }
    }
    pub fn work(&mut self) -> bool{
        self.render.camera_count = self.used_camera_count;
        self.render.shadow_map_count = self.used_light_count;
        for i in 0..cmp::min(self.used_camera_count, 10){
            self.cameras[i as usize].physic_object.frametime = self.render.frametime;
            self.cameras[i as usize].physic_object.reset_states();
            self.cameras[i as usize].physic_object.exec();
            let mt = self.cameras[i as usize].get_projection(self.render.resolution_x as f32/self.render.resolution_y as f32);
            for j in 0..16{
                self.render.set_deffered_uniform_data(j+i*16, mt.mat[j as usize]);
            }
            self.render.set_deffered_uniform_data(i*4+160, self.cameras[i as usize].physic_object.pos.x);
            self.render.set_deffered_uniform_data(i*4+161, self.cameras[i as usize].physic_object.pos.y);
            self.render.set_deffered_uniform_data(i*4+162, self.cameras[i as usize].physic_object.pos.z);
            self.render.set_deffered_uniform_data(i*4+163, 0.0);
            self.render.set_deffered_uniform_data(i*4+200, self.cameras[i as usize].physic_object.rot.x);
            self.render.set_deffered_uniform_data(i*4+201, self.cameras[i as usize].physic_object.rot.y);
            self.render.set_deffered_uniform_data(i*4+202, self.cameras[i as usize].physic_object.rot.z);
            self.render.set_deffered_uniform_data(i*4+203, 0.0);
        }
        self.control.get_mouse_pos();
        return self.render.continue_loop();
    }
    pub fn end(&mut self){
        self.render.destroy();
    }
}