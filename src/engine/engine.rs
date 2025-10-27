#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::light::LightType;

use super::{camera::Camera, light::Light, math::vec3::Vec3, physics::PhysicsObject, render::render::{Control, Render}, audio::audio::AudioEngine};

#[derive(Clone)]
pub struct Engine{
    pub render: Render,
    pub audio: AudioEngine,
    pub control: Control,
    pub cameras: [Camera; 10],
    pub used_camera_count: u32,
    pub lights: [Light; 100],
    pub used_light_count: u32,
    pub physics_tick: u32,
    cumulated_time: f32,
    pub times_to_calculate_physics: u32,
    pub obj_ph: Vec<PhysicsObject>,
    pub fps: u32,
    cfps: u32,
    tmpassed: f32,
    pub primary_camera: usize,
    pub object_object_interaction: bool,
}

impl Engine{
    pub fn new() -> Engine{
        let rn = Render::new();
        Engine { 
            render: rn,
            audio: AudioEngine::new(),
            control: Control::new(rn), 
            cameras: [Camera{ physic_object: PhysicsObject::new(vec![Vec3::newdefined(0.15, 0f32, 0.15), Vec3::newdefined(-0.15, -2f32, -0.15)], false), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false, rotation_colision_calc: false }; 10],
            used_camera_count: 1,
            lights: [Light::new(super::light::LightType::Directional); 100],
            used_light_count: 1,
            physics_tick: 4,
            cumulated_time: 0.0,
            times_to_calculate_physics: 0,
            obj_ph: vec![],
            fps: 0,
            cfps: 0,
            tmpassed: 0.0,
            primary_camera: 0,
            object_object_interaction: true,
        }
    }
    pub fn work(&mut self) -> bool{
        self.audio.exec();
        self.cumulated_time += self.render.frametime;
        self.tmpassed += self.render.frametime;
        self.cfps += 1;
        if self.tmpassed >= 1000.0 {
            self.fps = self.cfps;
            self.cfps = 0;
            self.tmpassed = 0.0;
        }
        self.times_to_calculate_physics = (self.cumulated_time/self.physics_tick as f32).round() as u32;
        if self.times_to_calculate_physics >= 1{
            self.cumulated_time -= (self.times_to_calculate_physics * self.physics_tick) as f32;
        }
        self.render.camera_count = self.used_camera_count;
        self.render.shadow_map_count = self.used_light_count;
        for i in 0..u32::min(self.used_camera_count, 10){
            let mt = self.cameras[i as usize].get_projection(self.render.resolution_x as f32/self.render.resolution_y as f32);
            for j in 0..16{
                self.render.set_deffered_uniform_data(j+i*16, mt.mat[j as usize]);
            }
            let invmt = mt.inverse();
            for j in 0..16{
                self.render.set_deffered_uniform_data(j+i*16+160, invmt.mat[j as usize]);
            }
            self.render.set_deffered_uniform_data(i*4+320, self.cameras[i as usize].physic_object.pos.x);
            self.render.set_deffered_uniform_data(i*4+321, self.cameras[i as usize].physic_object.pos.y);
            self.render.set_deffered_uniform_data(i*4+322, self.cameras[i as usize].physic_object.pos.z);
            self.render.set_deffered_uniform_data(i*4+323, 0.0);
            self.render.set_deffered_uniform_data(i*4+360, self.cameras[i as usize].physic_object.rot.x);
            self.render.set_deffered_uniform_data(i*4+361, self.cameras[i as usize].physic_object.rot.y);
            self.render.set_deffered_uniform_data(i*4+362, self.cameras[i as usize].physic_object.rot.z);
            self.render.set_deffered_uniform_data(i*4+363, 0.0);
        }
        for i in 0..u32::min(self.used_light_count, 100){
            let mt = self.lights[i as usize].getvec();
            for j in 0..16{
                self.render.set_shadow_uniform_data(j+i*16, mt[j as usize]);
            }
            if self.lights[i as usize].light_type == LightType::Spot{
                self.render.set_shadow_uniform_data(i*4+1600, self.lights[i as usize].pos.x);
                self.render.set_shadow_uniform_data(i*4+1601, self.lights[i as usize].pos.y);
                self.render.set_shadow_uniform_data(i*4+1602, self.lights[i as usize].pos.z);
                self.render.set_shadow_uniform_data(i*4+1603, 1.0f32);
            }else{
                self.render.set_shadow_uniform_data(i*4+1600, self.lights[i as usize].direction.x);
                self.render.set_shadow_uniform_data(i*4+1601, self.lights[i as usize].direction.y);
                self.render.set_shadow_uniform_data(i*4+1602, self.lights[i as usize].direction.z);
                self.render.set_shadow_uniform_data(i*4+1603, 0.0f32);
            }
            self.render.set_shadow_uniform_data(i*4+2000, self.lights[i as usize].color.x);
            self.render.set_shadow_uniform_data(i*4+2001, self.lights[i as usize].color.y);
            self.render.set_shadow_uniform_data(i*4+2002, self.lights[i as usize].color.z);
            self.render.set_shadow_uniform_data(i*4+2003, 0.0);
        }
        self.control.get_mouse_pos();
        for _ in 0..self.times_to_calculate_physics{
            for i in 0..u32::min(self.used_camera_count, 10){
                self.cameras[i as usize].physic_object.reset_states();
                self.cameras[i as usize].physic_object.exec();
            }
            for i in 0..self.obj_ph.len(){
                for j in 0..u32::min(self.used_camera_count, 10){
                    self.cameras[j as usize].physic_object.interact_with_other_object(self.obj_ph[i]);
                }
            }
        }
        return self.render.continue_loop();
    }
    pub fn end(&mut self){
        self.audio.destroy();
        self.render.destroy();
    }
}