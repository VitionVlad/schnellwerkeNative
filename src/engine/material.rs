#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, render::render::{CullMode, MaterialShaders}};

#[derive(Copy, Clone)]
pub struct Material{
    pub material_shaders: MaterialShaders,
}

impl Material{
    pub fn new(eng: &Engine, vert: Vec<u8>, frag: Vec<u8>, shadow: Vec<u8>, cullmodes: [CullMode; 2]) -> Material{
        Material{
            material_shaders: MaterialShaders::new(eng.render, vert, frag, shadow, cullmodes[0], cullmodes[1])
        }
    }
}