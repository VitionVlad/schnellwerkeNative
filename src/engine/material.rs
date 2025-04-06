use super::{engine::Engine, render::render::{CullMode, Materialc}};

pub struct Material{
    pub material: Materialc,
}

impl Material{
    pub fn new(eng: Engine, vert: Vec<u8>, frag: Vec<u8>, cullmode: CullMode) -> Material{
        Material{
            material: Materialc::new(eng.render, vert, frag, cullmode),
        }
    }
}