use std::fs;

use engine::{engine::Engine, material::Material, model::Model, object::Object};
mod engine;

fn main() {
    let mut eng = Engine::new();
    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let mat = Material::new(eng, vert, frag, engine::render::render::CullMode::CullModeNone);

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

    let model = Model::new(eng, vert);
    let mesh = Object::new(eng, model, mat);
    while eng.work(){
        mesh.exec();
    }
    eng.end();
}
