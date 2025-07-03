use std::fs;

use engine::{engine::Engine, image::Image, light::LightType, material::Material, scene::Scene, ui::{UIplane, UItext}};

use crate::engine::{loader::modelasset::ModelAsset, math::vec3::Vec3, model::Model, object::Object};
mod engine;

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    eng.used_camera_count = 2;
    eng.lights[0].light_type = LightType::Spot;
 
    let mut wkfc = 2.0f32;

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let dfragqo = fs::read("shaders/fdeffqo").unwrap();
    let dfragem = fs::read("shaders/fdeffem").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let plsh = fs::read("shaders/pltx").unwrap();
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let mat2 = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let mat3 = Material::new(&eng, dvert.clone(), dfragqo, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let mat4 = Material::new(&eng, dvert, dfragem, shadow, [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let mat5 = Material::new(&eng, vert, plsh, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, image);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.object.mesh.ubo[16] = wkfc;

    let ti = Image::new_from_files(&eng, ["assets/text.tiff".to_string()].to_vec());
    let mut text = UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_");
    text.signal = false;

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.pos.y = 3f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = false;

    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-10.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...");
    }

    let mut train = Scene::load_from_obj(&mut eng, "assets/train.obj", mat2);
    train.render_all_cameras = false;
    train.camera_number = 0;

    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-30.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...");
    }

    let mut trainqo = Scene::load_from_obj(&mut eng, "assets/train_quest.obj", mat3);
    trainqo.render_all_cameras = false;
    trainqo.camera_number = 0;
    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-50.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...");
    }

    let mut traindr = Scene::load_from_obj(&mut eng, "assets/train_door.obj", mat2);
    traindr.render_all_cameras = false;
    traindr.camera_number = 0;
    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-70.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...");
    }

    let mut vrt1 = ModelAsset::load_obj("assets/train_em.obj");
    let md1 = Model::new(&mut eng, vrt1.vertices[0].clone());

    let mut trainem = Object::new(&mut eng, md1, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);
    trainem.mesh.camera_number = 0;
    trainem.mesh.render_all_cameras = false;
    vrt1 = ModelAsset::load_obj("assets/train_gl.obj");

    let md2 = Model::new(&mut eng, vrt1.vertices[0].clone());

    let mut traingl = Object::new(&mut eng, md2, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);
    traingl.mesh.camera_number = 1;
    traingl.mesh.render_all_cameras = false;
    traingl.draw_distance = 300f32;

    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-90.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...DONE\nSynchronizing chrono-displacement engine...");
    }

    for _ in 0..2{
      eng.work();

      text.pos.y = eng.render.resolution_y as f32/2.0-110.0;
      text.pos.x = 10.0;
      text.pos.z = 0.9;
      text.size.x = 10.0;
      text.size.y = 20.0;

      text.exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...DONE\nSynchronizing chrono-displacement engine...DONE\nFinal systems check...");
    }

    let mut intspr = UIplane::new_from_file(&mut eng, mat5, ["assets/interact.tiff".to_string()].to_vec());

    eng.cameras[0].physic_object.gravity = true;
    eng.cameras[0].physic_object.pos.y = 3f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = true;
    eng.control.mouse_lock = true;
    text.signal = true;

    //let mut sn = Speaker::new(&mut eng, "assets/audio/sample.wav");

    eng.lights[0].color = Vec3::newdefined(1.0, 1.0, 0.9);
    eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 0.0);
    eng.lights[0].rot = Vec3::newdefined(1.5708, 0.0, 0.0);

    while eng.work(){
      eng.cameras[0].physic_object.rot.x = eng.control.ypos as f32/eng.render.resolution_y as f32;
      eng.cameras[0].physic_object.rot.y = eng.control.xpos as f32/eng.render.resolution_x as f32;

      if wkfc >= 0.0{
        wkfc -= (TICKSZ/5.0)*eng.times_to_calculate_physics as f32;
        viewport.object.mesh.ubo[16] = wkfc;
      }else{
        viewport.object.mesh.ubo[16] = 0.0;
      }
      
      viewport.object.mesh.ubo[17] += TICKSZ;

      if eng.cameras[0].physic_object.rot.x < -1.5 {
        eng.cameras[0].physic_object.rot.x = -1.5;
      }
      if eng.cameras[0].physic_object.rot.x > 1.5 {
        eng.cameras[0].physic_object.rot.x = 1.5;
      }
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
      
      eng.cameras[1] = eng.cameras[0];
      //sn.exec(&mut eng);
      traingl.physic_object.solid = false;
      trainem.physic_object.solid = false;
      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.exec(&mut eng);
      train.exec(&mut eng);
      trainqo.exec(&mut eng);
      trainem.exec(&mut eng);
      traingl.exec(&mut eng);
      traindr.exec(&mut eng);

      intspr.object.draw = false;
      for i in 0..trainqo.objects.len(){
        if trainqo.objects[i].is_looking_at{
          intspr.object.draw = true;
        }
      }
      for i in 0..traindr.objects.len(){
        if traindr.objects[i].is_looking_at{
          intspr.object.draw = true;
        }
      }

      intspr.object.physic_object.pos.z = 0.9;
      intspr.object.physic_object.scale.x = 32.0;
      intspr.object.physic_object.scale.y = 32.0;
      intspr.object.physic_object.pos.x = eng.render.resolution_x as f32/2.0 - 16.0;
      intspr.object.physic_object.pos.y = eng.render.resolution_y as f32 * 0.75 - 16.0;
      intspr.exec(&mut eng);

      text.pos.y = eng.render.resolution_y as f32 - text.size.y;
      text.pos.x = 0.0;
      text.pos.z = 0.8;
      text.size.x = 15.0;
      text.size.y = 30.0;

      let fps = eng.fps;
      text.exec(&mut eng, &format!("fps:{}", fps));
    }
    eng.end();
}
