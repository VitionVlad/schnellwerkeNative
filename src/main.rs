#![allow(dead_code)]
use std::fs::{self};

use engine::{engine::Engine, image::Image, light::LightType, material::Material, ui::UIplane};

use crate::engine::{loader::{imageasset::ImageAsset, modelasset::ModelAsset, jsonparser::JsonF}, math::vec3::Vec3, model::Model, object::Object, scene::Scene, ui::UItext};
mod engine;

fn dst(v1: Vec3, v2: Vec3) -> f32{
  return ((v2.x-v1.x).powi(2) + (v2.y-v1.y).powi(2) + (v2.z-v1.z).powi(2)).sqrt();
}

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    let mut wkfc = 2.0f32;
    eng.render.set_title("Project Ost89");
    eng.render.set_new_resolution(1280, 720);
    eng.render.shadow_map_resolution = 1000;
    eng.used_light_count = 1;
    eng.lights[0].direction = Vec3::newdefined(1f32, 1f32, -1f32);
    eng.lights[0].light_type = LightType::Directional;

    let loc = 0;

    for _ in 0..2{
      eng.work();
    }

    let langj = JsonF::load_from_file("assets/lang.json");

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let imgf = fs::read("shaders/img").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let imgmat = Material::new(&eng, vert.clone(), imgf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matgeneral = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let rwicon = ImageAsset::load_tiff("assets/icon.tiff");
    let black = Image::new_color(&eng, [0, 0, 0, i8::MAX]);
    let logo = Image::new_from_files(&eng, vec!["assets/logo.tiff".to_string()]);
    eng.render.set_icon(rwicon.size[0], rwicon.size[1], rwicon.data);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;

    let mut pausebg = UIplane::new(&mut eng, imgmat, black);
    pausebg.object.physic_object.pos.z = 0.9;

    let mut logops = UIplane::new(&mut eng, imgmat, logo);
    logops.object.physic_object.pos.z = 0.8;

    for _ in 0..2{
      eng.work();

      viewport.object.mesh.ubo[48] = wkfc;
      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.exec(&mut eng);
    }

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
    golf.physic_object.pos.y = 0.1f32;
    golf.draw_distance = 500f32;
    golf.physic_object.air_friction = 0.975;

    let tl: Image = Image::new_from_files(&eng, ["assets/textlat.tiff".to_string()].to_vec());
    let tlc: Image = Image::new_from_files(&eng, ["assets/textcyr.tiff".to_string()].to_vec());
    let mut text: [[UItext; 5]; 2] = [[
      UItext::new(&mut eng, matt, tl, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%"),
      UItext::new(&mut eng, matt, tl, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%"),
      UItext::new(&mut eng, matt, tl, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%"),
      UItext::new(&mut eng, matt, tl, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%"),
      UItext::new(&mut eng, matt, tl, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%"),
    ],[
      UItext::new(&mut eng, matt, tlc, "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, tlc, "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, tlc, "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, tlc, "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, tlc, "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_"),
    ]];

    for i in 0..5{
      text[0][i].pos.z = 0.5;
      text[1][i].pos.z = 0.5;
    }

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

    let mut pause = false;
    let mut pausemn = 0;
    let mut lang = 0usize;
    let textscale = 1.0;

    while eng.work(){
      eng.cameras[0].physic_object.pos.z = golf.physic_object.pos.z + 39.375f32;
      eng.cameras[0].physic_object.pos.x = golf.physic_object.pos.x - 39.375f32;
      eng.cameras[0].physic_object.pos.y = 56.5f32;
      eng.cameras[0].physic_object.rot.x = 0.7854f32;
      eng.cameras[0].physic_object.rot.y = 0.7854f32;
      eng.cameras[0].is_orthographic = true;
      eng.cameras[0].fov = 10f32;
      eng.cameras[0].zfar = 100f32;
      eng.cameras[0].znear = 1f32;

      eng.lights[0].camera.fov = 40f32;
      eng.lights[0].camera.zfar = 100f32;
      eng.lights[0].pos.x = golf.physic_object.pos.x + 39.375f32;
      eng.lights[0].pos.y = 50f32;
      eng.lights[0].pos.z = golf.physic_object.pos.z + 39.375f32;
      eng.lights[0].rot.x = eng.cameras[0].physic_object.rot.x;
      eng.lights[0].rot.y = -eng.cameras[0].physic_object.rot.y;

      viewport.object.mesh.ubo[49] = golf.physic_object.pos.x;
      viewport.object.mesh.ubo[50] = golf.physic_object.pos.z;

      match loc {
          0 => {
            //BRD
            eng.used_light_count = 3;
            eng.lights[1].light_type = LightType::Spot;
            eng.lights[1].camera.physic_object.solid = false;
            eng.lights[1].camera.fov = 40f32;
            eng.lights[1].rot.y = -golf.physic_object.rot.y;
            eng.lights[1].color = Vec3::newdefined(1.0, 1.0, 0.9);
            eng.lights[1].pos.x = golf.physic_object.pos.x + golf.physic_object.speed.x + f32::sin(eng.lights[1].rot.y)*1.4275f32 - f32::cos(eng.lights[1].rot.y)*0.75f32;
            eng.lights[1].pos.y = 0.5f32;
            eng.lights[1].pos.z = golf.physic_object.pos.z + golf.physic_object.speed.z - f32::cos(eng.lights[1].rot.y)*1.4275f32 - f32::sin(eng.lights[1].rot.y)*0.75f32;

            eng.lights[2].light_type = LightType::Spot;
            eng.lights[2].camera.physic_object.solid = false;
            eng.lights[2].camera.fov = 40f32;
            eng.lights[2].rot.y = -golf.physic_object.rot.y;
            eng.lights[2].color = Vec3::newdefined(1.0, 1.0, 0.9);
            eng.lights[2].pos.x = golf.physic_object.pos.x + golf.physic_object.speed.x + f32::sin(eng.lights[1].rot.y)*1.4275f32 - f32::cos(eng.lights[1].rot.y)*-0.75f32;
            eng.lights[2].pos.y = 0.5f32;
            eng.lights[2].pos.z = golf.physic_object.pos.z + golf.physic_object.speed.z - f32::cos(eng.lights[1].rot.y)*1.4275f32 - f32::sin(eng.lights[1].rot.y)*-0.75f32;

            if dst(golf.physic_object.pos, Vec3::newdefined(365.5f32, 3f32, -3.9746127)) < 30f32{
              eng.used_light_count = 4;
              eng.lights[3].light_type = LightType::Spot;
              eng.lights[3].camera.physic_object.solid = false;
              eng.lights[3].camera.fov = 130f32;
              eng.lights[3].rot.x = 1.5708;
              eng.lights[3].color = Vec3::newdefined(1.0, 1.0, 0.8);
              eng.lights[3].pos.x = 365.5f32;
              eng.lights[3].pos.y = 3f32;
              eng.lights[3].pos.z = -3.9746127;
            }else if dst(golf.physic_object.pos, Vec3::newdefined(774.67224, 3f32, 3.199431)) < 30f32{
              eng.used_light_count = 4;
              eng.lights[3].light_type = LightType::Spot;
              eng.lights[3].camera.physic_object.solid = false;
              eng.lights[3].camera.fov = 90f32;
              eng.lights[3].rot.x = 1.5708;
              eng.lights[3].color = Vec3::newdefined(1.0, 1.0, 0.8);
              eng.lights[3].pos.x = 774.67224;
              eng.lights[3].pos.y = 3f32;
              eng.lights[3].pos.z = 3.199431;
            }
            eng.lights[0].color = Vec3::newdefined(0.00025, 0.00025, 0.0005);
          },
          _ => {}
      }

      if wkfc > 0.0{
        wkfc -= (TICKSZ/2.5)*eng.times_to_calculate_physics as f32;
        viewport.object.mesh.ubo[48] = wkfc;
      }else{
        viewport.object.mesh.ubo[48] = 0.0;
      }

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

      if eng.control.get_key_state(40) && !pause{
        golf.physic_object.acceleration.z += f32::cos(-golf.physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.acceleration.x += f32::sin(-golf.physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.air_friction = 0.915;
      }
      if eng.control.get_key_state(44) && !pause{
        golf.physic_object.acceleration.z += f32::cos(-golf.physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.acceleration.x += f32::sin(-golf.physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        golf.physic_object.air_friction = 0.98;
      }
      if eng.control.get_key_state(25) && !pause{
        golf.physic_object.rot.y -= 0.05 * golf.physic_object.speed.x.abs().max(golf.physic_object.speed.z.abs()).min(0.1) * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(22) && !pause{
        golf.physic_object.rot.y += 0.05 * golf.physic_object.speed.x.abs().max(golf.physic_object.speed.z.abs()).min(0.1) * eng.times_to_calculate_physics as f32;
      }

      if eng.control.get_key_state(49) && tm <= 0{
        //eng.control.mouse_lock = !eng.control.mouse_lock;
        pause = !pause;
        tm = 100;
        pausemn = 0;
      }

      if eng.control.mousebtn[2]{
        //eng.control.mouse_lock = !eng.control.mouse_lock;
        println!("{}, {}, {}", golf.physic_object.pos.x, golf.physic_object.pos.y, golf.physic_object.pos.z);
        //tm = 100;
      }

      brd.exec(&mut eng);
      golf.exec(&mut eng);

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.exec(&mut eng);

      pausebg.object.draw = false;
      logops.object.draw = false;
      for i in 0..5{
        text[0][i].draw = false;
        text[0][i].exec(&mut eng, " ");
        text[1][i].draw = false;
        text[1][i].exec(&mut eng, " ");
      }
      if pause{
        pausebg.object.physic_object.scale.y = eng.render.resolution_y as f32;
        pausebg.object.physic_object.scale.x = 400.0;
        pausebg.object.physic_object.pos.y = 0.0;
        pausebg.object.physic_object.pos.x = (eng.render.resolution_x as f32)/2.0 - 200.0;
        pausebg.object.draw = true;

        let abci = langj.other_param[lang].other_param[0].numeral_val as usize;

        match pausemn {
            0 => {
              logops.object.physic_object.scale.y = 400.0;
              logops.object.physic_object.scale.x = 400.0;
              logops.object.physic_object.pos.y = eng.render.resolution_y as f32 /2.0 - 300.0;
              logops.object.physic_object.pos.x = (eng.render.resolution_x as f32)/2.0 - 200.0;
              logops.object.draw = true;

              text[abci][0].draw = true;
              text[abci][0].size.x = 20.0*textscale;
              text[abci][0].size.y = 40.0*textscale;
              text[abci][0].signal = true;
              text[abci][0].per_symbol = false;
              let mut lctxt = langj.other_param[lang].other_param[1].strvalar[0].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][0].size.x + text[abci][0].size.y);
              text[abci][0].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][0].pos.y = eng.render.resolution_y as f32 /2.0 + 30.0*textscale;
              if text[abci][0].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pause = false;
              }

              text[abci][1].draw = true;
              text[abci][1].size.x = 20.0*textscale;
              text[abci][1].size.y = 40.0*textscale;
              text[abci][1].signal = true;
              text[abci][1].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[1].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][1].size.x + text[abci][1].size.y);
              text[abci][1].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][1].pos.y = eng.render.resolution_y as f32 /2.0 + 80.0*textscale;
              if text[abci][1].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pause = false;
              }

              text[abci][2].draw = true;
              text[abci][2].size.x = 20.0*textscale;
              text[abci][2].size.y = 40.0*textscale;
              text[abci][2].signal = true;
              text[abci][2].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[2].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][2].size.x + text[abci][2].size.y);
              text[abci][2].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][2].pos.y = eng.render.resolution_y as f32 /2.0 + 130.0*textscale;
              if text[abci][2].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 1;
              }

              text[abci][3].draw = true;
              text[abci][3].size.x = 20.0*textscale;
              text[abci][3].size.y = 40.0*textscale;
              text[abci][3].signal = true;
              text[abci][3].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[3].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][3].size.x + text[abci][3].size.y);
              text[abci][3].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][3].pos.y = eng.render.resolution_y as f32 /2.0 + 180.0*textscale;
              if text[abci][3].exec(&mut eng, &lctxt) && eng.control.mousebtn[2]{
                break;
              }
            }
            1 => {
              text[abci][0].draw = true;
              text[abci][0].size.x = 40.0*textscale;
              text[abci][0].size.y = 80.0*textscale;
              text[abci][0].signal = false;
              text[abci][0].per_symbol = false;
              let mut lctxt = langj.other_param[lang].other_param[1].strvalar[2].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][0].size.x + text[abci][0].size.y);
              text[abci][0].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][0].pos.y = eng.render.resolution_y as f32 /2.0 - 120.0*textscale;
              text[abci][0].exec(&mut eng, &lctxt);

              text[abci][1].draw = true;
              text[abci][1].size.x = 20.0*textscale;
              text[abci][1].size.y = 40.0*textscale;
              text[abci][1].signal = true;
              text[abci][1].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[4].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][1].size.x + text[abci][1].size.y);
              text[abci][1].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][1].pos.y = eng.render.resolution_y as f32 /2.0 - 30.0*textscale;
              if text[abci][1].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 2;
              }

              text[abci][2].draw = true;
              text[abci][2].size.x = 20.0*textscale;
              text[abci][2].size.y = 40.0*textscale;
              text[abci][2].signal = true;
              text[abci][2].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[5].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][2].size.x + text[abci][2].size.y);
              text[abci][2].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][2].pos.y = eng.render.resolution_y as f32 /2.0 + 20.0*textscale;
              if text[abci][2].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 3;
              }

              text[abci][3].draw = true;
              text[abci][3].size.x = 20.0*textscale;
              text[abci][3].size.y = 40.0*textscale;
              text[abci][3].signal = true;
              text[abci][3].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[15].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][3].size.x + text[abci][3].size.y);
              text[abci][3].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][3].pos.y = eng.render.resolution_y as f32 /2.0 + 70.0*textscale;
              if text[abci][3].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                lang+=1;
                if lang >= langj.other_param.len(){
                  lang = 0;
                }
                //pausemn = 0;
              }

              text[abci][4].draw = true;
              text[abci][4].size.x = 20.0*textscale;
              text[abci][4].size.y = 40.0*textscale;
              text[abci][4].signal = true;
              text[abci][4].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[6].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][4].size.x + text[abci][4].size.y);
              text[abci][4].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][4].pos.y = eng.render.resolution_y as f32 /2.0 + 120.0*textscale;
              if text[abci][4].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 0;
              }
            }
            2 => {
              text[abci][0].draw = true;
              text[abci][0].size.x = 40.0*textscale;
              text[abci][0].size.y = 80.0*textscale;
              text[abci][0].signal = false;
              text[abci][0].per_symbol = false;
              let mut lctxt = langj.other_param[lang].other_param[1].strvalar[4].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][0].size.x + text[abci][0].size.y);
              text[abci][0].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][0].pos.y = eng.render.resolution_y as f32 /2.0 - 120.0*textscale;
              text[abci][0].exec(&mut eng, &lctxt);

              text[abci][1].draw = true;
              text[abci][1].size.x = 20.0*textscale;
              text[abci][1].size.y = 40.0*textscale;
              text[abci][1].signal = true;
              text[abci][1].per_symbol = false;
              lctxt = format!("{}{}", langj.other_param[lang].other_param[1].strvalar[7].clone(), (eng.audio.vol*100.0) as u32);
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][1].size.x + text[abci][1].size.y);
              text[abci][1].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][1].pos.y = eng.render.resolution_y as f32 /2.0 - 30.0*textscale;
              if text[abci][1].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                eng.audio.vol = ((eng.audio.vol*100.0) as i32 - 10) as f32 / 100.0;
                if eng.audio.vol < 0.0 {
                  eng.audio.vol = 1.0;
                }
              }

              text[abci][2].draw = true;
              text[abci][2].size.x = 20.0*textscale;
              text[abci][2].size.y = 40.0*textscale;
              text[abci][2].signal = true;
              text[abci][2].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[6].clone();
              text[abci][2].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][2].size.x + text[abci][2].size.y);
              text[abci][2].pos.y = eng.render.resolution_y as f32 /2.0 + 20.0*textscale;
              if text[abci][2].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 1;
              }
            }
            3 => {
              text[abci][0].draw = true;
              text[abci][0].size.x = 40.0*textscale;
              text[abci][0].size.y = 80.0*textscale;
              text[abci][0].signal = false;
              text[abci][0].per_symbol = false;
              let mut lctxt = langj.other_param[lang].other_param[1].strvalar[5].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][0].size.x + text[abci][0].size.y);
              text[abci][0].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][0].size.x;
              text[abci][0].pos.y = eng.render.resolution_y as f32 /2.0 - 120.0*textscale;
              text[abci][0].exec(&mut eng, &lctxt);

              text[abci][1].draw = true;
              text[abci][1].size.x = 20.0*textscale;
              text[abci][1].size.y = 40.0*textscale;
              text[abci][1].signal = true;
              text[abci][1].per_symbol = false;
              lctxt = format!("{}{}", langj.other_param[lang].other_param[1].strvalar[8].clone(), (eng.render.resolution_scale*100.0) as u32);
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][1].size.x + text[abci][1].size.y);
              text[abci][1].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][1].pos.y = eng.render.resolution_y as f32 /2.0 - 30.0*textscale;
              if text[abci][1].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                eng.render.resolution_scale = ((eng.render.resolution_scale*100.0) as i32 - 10) as f32 / 100.0;
                if eng.render.resolution_scale < 0.1 {
                  eng.render.resolution_scale = 1.0;
                }
              }

              text[abci][2].draw = true;
              text[abci][2].size.x = 20.0*textscale;
              text[abci][2].size.y = 40.0*textscale;
              text[abci][2].signal = true;
              text[abci][2].per_symbol = false;
              lctxt = format!("{}{}", langj.other_param[lang].other_param[1].strvalar[9].clone(), match eng.render.shadow_map_resolution {
                1000 => langj.other_param[lang].other_param[1].strvalar[10].clone(),
                2000 => langj.other_param[lang].other_param[1].strvalar[11].clone(),
                4000 => langj.other_param[lang].other_param[1].strvalar[12].clone(),
                8000 => langj.other_param[lang].other_param[1].strvalar[13].clone(),
                  _ => "".to_string()
              });
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][2].size.x + text[abci][2].size.y);
              text[abci][2].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][2].pos.y = eng.render.resolution_y as f32 /2.0 + 20.0*textscale;
              if text[abci][2].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                eng.render.shadow_map_resolution *= 2;
                if eng.render.shadow_map_resolution > 8000{
                  eng.render.shadow_map_resolution = 1000;
                }
                tm = 100;
              }

              text[abci][3].draw = true;
              text[abci][3].size.x = 20.0*textscale;
              text[abci][3].size.y = 40.0*textscale;
              text[abci][3].signal = true;
              text[abci][3].per_symbol = false;
              lctxt = format!("{}{}", langj.other_param[lang].other_param[1].strvalar[14].clone(), (eng.render.fullscreen) as u32);
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][3].size.x + text[abci][3].size.y);
              text[abci][3].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][3].pos.y = eng.render.resolution_y as f32 /2.0 + 70.0*textscale;
              if text[abci][3].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                eng.render.fullscreen = !eng.render.fullscreen;
              }

              text[abci][4].draw = true;
              text[abci][4].size.x = 20.0*textscale;
              text[abci][4].size.y = 40.0*textscale;
              text[abci][4].signal = true;
              text[abci][4].per_symbol = false;
              lctxt = langj.other_param[lang].other_param[1].strvalar[6].clone();
              pausebg.object.physic_object.scale.x = pausebg.object.physic_object.scale.x.max(lctxt.len() as f32 * text[abci][4].size.x + text[abci][4].size.y);
              text[abci][4].pos.x = (eng.render.resolution_x as f32 / 2.0) - (lctxt.len() as f32 / 2.0) * text[abci][1].size.x;
              text[abci][4].pos.y = eng.render.resolution_y as f32 /2.0 + 120.0*textscale;
              if text[abci][4].exec(&mut eng, &lctxt) && eng.control.mousebtn[2] && tm <= 0{
                tm = 100;
                pausemn = 1;
              }
            }
            _ => {}
        }
        pausebg.object.physic_object.pos.x = (eng.render.resolution_x as f32)/2.0 - pausebg.object.physic_object.scale.x/2.0;
      }
      pausebg.exec(&mut eng);
      logops.exec(&mut eng);

      //fpscnt.pos.x = 0.0;
      //fpscnt.pos.y = eng.render.resolution_y as f32 - 32f32;
      //fpscnt.size.x = 16f32;
      //fpscnt.size.y = 32f32;
      //let fps = eng.fps;
      //fpscnt.exec(&mut eng, &format!("fps: {}", fps));
    }
    eng.end();
}
