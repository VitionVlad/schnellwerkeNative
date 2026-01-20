#![allow(dead_code)]
use std::fs::{self};

use engine::{engine::Engine, image::Image, material::Material, ui::UIplane};

use crate::engine::{loader::{gltf::Gltf, jsonparser::JsonF}, ui::UItext};
mod engine;

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    //let mut wkfc = 2.0f32;
    eng.render.set_title("ARSD");
    eng.render.set_new_resolution(1280, 720);

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    //let dvert = fs::read("shaders/vdeffered").unwrap();
    //let dfrag = fs::read("shaders/fdeffered").unwrap();
    //let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();

    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let black = Image::new_color(&eng, [0, 0, 0, i8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;

    let mut fpscnt = UItext::new_from_file(&mut eng, matt, "assets/textlat.png", "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%");

    let jgltf = JsonF::load_from_file("assets/BRD2.gltf");
    let pgltf = Gltf::parse_gltf(jgltf);

    println!("selected scene number: {}", pgltf.scene);
    println!("scenes number: {}", pgltf.scenes.len());
    println!("nodes number: {}", pgltf.scenes[0].nodes.len());
    println!("object number: {}", pgltf.objects.len());
    println!("mesh number: {}", pgltf.meshes.len());
    println!("texture number: {}", pgltf.textures.len());

    for i in 0..pgltf.objects.len(){
      println!("{}, {}, {}", pgltf.meshes[i].attributes[0], pgltf.meshes[i].attributes[1], pgltf.meshes[i].attributes[2]);
    }

    while eng.work(){
      viewport.exec(&mut eng);

      fpscnt.pos.x = 0.0;
      fpscnt.pos.y = eng.render.resolution_y as f32 - 20f32;
      fpscnt.size.x = 10f32;
      fpscnt.size.y = 20f32;
      let fps = eng.fps;
      fpscnt.exec(&mut eng, &format!("fps: {}", fps));
    }
    eng.end();
}