use std::fs;

use engine::{engine::Engine, image::Image, light::LightType, loader::modelasset::ModelAsset, material::Material, model::Model, object::Object, plane::PLANE};
mod engine;

fn main() {
    const SPEED: f32 = 0.001f32;
    let mut eng = Engine::new();
    eng.lights[0].light_type = LightType::Spot;

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();

    let mat = Material::new(eng, vert, frag, vec![], engine::render::render::CullMode::CullModeNone);
    let mat2 = Material::new(eng, dvert, dfrag, shadow, engine::render::render::CullMode::CullModeNone);

    let image = Image::new_color(eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);
    let img2 = Image::new_from_files(eng, vec!["assets/texture2.tiff", "assets/texture.tiff"]);

    let obj = ModelAsset::load_obj("assets/model.obj");

    let model = Model::new(eng, PLANE.to_vec());
    let ldobj1 = Model::new(eng, obj.vertices[0].clone());
    let ldobj2 = Model::new(eng, obj.vertices[1].clone());
    let ldobj3 = Model::new(eng, obj.vertices[2].clone());

    let mut mesh = Object::new(eng, model, mat, image, engine::render::render::MeshUsage::LightingPass, true);
    let mut mesh2 = Object::new(eng, ldobj1, mat2, img2, engine::render::render::MeshUsage::ShadowAndDefferedPass, true);
    let mut mesh3 = Object::new(eng, ldobj2, mat2, img2, engine::render::render::MeshUsage::ShadowAndDefferedPass, true);
    let mut mesh4 = Object::new(eng, ldobj3, mat2, img2, engine::render::render::MeshUsage::ShadowAndDefferedPass, true);

    eng.cameras[0].physic_object.gravity = true;
    eng.cameras[0].physic_object.pos.x = -2.5f32;
    eng.cameras[0].physic_object.pos.y = 25f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = true;
    eng.control.mouse_lock = true;
    while eng.work(vec![
      &mut mesh, 
      &mut mesh2,
      &mut mesh3,
      &mut mesh4,
    ]){
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
      if eng.control.get_key_state(0){
        eng.control.mouse_lock = true;
      }
      if eng.control.mousebtn[0]{
        eng.lights[0].pos = eng.cameras[0].physic_object.pos;
        eng.lights[0].rot = eng.cameras[0].physic_object.rot;
      }
    }
    eng.end();
}
