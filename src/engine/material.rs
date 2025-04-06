use super::{engine::Engine, render::render::{CullMode, MaterialShaders}};

pub struct Material{
    pub material_shaders: MaterialShaders,
}

impl Material{
    pub fn new(eng: Engine, vert: Vec<u8>, frag: Vec<u8>, cullmode: CullMode) -> Material{
        Material{
            material_shaders: MaterialShaders::new(eng.render, vert, frag, cullmode),
        }
    }
}