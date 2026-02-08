#![allow(dead_code)]
use std::fs::{self};

use engine::{engine::Engine, image::Image, material::Material, ui::UIplane};

use crate::engine::{loader::glscene::Glscene, math::{vec2::Vec2, vec3::Vec3, vec4::Vec4}, model::Model, object::Object, scene::Scene, ui::UItext};
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
    let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
    let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    let roll = sinr_cosp.atan2(cosr_cosp);

    let sinp = 2.0 * (q.w * q.y - q.z * q.x);
    let pitch = if sinp.abs() >= 1.0 {
        std::f32::consts::FRAC_PI_2.copysign(sinp)
    } else {
        sinp.asin()
    };

    let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
    let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
    let yaw = siny_cosp.atan2(cosy_cosp);

    Vec3 { x: roll, y: pitch, z: yaw }
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

    let mut scn = Scene::new_blank();

    let gltfsc  = Glscene::readglb("assets/BRD1.glb");

    let mut ldmt = vec![];

    for i in 0..gltfsc.material_data.len(){
      let mut totdata = vec![];
      for j in 0..gltfsc.material_data[i].len(){
        totdata.extend_from_slice(&gltfsc.material_data[i][j].data);
      }
      ldmt.push(Image::new(&mut eng, [gltfsc.material_data[i][0].size[0], gltfsc.material_data[i][0].size[1], gltfsc.material_data[i].len() as u32], totdata));
    }

    for i in 0..gltfsc.objs.len(){
      let tobj = Model::new(&mut eng, gltfsc.objs[i].vertices.clone());
      scn.objects.push(Object::new(&mut eng, tobj, matgeneral, ldmt[gltfsc.objs[i].material], engine::render::render::MeshUsage::ShadowAndDefferedPass, true));
      let lobj = scn.objects.len()-1;
      scn.objects[lobj].physic_object.pos = gltfsc.objs[i].position;
      scn.objects[lobj].physic_object.scale = gltfsc.objs[i].scale;
      scn.objects[lobj].physic_object.rot = gltfsc.objs[i].rot;
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
        tm = 50;
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