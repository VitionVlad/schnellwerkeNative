use std::fs;

use engine::{engine::Engine, material::Material, render::render::Model};
mod engine;

fn main() {
    let mut eng = Engine::new();
    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let mut _mat = Material::new(eng, vert, frag, engine::render::render::CullMode::CullModeNone);

    let vert = vec![
        0.0f32, -0.5f32, 0f32, 
        0.5f32, 0.5f32, 0f32,
        -0.5f32, 0.5f32, 0f32, 
        0f32, 1f32,
        1f32, 0f32,
        1f32, 1f32,
        0f32, 0f32, 0f32,
        0f32, 0f32, 0f32,
        0f32, 0f32, 0f32,
    ];

    let _model = Model::new(eng.render, vert);
    while eng.work(){
    }
    eng.end();
}
