use super::{engine::Engine, render::render::{CullMode, MaterialShaders}};

#[warn(dead_code)]
pub struct Material{
    pub material_shaders: MaterialShaders,
}

#[warn(dead_code)]
impl Material{
    pub fn new(eng: Engine, vert: Vec<u8>, frag: Vec<u8>, cullmode: CullMode) -> Material{
        Material{
            material_shaders: MaterialShaders::new(eng.render, vert, frag, cullmode),
        }
    }
}