use std::fs::{self};

use engine::{engine::Engine, image::Image, light::LightType, material::Material, ui::UIplane};

use crate::engine::{loader::modelasset::ModelAsset, math::vec3::Vec3, model::Model, object::Object, scene::Scene, ui::UItext};
mod engine;

fn main() {
    const SPEED: f32 = 0.005f32;
    let mut eng = Engine::new();
    eng.render.set_title("Project Ost");
    eng.render.shadow_map_resolution = 4000;
    eng.used_light_count = 1;
    eng.lights[0].light_type = LightType::Spot;

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let dfragu = fs::read("shaders/fdefferedu").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matgeneral = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let matuniq = Material::new(&eng, dvert.clone(), dfragu, shadow.clone(), [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeFrontBit]);
    let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);

    let mut friedrichstrasse = Scene::load_from_obj(&mut eng, "assets/friedrichstrasse.obj", matgeneral);

    let mut friedrichstrasseu = Scene::load_from_obj(&mut eng, "assets/friedrichstrasse_uniq.obj", matuniq);

    for i in 0..friedrichstrasse.objects.len(){
      friedrichstrasse.objects[i].draw_distance = 1000f32;
    }

    for i in 0..friedrichstrasseu.objects.len(){
      friedrichstrasseu.objects[i].draw_distance = 1000f32;
    }

    let ma1 = ModelAsset::load_obj("assets/pawn.obj");
    let m1 = Model::new(&eng, ma1.vertices[0].clone());
    let mut pawnpl = Object::new(&mut eng, m1, matgeneral, image, engine::render::render::MeshUsage::ShadowAndDefferedPass, false);

    let mut viewport = UIplane::new(&mut eng, mat, image);
    viewport.object.physic_object.pos.z = 1.0;

    let ti = Image::new_from_files(&eng, ["assets/text.tiff".to_string()].to_vec());
    let mut fpscnt = UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_");
    fpscnt.pos.z = 0.9;

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;
    eng.cameras[0].physic_object.pos.y = 6.0f32;
    eng.cameras[0].physic_object.v2.y = -4f32;

    pawnpl.draw_distance = 1000f32;

    while eng.work(){
      if eng.control.get_key_state(40){
        pawnpl.physic_object.acceleration.x += SPEED * eng.times_to_calculate_physics as f32;
      }
      else if eng.control.get_key_state(44){
        pawnpl.physic_object.acceleration.x -= SPEED * eng.times_to_calculate_physics as f32;
      }
      else if eng.control.get_key_state(25){
        pawnpl.physic_object.acceleration.z -= SPEED * eng.times_to_calculate_physics as f32;
      }
      else if eng.control.get_key_state(22){
        pawnpl.physic_object.acceleration.z += SPEED * eng.times_to_calculate_physics as f32;
      }
      else if eng.control.mousebtn[2]{
        eng.lights[0].pos = eng.cameras[0].physic_object.pos;
        eng.lights[0].rot = eng.cameras[0].physic_object.rot;
        eng.lights[0].color = Vec3::newdefined(10.0, 10.0, 9.0);
      }

      eng.cameras[0].physic_object.rot.x = 0.2617;
      eng.cameras[0].physic_object.rot.y = -1.8325;

      eng.cameras[0].physic_object.pos.x = pawnpl.physic_object.pos.x + 10f32;
      eng.cameras[0].physic_object.pos.y = pawnpl.physic_object.pos.y + 6f32;
      eng.cameras[0].physic_object.pos.z = pawnpl.physic_object.pos.z - 3f32;

      friedrichstrasse.exec(&mut eng);
      friedrichstrasseu.exec(&mut eng);

      pawnpl.exec(&mut eng);

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.exec(&mut eng);

      fpscnt.pos.x = 0.0;
      fpscnt.pos.y = eng.render.resolution_y as f32 - 32f32;
      fpscnt.size.x = 16f32;
      fpscnt.size.y = 32f32;
      let fps = eng.fps;
      fpscnt.exec(&mut eng, &format!("fps: {}", fps));
    }
    eng.end();
}
