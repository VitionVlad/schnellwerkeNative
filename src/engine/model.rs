#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, math::vec3::Vec3, physics::physics::getpoints, render::render::Vertexes};

#[derive(Copy, Clone)]
pub struct Model{
    pub vertexbuf: Vertexes,
    pub points: [Vec3; 2],
}

impl Model{
    pub fn new(engine: &Engine, vertices: Vec<f32>) -> Model{
        let pt = getpoints(vertices.clone());
        Model { 
            vertexbuf: Vertexes::new(engine.render, vertices),
            points: [pt[0], pt[1]],
        }        
    }
    pub fn init(&mut self, engine: &Engine, vertices: Vec<f32>){
        let pt = getpoints(vertices.clone());
        self.vertexbuf.destroy(engine.render);
        self.vertexbuf.init(engine.render, vertices);
    }
}