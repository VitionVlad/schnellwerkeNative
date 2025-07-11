#![windows_subsystem = "windows"]

use std::{fs::{self, File}, io::{Seek, Write}, path::Path};

use engine::{engine::Engine, image::Image, light::LightType, material::Material, scene::Scene, ui::{UIplane, UItext}};

use crate::engine::{loader::{imageasset::ImageAsset, modelasset::ModelAsset}, math::{vec2::Vec2, vec3::Vec3}, model::Model, object::Object, speaker::Speaker};
mod engine;

/*
q1s: 28
q2s: 1916
q3s: END
q4s: PARIS
*/

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    eng.used_camera_count = 2;
    eng.lights[0].light_type = LightType::Spot;

    let mut ldpsz = -8.0;

    let mut file: File = match Path::new("sav").exists() {
        true => {
          let cfg = fs::read("sav").unwrap();
          let mut i = 0;
          while i < cfg.len(){
            if cfg[i] == b'1'{
              i+=1;
              eng.render.resolution_x = u32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              i+=4;
              eng.render.resolution_y = u32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              eng.render.set_new_resolution(eng.render.resolution_x, eng.render.resolution_y);
              i+=3;
            }
            if cfg[i] == b'2'{
              i+=1;
              if cfg[i] == b'1'{
                eng.render.fullscreen = true;
              }else{
                eng.render.fullscreen = false;
              }
            }
            if cfg[i] == b'3'{
              i+=1;
              eng.render.resolution_scale = f32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              i+=3;
            }
            if cfg[i] == b'4'{
              i+=1;
              eng.render.shadow_map_resolution = u32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              i+=3;
            }
            if cfg[i] == b'5'{
              i+=1;
              eng.audio.vol = f32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              i+=3;
            }
            if cfg[i] == b'6'{
              i+=1;
              ldpsz = f32::from_be_bytes([cfg[i as usize], cfg[i as usize + 1], cfg[i as usize + 2], cfg[i as usize + 3]]);
              i+=3;
            }
            i+=1;
          }
          let _ = fs::remove_file("sav");
          File::create_new("sav").unwrap()
        },
        false => File::create_new("sav").unwrap(),
    };

    pub fn saveset(eng: &mut Engine, file: &mut File){
      let _ = file.seek(std::io::SeekFrom::Start(0));
      let mut stt = vec![];
      stt.push(b'1');
      let xb = eng.render.resolution_x.to_be_bytes();
      let yb = eng.render.resolution_y.to_be_bytes();
      for i in 0..4{
        stt.push(xb[i]);
      }
      for i in 0..4{
        stt.push(yb[i]);
      }
      stt.push(b'2');
      if eng.render.fullscreen{
        stt.push(b'1');
      }else{
        stt.push(b'0');
      }
      stt.push(b'3');
      let rscb = eng.render.resolution_scale.to_be_bytes();
      for i in 0..4{
        stt.push(rscb[i]);
      }
      stt.push(b'4');
      let rscb = eng.render.shadow_map_resolution.to_be_bytes();
      for i in 0..4{
        stt.push(rscb[i]);
      }
      stt.push(b'5');
      let rscb = eng.audio.vol.to_be_bytes();
      for i in 0..4{
        stt.push(rscb[i]);
      }
      let _ = file.write_all(&stt);
    }

    pub fn savepos(eng: &mut Engine, file: &mut File){
      let _ = file.seek(std::io::SeekFrom::Start(26));
      let mut stt = vec![];
      stt.push(b'6');
      let rscb = eng.cameras[0].physic_object.pos.z.to_be_bytes();
      for i in 0..4{
        stt.push(rscb[i]);
      }
      let _ = file.write_all(&stt);
    }

    saveset(&mut eng, &mut file);

    let mut oldr = [eng.render.resolution_x, eng.render.resolution_y];

    let icn = ImageAsset::load_tiff("assets/icon.tiff");
    eng.render.set_icon(icn.size[0], icn.size[1], icn.data);
    eng.render.set_title("ZUG");
 
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
    let mut text: [UItext; 5] = [
      UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
      UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
    ];

    text[0].signal = false;
    text[1].signal = false;

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.pos.y = 3f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = false;

    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-10.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...");
    }

    let mut train = Scene::load_from_obj(&mut eng, "assets/train.obj", mat2);
    train.render_all_cameras = false;
    train.camera_number = 0;

    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-30.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...");
    }

    let mut trainqo = Scene::load_from_obj(&mut eng, "assets/train_quest.obj", mat3);
    trainqo.render_all_cameras = false;
    trainqo.camera_number = 0;
    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-50.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...");
    }

    let mut traindr = Scene::load_from_obj(&mut eng, "assets/train_door.obj", mat2);
    traindr.render_all_cameras = false;
    traindr.camera_number = 0;
    //qd 0, 2, 4, 6
    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-70.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...");
    }

    let mut vrt1 = ModelAsset::load_obj("assets/train_em.obj");
    let md1 = Model::new(&mut eng, vrt1.vertices[0].clone());

    let mut trainem = Object::new(&mut eng, md1, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);
    trainem.mesh.camera_number = 0;
    trainem.mesh.render_all_cameras = false;
    trainem.draw_distance = 300f32;
    vrt1 = ModelAsset::load_obj("assets/train_gl.obj");

    let md2 = Model::new(&mut eng, vrt1.vertices[0].clone());

    let mut traingl = Object::new(&mut eng, md2, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);
    traingl.mesh.camera_number = 1;
    traingl.mesh.render_all_cameras = false;
    traingl.draw_distance = 300f32;

    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-90.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...DONE\nSynchronizing chrono-displacement engine...");
    }

    for _ in 0..2{
      eng.work();

      text[0].pos.y = eng.render.resolution_y as f32/2.0-110.0;
      text[0].pos.x = 10.0;
      text[0].pos.z = 0.9;
      text[0].size.x = 10.0;
      text[0].size.y = 20.0;

      text[0].exec(&mut eng, "Initializing power systems...DONE\nLoading vital packages...DONE\nEstablishing communication lines...DONE\nLoading armaments and supplies...DONE\nSynchronizing chrono-displacement engine...DONE\nFinal systems check...");
    }

    let mut intspr = UIplane::new_from_file(&mut eng, mat5, ["assets/interact.tiff".to_string()].to_vec());

    eng.cameras[0].physic_object.gravity = true;
    eng.cameras[0].physic_object.pos.y = 3f32;
    eng.cameras[0].physic_object.mass = 0.005f32;
    eng.cameras[0].physic_object.solid = true;
    eng.control.mouse_lock = true;
    text[0].signal = true;
    text[0].per_symbol = true;

    let mut trains = Speaker::new(&mut eng, "assets/audio/train.mp3");
    trains.use_pan = false;
    trains.play = true;
    trains.pos_dependency = false;
    trains.volume = 0.25;

    let mut wk = Speaker::new(&mut eng, "assets/audio/walking.mp3");
    wk.use_pan = false;
    wk.play = false;
    wk.pos_dependency = false;
    wk.volume = 0.35;

    let mut mwk = Speaker::new(&mut eng, "assets/audio/metsteps.mp3");
    mwk.use_pan = false;
    mwk.play = false;
    mwk.pos_dependency = false;
    mwk.volume = 0.35;
    
    let mut gr = Speaker::new(&mut eng, "assets/audio/gear.mp3");
    gr.use_pan = false;
    gr.play = false;
    gr.pos_dependency = false;
    gr.volume = 1.0;

    let mut mars = Speaker::new(&mut eng, "assets/audio/marseillaise.mp3");
    mars.use_pan = true;
    mars.play = true;
    mars.pos_dependency = true;
    mars.pos = Vec3::newdefined(1.54, 1.3, 47.42);
    mars.power = 10f32;
    mars.volume = 1.0;

    eng.lights[0].color = Vec3::newdefined(1.0, 1.0, 0.9);
    eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 0.0);
    eng.lights[0].rot = Vec3::newdefined(1.5708, 0.0, 0.0);

    let mut inspecting: bool;

    let mut qa = -1;

    let mut enpsc: [char; 5] = ['-'; 5];

    let mut tm: i32 = 0;

    let mut relpos = Vec2::new();

    let mut savpos = Vec2::new();

    let mut relposx = 0.0;

    let mut menusel = 0;

    let mut resmod = 0;

    let mut locsv = 0;

    for i in 0..traindr.objects.len(){
      if ldpsz >= traindr.objects[i].physic_object.v2.z{
        traindr.objects[i].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
        traindr.objects[i].draw = false;
      }
    }

    eng.cameras[0].physic_object.pos.z = ldpsz;

    while eng.work(){
      if !eng.control.mouse_lock {
        relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
        relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
        relposx = 0.0;
      }

      eng.lights[0].color = Vec3::newdefined(1.0, 1.0, 0.9);
      eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 0.0);
      eng.lights[0].rot = Vec3::newdefined(1.5708, 0.0, 0.0);

      intspr.object.draw = false;
      inspecting = false;

      if eng.cameras[0].physic_object.pos.z > 11.7{
        eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 23.3);
        if locsv < 1{
          locsv+=1;
          savepos(&mut eng, &mut file);
        }
      }
      if eng.cameras[0].physic_object.pos.z > 35.1{
        eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 46.606);
        if locsv < 2{
          locsv+=1;
          savepos(&mut eng, &mut file);
        }
      }
      if eng.cameras[0].physic_object.pos.z > 58.5{
        eng.lights[0].pos = Vec3::newdefined(0.0, 4.25, 69.897);
        if locsv < 3{
          locsv+=1;
          savepos(&mut eng, &mut file);
        }
      }

      if (eng.cameras[0].physic_object.pos.z > 9.7 && eng.cameras[0].physic_object.pos.z < 13.7) || (eng.cameras[0].physic_object.pos.z > 33.1 && eng.cameras[0].physic_object.pos.z < 37.1) || (eng.cameras[0].physic_object.pos.z > 56.5 && eng.cameras[0].physic_object.pos.z < 60.5){
        trains.volume = 0.5;
      }else{
        trains.volume = 0.25;
      }

      viewport.object.mesh.ubo[17] += TICKSZ;

      if tm > 0{
        tm -= eng.times_to_calculate_physics as i32;
      }else{
        gr.play = false;
      }
      
      wk.play = false;
      mwk.play = false;

      for i in 0..5{
        text[i].draw = false;
        text[i].signal = false;
        text[i].exec(&mut eng, " ");
      }

      if eng.control.mouse_lock{
        eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
        eng.cameras[0].physic_object.rot.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - relpos.y;
        savpos.x = eng.cameras[0].physic_object.rot.x;
        savpos.y = eng.cameras[0].physic_object.rot.y;

        if eng.cameras[0].physic_object.rot.x < -1.5 {
          relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x + 1.5;
          eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
        }
        if eng.cameras[0].physic_object.rot.x > 1.5 {
          relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - 1.5;
          eng.cameras[0].physic_object.rot.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
        }

        if wkfc <= 0.0{
          if eng.control.get_key_state(40){
            eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
            eng.cameras[0].physic_object.acceleration.x += f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
            if trains.volume == 0.5{
              mwk.play = true;
            }else{
              wk.play = true;
            }
          }
          if eng.control.get_key_state(44){
            eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
            eng.cameras[0].physic_object.acceleration.x += f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
            if trains.volume == 0.5{
              mwk.play = true;
            }else{
              wk.play = true;
            }
          }
          if eng.control.get_key_state(25){
            eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
            eng.cameras[0].physic_object.acceleration.z += f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
            if trains.volume == 0.5{
              mwk.play = true;
            }else{
              wk.play = true;
            }
          }
          if eng.control.get_key_state(22){
            eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
            eng.cameras[0].physic_object.acceleration.z += f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
            if trains.volume == 0.5{
              mwk.play = true;
            }else{
              wk.play = true;
            }
          }
        }

        for i in 0..trainqo.objects.len(){
          if trainqo.objects[i].is_looking_at{
            intspr.object.draw = true;
          }
          if trainqo.objects[i].is_looking_at && eng.control.mousebtn[2]{
            intspr.object.draw = false;
            inspecting = true;
          }
        }
        for i in 0..traindr.objects.len(){
          if traindr.objects[i].physic_object.pos.x != 0.0{
            traindr.objects[i].physic_object.solid = false;
            traindr.objects[i].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
            if traindr.objects[i].physic_object.pos.x < -1.725{
              traindr.objects[i].physic_object.pos.x = -1.725;
              traindr.objects[i].draw = false;
              traindr.objects[i].draw_shadow = false;
              traindr.objects[i].physic_object.solid = false;
            }
          }
          if traindr.objects[i].is_looking_at && traindr.objects[i].physic_object.solid && qa == -1{
            intspr.object.draw = true;
            if eng.control.mousebtn[2] && i != 0 && i != 2 && i != 4 && i != 6{
              traindr.objects[i].physic_object.solid = false;
              traindr.objects[i].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
            }
            if eng.control.mousebtn[2] && (i == 0 || i == 2 || i == 4 || i == 6){
              enpsc[0] = '-';
              enpsc[1] = '-';
              enpsc[2] = '-';
              enpsc[3] = '-';
              enpsc[4] = '-';
              qa = i as i32 /2;
              tm = 50;
            }
          }
        }
      }

      if wkfc >= 0.0{
        wkfc -= (TICKSZ/5.0)*eng.times_to_calculate_physics as f32;
        viewport.object.mesh.ubo[16] = wkfc;
        relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32;
        relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - 3.14;
        eng.cameras[0].physic_object.rot.x = 0.0;
        eng.cameras[0].physic_object.rot.y = 3.14;
        eng.cameras[0].physic_object.pos.x = 0.0;
        eng.cameras[0].physic_object.pos.z = ldpsz;
        qa = -1;
      }else{
        viewport.object.mesh.ubo[16] = 0.0;
      }

      if eng.control.get_key_state(49) && tm <= 0{
        eng.control.mouse_lock = !eng.control.mouse_lock;
        qa = -1;
        tm = 100;
        menusel = 0;
      }

      if wkfc > 2.5 {
        text[0].draw = true;
        text[1].draw = false;
        text[0].signal = false;
        text[1].signal = false;
        text[1].exec(&mut eng, " ");
        text[0].size.x = 15.0;
        text[0].size.y = 30.0;
        if wkfc > 9.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 1.5;
          text[0].exec(&mut eng, "END");
        }else if wkfc > 8.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0 - text[0].size.y;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.5;
          text[0].exec(&mut eng, "The Great War ended on November 11th,\n   1918, with Germany capitulation");
        }else if wkfc > 7.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.5;
          text[0].exec(&mut eng, "It was called the war to end all wars");
        }else if wkfc > 6.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.0;
          text[0].exec(&mut eng, "It changed the world, but not enough");
        }else if wkfc > 5.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 10.0;
          text[0].exec(&mut eng, "Peace proved fragile");
        }else if wkfc > 4.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 11.0;
          text[0].exec(&mut eng, "Another war was coming");
        }else if wkfc > 3.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.0;
          text[0].exec(&mut eng, "And sooner than we dared to think");
        }else if wkfc > 2.8{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.0;
          text[0].exec(&mut eng, "And sooner than we dared to think.");
        }else if wkfc > 2.6{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.0;
          text[0].exec(&mut eng, "And sooner than we dared to think..");
        }else if wkfc > 2.0{
          text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
          text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 18.0;
          text[0].exec(&mut eng, "And sooner than we dared to think...");
        }
      }

      if qa == 0{
        eng.control.mouse_lock = false;
        text[1].draw = true;
        text[1].pos.y = eng.render.resolution_y as f32 / 4.0;
        text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x*7.0;
        text[1].exec(&mut eng, &format!("Enter code: {}{}", enpsc[0], enpsc[1]));

        text[0].size = text[1].size;
        text[0].draw = true;
        text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
        text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x*5.0;
        text[0].signal = true;
        text[0].per_symbol = true;

        let mut curpos = 0;
        if enpsc[0] != '-'{
          curpos = 1;
        }

        if text[0].exec(&mut eng, "0123456789") && eng.control.mousebtn[2] && tm <= 0{
          enpsc[curpos] = text[0].symbol_pressed as char;
          tm = 100;
          gr.play = true;
        }

        if enpsc[0] == '2' && enpsc[1] == '8' && tm <= 0{
          traindr.objects[0].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
          traindr.objects[0].physic_object.solid = false;
          eng.control.mouse_lock = true;
          qa = -1;
        }

        if (enpsc[0] != '2' || enpsc[1] != '8') && enpsc[1] != '-'{
          enpsc[0] = '-';
          enpsc[1] = '-';
        }
      }

      if qa == 1{
        if tm > 0{
          tm -= eng.times_to_calculate_physics as i32;
        }

        eng.control.mouse_lock = false;
        text[1].draw = true;
        text[1].pos.y = eng.render.resolution_y as f32 / 4.0;
        text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x*8.0;
        text[1].exec(&mut eng, &format!("Enter code: {}{}{}{}", enpsc[0], enpsc[1], enpsc[2], enpsc[3]));

        text[0].size = text[1].size;
        text[0].draw = true;
        text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
        text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x*5.0;
        text[0].signal = true;
        text[0].per_symbol = true;

        let mut curpos = 0;
        for i in 0..4{
          if enpsc[i] == '-'{
            curpos = i;
            break;
          }
        }

        if text[0].exec(&mut eng, "0123456789") && eng.control.mousebtn[2] && tm <= 0{
          enpsc[curpos] = text[0].symbol_pressed as char;
          tm = 100;
          gr.play = true;
        }

        if enpsc[0] == '1' && enpsc[1] == '9' && enpsc[2] == '1' && enpsc[3] == '6' && tm <= 0{
          traindr.objects[2].physic_object.solid = false;
          traindr.objects[2].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
          eng.control.mouse_lock = true;
          qa = -1;
        }

        if (enpsc[0] != '1' || enpsc[1] != '9' || enpsc[2] != '1' || enpsc[3] != '6') && enpsc[3] != '-'{
          enpsc[0] = '-';
          enpsc[1] = '-';
          enpsc[2] = '-';
          enpsc[3] = '-';
        }
      }

      if qa == 2{
        if tm > 0{
          tm -= eng.times_to_calculate_physics as i32;
        }

        eng.control.mouse_lock = false;
        text[1].draw = true;
        text[1].pos.y = eng.render.resolution_y as f32 / 4.0;
        text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x*8.0;
        text[1].exec(&mut eng, &format!("Enter code: {}{}{}", enpsc[0], enpsc[1], enpsc[2]));

        text[0].size = text[1].size;
        text[0].draw = true;
        text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
        text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x*5.0;
        text[0].signal = true;
        text[0].per_symbol = true;

        let mut curpos = 0;
        for i in 0..3{
          if enpsc[i] == '-'{
            curpos = i;
            break;
          }
        }

        if text[0].exec(&mut eng, "QWERTYUIOP\n ASDFGHKL\n ZXCVBNM") && eng.control.mousebtn[2] && tm <= 0{
          enpsc[curpos] = text[0].symbol_pressed as char;
          tm = 100;
          gr.play = true;
        }

        if enpsc[0] == 'E' && enpsc[1] == 'N' && enpsc[2] == 'D' && tm <= 0{
          traindr.objects[4].physic_object.solid = false;
          traindr.objects[4].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
          eng.control.mouse_lock = true;
          qa = -1;
        }

        if (enpsc[0] != 'E' || enpsc[1] != 'N' || enpsc[2] != 'D') && enpsc[2] != '-'{
          enpsc[0] = '-';
          enpsc[1] = '-';
          enpsc[2] = '-';
        }
      }

      if qa == 3{
        if tm > 0{
          tm -= eng.times_to_calculate_physics as i32;
        }

        eng.control.mouse_lock = false;
        text[1].draw = true;
        text[1].pos.y = eng.render.resolution_y as f32 / 4.0;
        text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x*8.0;
        text[1].exec(&mut eng, &format!("Enter code:{}{}{}{}{}", enpsc[0], enpsc[1], enpsc[2], enpsc[3], enpsc[4]));

        text[0].size = text[1].size;
        text[0].draw = true;
        text[0].pos.y = eng.render.resolution_y as f32 / 2.0;
        text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x*5.0;
        text[0].signal = true;
        text[0].per_symbol = true;

        let mut curpos = 0;
        for i in 0..5{
          if enpsc[i] == '-'{
            curpos = i;
            break;
          }
        }

        if text[0].exec(&mut eng, "QWERTYUIOP\n ASDFGHKL\n ZXCVBNM") && eng.control.mousebtn[2] && tm <= 0{
          enpsc[curpos] = text[0].symbol_pressed as char;
          tm = 100;
          gr.play = true;
        }

        if enpsc[0] == 'P' && enpsc[1] == 'A' && enpsc[2] == 'R' && enpsc[3] == 'I' && enpsc[4] == 'S' && tm <= 0{
          wkfc = 10f32;
          traindr.objects[6].physic_object.solid = false;
          traindr.objects[6].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
          eng.control.mouse_lock = true;
          qa = -1;
          for i in 0..traindr.objects.len(){
            traindr.objects[i].draw = true;
            traindr.objects[i].physic_object.solid = true;
            traindr.objects[i].draw_shadow = true;
            traindr.objects[i].physic_object.pos.x = 0.0;
          }

          eng.cameras[0].physic_object.pos.x = 0.0;
          eng.cameras[0].physic_object.pos.z = -8.0;
          locsv = 0;
          ldpsz = -8.0;
          savepos(&mut eng, &mut file);
        }

        if (enpsc[0] != 'P' || enpsc[1] != 'A' || enpsc[2] != 'R' || enpsc[3] != 'I' || enpsc[4] != 'S') && enpsc[4] != '-'{
          enpsc[0] = '-';
          enpsc[1] = '-';
          enpsc[2] = '-';
          enpsc[3] = '-';
          enpsc[4] = '-';
        }
      }

      if inspecting && eng.cameras[0].fov > 15.0{
        eng.cameras[0].fov -= TICKSZ*150.0*eng.times_to_calculate_physics as f32;
        if eng.cameras[0].fov < 15.0{
          eng.cameras[0].fov = 15.0;
        }
      }
 
      if !inspecting && eng.cameras[0].fov < 90.0{
        eng.cameras[0].fov += TICKSZ*150.0*eng.times_to_calculate_physics as f32;
        if eng.cameras[0].fov > 90.0{
          eng.cameras[0].fov = 90.0;
        }
      }

      eng.cameras[1] = eng.cameras[0];
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
      traindr.use_global_values = false;

      intspr.object.physic_object.pos.z = 0.9;
      intspr.object.physic_object.scale.x = 32.0;
      intspr.object.physic_object.scale.y = 32.0;
      intspr.object.physic_object.pos.x = eng.render.resolution_x as f32/2.0 - 16.0;
      intspr.object.physic_object.pos.y = eng.render.resolution_y as f32 * 0.75 - 16.0;
      intspr.exec(&mut eng);

      //text[0].pos.y = eng.render.resolution_y as f32 - text[0].size.y;
      //text[0].pos.x = 0.0;
      //text[0].pos.z = 0.8;
      //text[0].size.x = 15.0;
      //text[0].size.y = 30.0;

      mars.exec(&mut eng);
      trains.exec(&mut eng);
      wk.exec(&mut eng);
      mwk.exec(&mut eng);
      gr.exec(&mut eng);

      if !eng.control.mouse_lock && qa == -1{
        match menusel {
          0 => {
            text[0].size.x = 40.0;
            text[0].size.y = 80.0;
            text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 1.5;
            text[0].pos.y = eng.render.resolution_y as f32 / 3.0;
            text[0].draw = true;
            text[0].signal = false;
            text[0].exec(&mut eng, "ZUG");

            text[1].size.x = 20.0;
            text[1].size.y = 40.0;
            text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x * 4.0;
            text[1].pos.y = eng.render.resolution_y as f32 / 2.0 + text[1].size.y * 0.5;
            text[1].draw = true;
            text[1].signal = true;
            text[1].per_symbol = false;
            if text[1].exec(&mut eng, "Continue") && eng.control.mousebtn[2] && tm <= 0{
              eng.control.mouse_lock = true;
              gr.play = true;
              tm = 100;
            }

            text[2].size.x = 20.0;
            text[2].size.y = 40.0;
            text[2].pos.x = eng.render.resolution_x as f32 / 2.0 - text[2].size.x * 4.0;
            text[2].pos.y = eng.render.resolution_y as f32 / 2.0 + text[2].size.y * 1.5;
            text[2].draw = true;
            text[2].signal = true;
            text[2].per_symbol = false;
            if text[2].exec(&mut eng, "New game") && eng.control.mousebtn[2] && tm <= 0{
              wkfc = 2f32;
              traindr.objects[6].physic_object.solid = false;
              traindr.objects[6].physic_object.pos.x -= TICKSZ*10.0*eng.times_to_calculate_physics as f32;
              eng.control.mouse_lock = true;
              qa = -1;
              for i in 0..traindr.objects.len(){
                traindr.objects[i].draw = true;
                traindr.objects[i].physic_object.solid = true;
                traindr.objects[i].draw_shadow = true;
                traindr.objects[i].physic_object.pos.x = 0.0;
              }
            
              eng.cameras[0].physic_object.pos.x = 0.0;
              eng.cameras[0].physic_object.pos.z = -8.0;
              ldpsz = -8.0;

              savepos(&mut eng, &mut file);

              locsv = 0;

              gr.play = true;
              tm = 100;
            }

            text[3].size.x = 20.0;
            text[3].size.y = 40.0;
            text[3].pos.x = eng.render.resolution_x as f32 / 2.0 - text[3].size.x * 4.0;
            text[3].pos.y = eng.render.resolution_y as f32 / 2.0 + text[3].size.y * 2.5;
            text[3].draw = true;
            text[3].signal = true;
            text[3].per_symbol = false;
            if text[3].exec(&mut eng, "Settings") && eng.control.mousebtn[2] && tm <= 0{
              menusel = 1;
              gr.play = true;
              tm = 100;
            }

            text[4].size.x = 20.0;
            text[4].size.y = 40.0;
            text[4].pos.x = eng.render.resolution_x as f32 / 2.0 - text[4].size.x * 2.0;
            text[4].pos.y = eng.render.resolution_y as f32 / 2.0 + text[4].size.y * 3.5;
            text[4].draw = true;
            text[4].signal = true;
            text[4].per_symbol = false;
            if text[4].exec(&mut eng, "Quit") && eng.control.mousebtn[2]{
              break;
            }
          },
          1 => {
            text[0].size.x = 40.0;
            text[0].size.y = 80.0;
            text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 4.0;
            text[0].pos.y = eng.render.resolution_y as f32 / 3.0;
            text[0].draw = true;
            text[0].signal = false;
            text[0].exec(&mut eng, "Settings");

            text[1].size.x = 20.0;
            text[1].size.y = 40.0;
            text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x * 3.5;
            text[1].pos.y = eng.render.resolution_y as f32 / 2.0 + text[1].size.y * 0.5;
            text[1].draw = true;
            text[1].signal = true;
            text[1].per_symbol = false;
            if text[1].exec(&mut eng, "Display") && eng.control.mousebtn[2] && tm <= 0{
              menusel = 2;
              gr.play = true;
              tm = 100;
            }

            text[2].size.x = 20.0;
            text[2].size.y = 40.0;
            text[2].pos.x = eng.render.resolution_x as f32 / 2.0 - text[2].size.x * 2.5;
            text[2].pos.y = eng.render.resolution_y as f32 / 2.0 + text[2].size.y * 1.5;
            text[2].draw = true;
            text[2].signal = true;
            text[2].per_symbol = false;
            if text[2].exec(&mut eng, "Audio") && eng.control.mousebtn[2] && tm <= 0{
              menusel = 3;
              gr.play = true;
              tm = 100;
            }

            text[3].size.x = 20.0;
            text[3].size.y = 40.0;
            text[3].pos.x = eng.render.resolution_x as f32 / 2.0 - text[3].size.x * 2.0;
            text[3].pos.y = eng.render.resolution_y as f32 / 2.0 + text[3].size.y * 2.5;
            text[3].draw = true;
            text[3].signal = true;
            text[3].per_symbol = false;
            if text[3].exec(&mut eng, "Back") && eng.control.mousebtn[2] && tm <= 0{
              menusel = 0;
              gr.play = true;
              tm = 100;
            }

            text[4].draw = false;
            text[4].signal = false;
            text[4].exec(&mut eng, " ");
          },
          2 => {
            text[0].size.x = 40.0;
            text[0].size.y = 80.0;
            text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 3.5;
            text[0].pos.y = eng.render.resolution_y as f32 / 3.0;
            text[0].draw = true;
            text[0].signal = false;
            text[0].exec(&mut eng, "Display");

            text[1].size.x = 20.0;
            text[1].size.y = 40.0;
            text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x * 8.0;
            text[1].pos.y = eng.render.resolution_y as f32 / 2.0 + text[1].size.y * 0.5;
            text[1].draw = true;
            text[1].signal = true;
            text[1].per_symbol = false;

            let rscale = ((eng.render.resolution_scale * 10.0) as i32) as f32 / 10.0;

            if text[1].exec(&mut eng, &format!("Render scale: {}", rscale.to_string())) && eng.control.mousebtn[2] && tm <= 0{
              eng.render.resolution_scale = ((eng.render.resolution_scale * 10.0) as i32 - 1) as f32 /10.0;
              if eng.render.resolution_scale < 0.2 {
                eng.render.resolution_scale = 1.0;
              }
              saveset(&mut eng, &mut file);
              gr.play = true;
              tm = 100;
            }

            let flscr = eng.render.fullscreen;

            let rsy = eng.render.shadow_map_resolution;

            let str = format!("Shadows quality: {}", match rsy { 500 => "Low", 1000 => "Medium", 2000 => "High", 4000 => "Very high", _ => "" });

            text[2].size.x = 20.0;
            text[2].size.y = 40.0;
            text[2].pos.x = eng.render.resolution_x as f32 / 2.0 - text[2].size.x * str.len() as f32 / 2.0;
            text[2].pos.y = eng.render.resolution_y as f32 / 2.0 + text[2].size.y * 1.5;
            text[2].draw = true;
            text[2].signal = true;
            text[2].per_symbol = false;

            if text[2].exec(&mut eng, &str) && eng.control.mousebtn[2] && tm <= 0{
              match resmod {
                  0 => {
                    eng.render.shadow_map_resolution = 500;
                    resmod += 1;
                  },
                  1 => {
                    eng.render.shadow_map_resolution = 1000;
                    resmod += 1;
                  },
                  2 => {
                    eng.render.shadow_map_resolution = 2000;
                    resmod += 1;
                  },
                  3 => {
                    eng.render.shadow_map_resolution = 4000;
                    resmod = 0;
                  },
                  _ => {},
              }
              saveset(&mut eng, &mut file);
              gr.play = true;
              tm = 100;
            }

            text[3].size.x = 20.0;
            text[3].size.y = 40.0;
            text[3].pos.x = eng.render.resolution_x as f32 / 2.0 - text[3].size.x * 6.0;
            text[3].pos.y = eng.render.resolution_y as f32 / 2.0 + text[3].size.y * 2.5;
            text[3].draw = true;
            text[3].signal = true;
            text[3].per_symbol = false;

            if text[3].exec(&mut eng, &format!("Fullscreen {}", match flscr { true => "+", false => "-"})) && eng.control.mousebtn[2] && tm <= 0{
              eng.render.fullscreen = !eng.render.fullscreen;
              gr.play = true;
              saveset(&mut eng, &mut file);
              tm = 100;
            }

            text[4].size.x = 20.0;
            text[4].size.y = 40.0;
            text[4].pos.x = eng.render.resolution_x as f32 / 2.0 - text[4].size.x * 2.0;
            text[4].pos.y = eng.render.resolution_y as f32 / 2.0 + text[4].size.y * 3.5;
            text[4].draw = true;
            text[4].signal = true;
            text[4].per_symbol = false;
            if text[4].exec(&mut eng, "Back") && eng.control.mousebtn[2]{
              menusel = 1;
              gr.play = true;
              tm = 100;
            }
          },
          3 => {
            text[0].size.x = 40.0;
            text[0].size.y = 80.0;
            text[0].pos.x = eng.render.resolution_x as f32 / 2.0 - text[0].size.x * 2.5;
            text[0].pos.y = eng.render.resolution_y as f32 / 3.0;
            text[0].draw = true;
            text[0].signal = false;
            text[0].exec(&mut eng, "Audio");

            let mut rscale = (eng.audio.vol * 100f32) as i32;

            let str = format!("Volume: {}", rscale);
            
            text[1].size.x = 20.0;
            text[1].size.y = 40.0;
            text[1].pos.x = eng.render.resolution_x as f32 / 2.0 - text[1].size.x * (str.len()/2) as f32;
            text[1].pos.y = eng.render.resolution_y as f32 / 2.0 + text[1].size.y * 0.5;
            text[1].draw = true;
            text[1].signal = true;
            text[1].per_symbol = false;

            if text[1].exec(&mut eng, &str) && eng.control.mousebtn[2] && tm <= 0{
              rscale -= 10;
              if rscale < 0{
                rscale = 100;
              }
              eng.audio.vol = rscale as f32/100f32;
              saveset(&mut eng, &mut file);
              gr.play = true;
              tm = 100;
            }

            text[2].size.x = 20.0;
            text[2].size.y = 40.0;
            text[2].pos.x = eng.render.resolution_x as f32 / 2.0 - text[2].size.x * 2.0;
            text[2].pos.y = eng.render.resolution_y as f32 / 2.0 + text[2].size.y * 1.5;
            text[2].draw = true;
            text[2].signal = true;
            text[2].per_symbol = false;
            if text[2].exec(&mut eng, "Back") && eng.control.mousebtn[2] && tm <= 0{
              menusel = 1;
              gr.play = true;
              tm = 100;
            }

            text[3].draw = false;
            text[3].signal = false;
            text[3].exec(&mut eng, " ");

            text[4].draw = false;
            text[4].signal = false;
            text[4].exec(&mut eng, " ");
          },
          _ => {},
        }
      }else{
        for i in 2..5{
          text[i].draw = false;
          text[i].signal = false;
          text[i].exec(&mut eng, " ");
        }
      }

      if oldr[0] != eng.render.resolution_x || oldr[1] != eng.render.resolution_y{
        saveset(&mut eng, &mut file);
        oldr = [eng.render.resolution_x, eng.render.resolution_y];
      }
    }
    eng.end();
}
