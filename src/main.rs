use std::fs;

use engine::{engine::Engine, material::Material, model::Model, object::Object, render::render::Texture};
mod engine;

fn main() {
    let mut eng = Engine::new();

    let img: Vec<i8> = vec![
        0, 0, 0, 127,
        127, 127, 127, 127,
        127, 127, 127, 127,
        0, 0, 0, 127,

        127, 127, 127, 127,
        0, 0, 0, 127,
        0, 0, 0, 127,
        127, 127, 127, 127,
    ];

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let mut mat = Material::new(eng, vert, frag, engine::render::render::CullMode::CullModeNone);
    mat.textures.push(Texture::new(eng.render, 2, 2, 2, img));

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
