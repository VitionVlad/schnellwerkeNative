#![allow(dead_code)]
#![allow(unused_variables)]

use super::{camera::Camera, light::Light, math::vec3::Vec3, physics::PhysicsObject, render::render::{Control, Render}};

#[derive(Copy, Clone)]
pub struct Engine{
    pub render: Render,
    pub control: Control,
    pub cameras: [Camera; 10],
    pub used_camera_count: u32,
    pub lights: [Light; 100],
    pub used_light_count: u32,
    pub physics_tick: u32,
    cumulated_time: f32,
    pub times_to_calculate_physics: u32,
}

impl Engine{
    pub fn new() -> Engine{
        let rn = Render::new();
        Engine { 
            render: rn,
            control: Control::new(rn), 
            cameras: [Camera{ physic_object: PhysicsObject::new(vec![Vec3::newdefined(0.25, 0f32, 0.25), Vec3::newdefined(-0.25, -5f32, -0.25)], false), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false, rotation_colision_calc: false }; 10],
            used_camera_count: 1,
            lights: [Light::new(super::light::LightType::Directional); 100],
            used_light_count: 1,
            physics_tick: 4,
            cumulated_time: 0.0,
            times_to_calculate_physics: 0,
        }
    }
    pub fn work(&mut self) -> bool{
        self.cumulated_time += self.render.frametime;
        self.times_to_calculate_physics = (self.cumulated_time/self.physics_tick as f32).floor() as u32;
        if self.times_to_calculate_physics >= 1{
            self.cumulated_time -= (self.times_to_calculate_physics * self.physics_tick) as f32;
        }
        self.render.camera_count = self.used_camera_count;
        self.render.shadow_map_count = self.used_light_count;
        for i in 0..u32::min(self.used_camera_count, 10){
            self.cameras[i as usize].physic_object.reset_states();
            for _ in 0..self.times_to_calculate_physics {
                self.cameras[i as usize].physic_object.exec();
            }
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
        for i in 0..u32::min(self.used_light_count, 100){
            let mt = self.lights[i as usize].getvec();
            for j in 0..16{
                self.render.set_shadow_uniform_data(j+i*16, mt[j as usize]);
            }
            self.render.set_shadow_uniform_data(i*4+1600, self.lights[i as usize].pos.x);
            self.render.set_shadow_uniform_data(i*4+1601, self.lights[i as usize].pos.y);
            self.render.set_shadow_uniform_data(i*4+1602, self.lights[i as usize].pos.z);
            self.render.set_shadow_uniform_data(i*4+1603, 0.0);
            self.render.set_shadow_uniform_data(i*4+2000, self.lights[i as usize].color.x);
            self.render.set_shadow_uniform_data(i*4+2001, self.lights[i as usize].color.y);
            self.render.set_shadow_uniform_data(i*4+2002, self.lights[i as usize].color.z);
            self.render.set_shadow_uniform_data(i*4+2003, 0.0);
        }
        self.control.get_mouse_pos();
        return self.render.continue_loop();
    }
    pub fn end(&mut self){
        self.render.destroy();
    }
}