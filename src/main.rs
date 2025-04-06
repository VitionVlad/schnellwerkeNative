use std::fs;

use engine::{engine::Engine, material::Material};
mod engine;

fn main() {
    let mut eng = Engine::new();
    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let mut _mat = Material::new(eng, vert, frag, engine::render::render::CullMode::CullModeNone);
    while eng.work(){
    }
    eng.end();
}
