#![allow(dead_code)]
use std::{f32::consts::PI, fs::{self}};

use engine::{engine::Engine, image::Image, material::Material, ui::UIplane};

use crate::engine::{loader::{gltf::Gltf, jsonparser::JsonF}, math::{vec2::Vec2, vec3::Vec3, vec4::Vec4}, model::Model, object::Object, scene::Scene, ui::UItext};
mod engine;

#[derive(Clone)]
#[derive(PartialEq)]
enum Rdbft{
  SCALAR,
  VEC2,
  VEC3,
  VEC4
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Aus{
  POSITION,
  NORMAL,
  UV,
  INDICES,
  OTHER,
}

struct Rdbf{
  tp: Rdbft,
  mu: Aus,
  scalar: Vec<u32>,
  vec2: Vec<Vec2>,
  vec3: Vec<Vec3>,
  vec4: Vec<Vec4>,
}

fn quat_to_euler(q: Vec4) -> Vec3 {
    //let x = q.x;
    //let y = q.y;
    //let z = q.z;
    //let w = q.w;
//
    //let sinr_cosp = 2.0 * (w * x + y * z);
    //let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
    //let roll = sinr_cosp.atan2(cosr_cosp);
//
    //let sinp = 2.0 * (w * y - z * x);
    //let pitch = if sinp.abs() >= 1.0 {
    //    (std::f32::consts::PI / 2.0).copysign(sinp)
    //} else {
    //    sinp.asin()
    //};
//
    //let siny_cosp = 2.0 * (w * z + x * y);
    //let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
    //let yaw = siny_cosp.atan2(cosy_cosp);
//
    //Vec3 { x: roll, y: pitch, z: yaw }

    let mut angles = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
    let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    angles.x = f32::atan2(sinr_cosp, cosr_cosp);

    let sinp = f32::sqrt(1.0 + 2.0 * (q.w * q.y - q.x * q.z));
    let cosp = f32::sqrt(1.0 - 2.0 * (q.w * q.y - q.x * q.z));
    angles.y = 2.0 * f32::atan2(sinp, cosp) - PI / 2.0;

    let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
    let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
    angles.z = f32::atan2(siny_cosp, cosy_cosp);

    angles
}

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    //let mut wkfc = 2.0f32;
    eng.render.set_title("ARSD");
    eng.render.set_new_resolution(1280, 720);

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();

    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matgeneral = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let black = Image::new_color(&eng, [0, 0, 0, i8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;

    let mut fpscnt = UItext::new_from_file(&mut eng, matt, "assets/textlat.png", "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%");

    let jgltf = JsonF::load_from_file("assets/BRD2.gltf");
    let pgltf = Gltf::parse_gltf(jgltf);

    let mut scn = Scene::new_blank();

    let mut matimg = vec![];
    let mut rwbf = vec![];

    for i in 0..pgltf.materials.len(){
      let mut uris = vec![];
      for j in 0..pgltf.materials[i].texture_indices.len(){
        let str = format!("assets/{}", pgltf.images[pgltf.textures[pgltf.materials[i].texture_indices[j]].image].uri.clone());
        uris.push(str.clone());
      }
      matimg.push(Image::new_from_files(&eng, uris));
    }

    for i in 0..pgltf.buffers.len(){
      rwbf.push(fs::read(format!("assets/{}", pgltf.buffers[i].uri.clone())).unwrap());
    }

    for gobj in pgltf.objects{
      let mesh = pgltf.meshes[gobj.mesh].clone();
      let mut acc = vec![];
      let mut accu = vec![];
      let mut bfv = vec![];

      let mut sbf: Vec<Rdbf> = vec![];

      for i in 0..mesh.attributes.len(){
        acc.push(pgltf.accesories[mesh.attributes[i]].clone());
        if mesh.attributesu[i] == "POSITION"{
          accu.push(Aus::POSITION);
        }else if mesh.attributesu[i] == "NORMAL"{
          accu.push(Aus::NORMAL);
        }else if mesh.attributesu[i] == "TEXCOORD_0"{
          accu.push(Aus::UV);
        }else{
          accu.push(Aus::OTHER);
        }
      }
      if mesh.enable_indices{
        acc.push(pgltf.accesories[mesh.indices].clone());
        accu.push(Aus::INDICES);
      }

      for i in 0..acc.len(){
        bfv.push(pgltf.bufferview[acc[i].bufferview]);
      }

      let mut bfvp = vec![];
      for i in 0..bfv.len(){
        let mut lbf = vec![];
        for j in bfv[i].boffset..(bfv[i].boffset+bfv[i].blenght){
          lbf.push(rwbf[bfv[i].buffer][j]);
        }
        bfvp.push(lbf);
      }

      for i in 0..acc.len(){
        if acc[i].tp == "SCALAR"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(2){
            lbf.push(u16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as u32);
          }
          sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: accu[i].clone(), scalar: lbf, vec2: vec![], vec3: vec![], vec4: vec![] });
        }else if acc[i].tp == "VEC2"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(8){
            lbf.push(Vec2{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC2, mu: accu[i].clone(), scalar: vec![], vec2: lbf, vec3: vec![], vec4: vec![] });
        }else if acc[i].tp == "VEC3"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec3{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC3, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: lbf, vec4: vec![] });
        }else if acc[i].tp == "VEC4"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec4{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]]),
              w: f32::from_le_bytes([bfvp[i][j+12], bfvp[i][j+13], bfvp[i][j+14], bfvp[i][j+15]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC4, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: vec![], vec4: lbf });
        }
      }

      let mut fvrt = vec![];

      let mut pi = 0usize;
      let mut ni = 0usize;
      let mut uvi = 0usize;
      let mut ii = 0usize;

      for i in 0..sbf.len(){
        if sbf[i].mu == Aus::INDICES{
          ii = i;
        }else if sbf[i].mu == Aus::POSITION{
          pi = i;
        }else if sbf[i].mu == Aus::UV{
          uvi = i;
        }else if sbf[i].mu == Aus::NORMAL{
          ni = i;
        }
      }

      for i in 0..sbf[ii].scalar.len(){
        fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize].x);
        fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize].y);
        fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize].z);

        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 1].x);
        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 1].y);
        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 1].z);

        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 2].x);
        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 2].y);
        //fvrt.push(sbf[pi].vec3[sbf[ii].scalar[i] as usize * 3 + 2].z);
      }

      for i in 0..sbf[ii].scalar.len(){
        fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize].x);
        fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize].y);

        //fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize * 2 + 1].x);
        //fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize * 2 + 1].y);

        //fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize * 2 + 2].x);
        //fvrt.push(sbf[uvi].vec2[sbf[ii].scalar[i] as usize * 2 + 2].y);
      }

      for i in 0..sbf[ii].scalar.len(){
        fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize].x);
        fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize].y);
        fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize].z);

        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 1].x);
        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 1].y);
        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 1].z);

        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 2].x);
        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 2].y);
        //fvrt.push(sbf[ni].vec3[sbf[ii].scalar[i] as usize * 3 + 2].z);
      }

      let md = Model::new(&mut eng, fvrt);

      scn.objects.push(Object::new(&mut eng, md, matgeneral, matimg[mesh.material], engine::render::render::MeshUsage::ShadowAndDefferedPass, true));
      let lem = scn.objects.len()-1;
      scn.objects[lem].physic_object.pos = Vec3 { x: gobj.position.x, y: gobj.position.y, z: gobj.position.z };
      scn.objects[lem].physic_object.scale = Vec3 { x: gobj.scale.x, y: gobj.scale.y, z: gobj.scale.z };
      scn.objects[lem].physic_object.rot = quat_to_euler(gobj.rotation);

    }

    scn.use_global_values = false;

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;

    let mut relpos = Vec2::new();

    let mut savpos = Vec2::new();

    let mut relposx = 0.0;

    let mut tm: i32 = 0;

    eng.render.shadow_map_resolution = 4000;

    while eng.work(){
      if tm > 0{
        tm -= eng.times_to_calculate_physics as i32;
      }

      if !eng.control.mouse_lock{
        relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
        relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
        relposx = 0.0;
      }

      if eng.control.get_key_state(40){
        eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.x += f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.y += f32::sin(eng.cameras[0].physic_object.rot.x) * SPEED * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(44){
        eng.cameras[0].physic_object.acceleration.z += f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.x += f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.y += f32::sin(eng.cameras[0].physic_object.rot.x) * -SPEED * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(25){
        eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.z += f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(22){
        eng.cameras[0].physic_object.acceleration.x += f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
        eng.cameras[0].physic_object.acceleration.z += f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED * eng.times_to_calculate_physics as f32;
      }
      if eng.control.get_key_state(49) && tm <= 0{
        eng.control.mouse_lock = !eng.control.mouse_lock;
        //pause = !pause;
        tm = 50;
        //pausemn = 0;
        //gmus = false;
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

        if eng.control.mousebtn[0] && tm <= 0{
          eng.used_light_count += 1;
          eng.lights[eng.used_light_count as usize - 1 as usize].pos = eng.cameras[0].physic_object.pos;
          eng.lights[eng.used_light_count as usize - 1 as usize].rot = eng.cameras[0].physic_object.rot;
          eng.lights[eng.used_light_count as usize - 1 as usize].color = Vec3 { x: 10.0, y: 10.0, z: 10.0 };
          eng.lights[eng.used_light_count as usize - 1 as usize].light_type = engine::light::LightType::Spot;
          eng.lights[eng.used_light_count as usize - 1 as usize].shadow = true;
          tm = 50;
          println!("light source created");
        }
        if eng.control.mousebtn[2] && eng.used_light_count > 0 {
          eng.lights[eng.used_light_count as usize - 1 as usize].pos = eng.cameras[0].physic_object.pos;
          eng.lights[eng.used_light_count as usize - 1 as usize].rot = eng.cameras[0].physic_object.rot;
          eng.lights[eng.used_light_count as usize - 1 as usize].color = Vec3 { x: 10.0, y: 10.0, z: 10.0 };
          eng.lights[eng.used_light_count as usize - 1 as usize].light_type = engine::light::LightType::Spot;
          eng.lights[eng.used_light_count as usize - 1 as usize].shadow = true;
        }
      }

      scn.exec(&mut eng);

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
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