#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::{audio::audio::Sound, engine::Engine, math::{vec2::Vec2, vec3::Vec3}};

pub struct Speaker{
    sound: Sound,
    pub pos: Vec3,
    pub play: bool,
    pub power: f32,
    pub use_pan: bool,
    pub pos_dependency: bool,
    pub volume: f32,
}

impl Speaker{
    pub fn new(eng: &mut Engine, path: &str) -> Speaker{
        Speaker{
            sound: Sound::new(eng.audio, path),
            pos: Vec3::new(),
            play: true,
            power: 20f32,
            use_pan: true,
            pos_dependency: true,
            volume: 1.0f32,
        }
    }
    fn calcpan(src: Vec2, listp: Vec2, listr: Vec2) -> f32{
        let vec_d = Vec2::newdefined(src.x - listp.x, src.y - listp.y);
        let vec_r_perp = Vec2::newdefined(listr.y, listr.x);
        let dot = vec_d.x * vec_r_perp.x + vec_d.y * vec_r_perp.y;
        let magnitude_d = (vec_d.x.powi(2) + vec_d.y.powi(2)).sqrt();
        if magnitude_d == 0.0 {
            0.0
        } else {
            dot / magnitude_d
        }
    }
    pub fn exec(&mut self, eng: &mut Engine){
        if self.pos_dependency{
            self.sound.vol = f32::max((self.power - f32::sqrt(f32::powi(eng.cameras[eng.primary_camera].physic_object.pos.x - self.pos.x, 2) + f32::powi(eng.cameras[eng.primary_camera].physic_object.pos.y - self.pos.y, 2) + f32::powi(eng.cameras[eng.primary_camera].physic_object.pos.z - self.pos.z, 2)))/self.power * self.volume, 0.0f32);
        }else{
            self.sound.vol = self.volume;
        }
        if self.use_pan{
            self.sound.pan = -Self::calcpan(Vec2::newdefined(self.pos.x, self.pos.z), Vec2::newdefined(eng.cameras[eng.primary_camera].physic_object.pos.x, eng.cameras[eng.primary_camera].physic_object.pos.z), Vec2::newdefined(eng.cameras[eng.primary_camera].physic_object.rot.y.sin(), eng.cameras[eng.primary_camera].physic_object.rot.y.cos()));
        }else{
            self.sound.pan = 0f32;
        }
        if self.play{
            self.sound.play();
        }else{
            self.sound.stop();
        }
    }
}