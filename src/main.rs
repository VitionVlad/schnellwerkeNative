use std::fs;

use engine::{engine::Engine, material::Material, model::Model, object::Object, plane::PLANE, render::render::Texture};
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
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let mut mat = Material::new(eng, vert, frag, vec![], engine::render::render::CullMode::CullModeNone);
    let mut mat2 = Material::new(eng, dvert, dfrag, shadow, engine::render::render::CullMode::CullModeNone);
    mat.textures.push(Texture::new(eng.render, 2, 2, 2, img.clone()));
    mat2.textures.push(Texture::new(eng.render, 2, 2, 2, img.clone()));

    let model = Model::new(eng, PLANE.to_vec());
    let mesh = Object::new(eng, model, mat, engine::render::render::MeshUsage::LightingPass);
    let mesh2 = Object::new(eng, model, mat2, engine::render::render::MeshUsage::ShadowAndDefferedPass);
    eng.render.resolution_scale = 0.1;
    while eng.work(){
        if eng.control.get_key_state(44){
        }
        mesh.exec();
        mesh2.exec();
    }
    eng.end();
}
