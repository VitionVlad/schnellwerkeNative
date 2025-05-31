use std::fs;

use engine::{engine::Engine, image::Image, light::LightType, material::Material, scene::Scene, ui::{UIplane, UItext}};
mod engine;

fn main() {
    const SPEED: f32 = 0.0025f32;
    let mut eng = Engine::new();
    eng.lights[0].light_type = LightType::Spot;

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();

    let mat = Material::new(&eng, vert.clone(), frag, vec![], engine::render::render::CullMode::CullModeNone);
    let matt = Material::new(&eng, vert, textf, vec![], engine::render::render::CullMode::CullModeNone);
    let mat2 = Material::new(&eng, dvert, dfrag, shadow, engine::render::render::CullMode::CullModeNone);

    let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);
    let mut viewport = UIplane::new(&mut eng, mat, image);

    let font = Image::new_from_files(&eng, vec!["assets/text.tiff".to_string()]);
    let mut text = UItext::new(&mut eng, matt, font, "abcdefghijklmnopqrstuvwxyz0123456789,.;");

    let mut scn = Scene::load_from_obj(&mut eng, "assets/model.obj", mat2);

    eng.cameras[0].physic_object.gravity = true;
    eng.cameras[0].physic_object.pos.y = 25f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = true;
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
      if eng.control.get_key_state(0){
        eng.control.mouse_lock = true;
      }
      if eng.control.mousebtn[0]{
        eng.lights[0].pos = eng.cameras[0].physic_object.pos;
        eng.lights[0].rot = eng.cameras[0].physic_object.rot;
      }

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.exec(&mut eng);
      scn.exec(&mut eng);
      text.pos.y = eng.render.resolution_y as f32 - text.size.y;
      text.pos.x = -(eng.render.resolution_x as f32) + text.size.x;
      text.pos.z = -0.9;
      text.exec(&mut eng, "hello world");
    }
    eng.end();
}
