use std::fs::{self};

use engine::{engine::Engine, image::Image, light::LightType, material::Material, ui::UIplane};

use crate::engine::{loader::modelasset::ModelAsset, math::vec3::Vec3, model::Model, object::Object, scene::Scene, ui::UItext};
mod engine;

fn main() {
    const SPEED: f32 = 0.005f32;
    let mut eng = Engine::new();
    eng.render.set_title("Project Ost89");
    eng.render.shadow_map_resolution = 4000;
    eng.used_light_count = 1;
    eng.lights[0].direction = Vec3::newdefined(1f32, 1f32, -1f32);
    eng.lights[0].light_type = LightType::Directional;

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matgeneral = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);

    let mut brd = Scene::load_from_obj(&mut eng, "assets/BRD.obj", matgeneral);
    brd.camera_number = 0;
    brd.render_all_cameras = false;
    for i in 0..brd.objects.len(){
      brd.objects[i].draw_distance = 500f32;
    }

    let golfmdf = ModelAsset::load_obj("assets/golf.obj");
    let golfmdl = Model::new(&mut eng, golfmdf.vertices[0].clone());
    let golftex = Image::new_from_files(&eng, golfmdf.mtl.matinfo[0].clone());

    let mut golf = Object::new(&mut eng, golfmdl, matgeneral, golftex, engine::render::render::MeshUsage::ShadowAndDefferedPass, false);
    golf.physic_object.pos.y = 4f32;
    golf.draw_distance = 500f32;
    golf.physic_object.air_friction = 0.975;

    let mut viewport = UIplane::new(&mut eng, mat, image);
    viewport.object.physic_object.pos.z = 1.0;

    let ti = Image::new_from_files(&eng, ["assets/text.tiff".to_string()].to_vec());
    let mut fpscnt = UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_");
    fpscnt.pos.z = 0.9;

    eng.cameras[0].physic_object.pos.y = 5.0f32;
    eng.cameras[0].physic_object.pos.z = 3.5f32;
    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;

    //eng.control.mouse_lock = true;

    //let mut relpos = Vec2::new();

    //let mut savpos = Vec2::new();

    //let mut relposx = 0.0;

    let mut tm: i32 = 0;

    golf.physic_object.step_height = 0.2f32;
    golf.physic_object.mass = 0.015f32;

    while eng.work(){
      eng.cameras[0].physic_object.pos.z = golf.physic_object.pos.z + 27.5f32;
      eng.cameras[0].physic_object.pos.x = golf.physic_object.pos.x - 27.5f32;
      eng.cameras[0].physic_object.pos.y = 40f32;
      eng.cameras[0].physic_object.rot.x = 0.7854f32;
      eng.cameras[0].physic_object.rot.y = 0.7854f32;
      eng.cameras[0].is_orthographic = false;
      eng.cameras[0].fov = 20f32;
      eng.cameras[0].zfar = 200f32;

      eng.lights[0].camera.zfar = 200f32;
      eng.lights[0].camera.fov = 30f32;
      eng.lights[0].pos.x = golf.physic_object.pos.x + 82.5f32;
      eng.lights[0].pos.y = 120f32;
      eng.lights[0].pos.z = golf.physic_object.pos.z + 82.5f32;
      eng.lights[0].rot.x = eng.cameras[0].physic_object.rot.x;
      eng.lights[0].rot.y = -eng.cameras[0].physic_object.rot.y;
      eng.lights[0].color = Vec3::newdefined(0.005, 0.005, 0.01);

      if tm > 0{
        tm -= eng.times_to_calculate_physics as i32;
      }

      //if !eng.control.mouse_lock {
      //  relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
      //  relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
      //  relposx = 0.0;
      //}else{
      //  eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
      //  eng.cameras[0].physic_object.rot.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - relpos.y;
      //  savpos.x = eng.cameras[0].physic_object.rot.x;
      //  savpos.y = eng.cameras[0].physic_object.rot.y;

      //  if eng.cameras[0].physic_object.rot.x < -1.5 {
      //    relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x + 1.5;
      //    eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
      //  }
      //  if eng.cameras[0].physic_object.rot.x > 1.5 {
      //    relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - 1.5;
      //    eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
      //  }
      //}

      if eng.control.get_key_state(40){
        golf.physic_object.acceleration.z += f32::cos(-golf.physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.acceleration.x += f32::sin(-golf.physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.air_friction = 0.925;
      }
      if eng.control.get_key_state(44){
        golf.physic_object.acceleration.z += f32::cos(-golf.physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.acceleration.x += f32::sin(-golf.physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.air_friction = 0.975;
      }
      if eng.control.get_key_state(25) && (f32::abs(golf.physic_object.speed.x) >= 0.01 || f32::abs(golf.physic_object.speed.z) >= 0.01){
        golf.physic_object.rot.y -= 0.01 * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(22) && (f32::abs(golf.physic_object.speed.x) >= 0.01 || f32::abs(golf.physic_object.speed.z) >= 0.01){
        golf.physic_object.rot.y += 0.01 * eng.times_to_calculate_physics as f32;
      }

      //if eng.control.get_key_state(49) && tm <= 0{
      //  eng.control.mouse_lock = !eng.control.mouse_lock;
      //  tm = 100;
      //}

      if eng.control.mousebtn[2] && tm <= 0{
        //eng.control.mouse_lock = !eng.control.mouse_lock;
        golf.physic_object.pos.y = 4f32;
        golf.physic_object.pos.x = 0.0;
        golf.physic_object.pos.z = 0.0;
        golf.physic_object.rot.y = 0.0;
        tm = 100;
      }

      brd.exec(&mut eng);
      golf.exec(&mut eng);

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
