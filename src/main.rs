use std::fs;

use engine::{engine::Engine, image::Image, loader::imageasset::ImageAsset, material::Material, model::Model, object::Object, plane::PLANE};
mod engine;

fn main() {
    const SPEED: f32 = 0.001f32;
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

    let mut tiff = ImageAsset::load_tiff("assets/texture.tiff");
    let mut tiff2 = ImageAsset::load_tiff("assets/texture2.tiff");
    tiff.data.append(&mut tiff2.data);

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let mat = Material::new(eng, vert, frag, vec![], engine::render::render::CullMode::CullModeNone);
    let mat2 = Material::new(eng, dvert, dfrag, shadow, engine::render::render::CullMode::CullModeNone);
    let image = Image::new(eng, [2, 2, 2], img);
    let img2 = Image::new(eng, [tiff.size[0], tiff.size[1], 2], tiff.data);

    let model = Model::new(eng, PLANE.to_vec());
    let mut mesh = Object::new(eng, model, mat, image, engine::render::render::MeshUsage::LightingPass, true);
    let mut mesh2 = Object::new(eng, model, mat2, img2, engine::render::render::MeshUsage::ShadowAndDefferedPass, true);

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.is_static = false;
    eng.control.mouse_lock = true;
    while eng.work(){
        eng.cameras[0].physic_object.rot.x = eng.control.ypos as f32/eng.render.resolution_y as f32;
        eng.cameras[0].physic_object.rot.y = eng.control.xpos as f32/eng.render.resolution_x as f32;
        if eng.control.get_key_state(40){
          eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
          eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        }
        if eng.control.get_key_state(44){
          eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
          eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        }
        if eng.control.get_key_state(25){
          eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
          eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        }
        if eng.control.get_key_state(22){
          eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
          eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        }
        if eng.control.get_key_state(49){
          eng.control.mouse_lock = false;
        }
        mesh.exec(&mut eng);
        mesh2.exec(&mut eng);
    }
    eng.end();
}
