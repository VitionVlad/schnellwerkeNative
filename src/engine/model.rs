use super::{engine::Engine, render::render::Vertexes};

#[warn(dead_code)]
#[derive(Copy, Clone)]
pub struct Model{
    pub vertexbuf: Vertexes,
}

#[warn(dead_code)]
impl Model{
    #[warn(dead_code)]
    pub fn new(engine: Engine, vertices: Vec<f32>) -> Model{
        Model { 
            vertexbuf: Vertexes::new(engine.render, vertices)
        }        
    }
}