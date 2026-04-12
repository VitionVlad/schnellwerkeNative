#![allow(dead_code)]
use std::{f32::consts::PI, fs::{self}};

use engine::{engine::Engine, image::Image, material::Material, ui::UIplane};

use crate::engine::{math::vec3::Vec3, scene::Scene, ui::UItext};
mod engine;

struct Colectable{
  index: usize,
  ctype: u8,
  consumed: bool,
}
pub fn distance(v1: Vec3, v2: Vec3) -> f32{
  f32::sqrt((v2.x - v1.x).powi(2) + (v2.z - v1.z).powi(2))
}

fn main() {
    const SPEED: f32 = 0.0025f32;
    const TICKSZ: f32 = 1.0/250.0;
    let mut eng = Engine::new();
    //let mut wkfc = 2.0f32;
    eng.render.set_title("35mm");
    eng.render.set_new_resolution(800, 600);

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();

    let matt = Material::new(&eng, vert.clone(), textf, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let mat = Material::new(&eng, vert.clone(), frag, vec![], [engine::render::render::CullMode::CullModeNone, engine::render::render::CullMode::CullModeNone]);
    let matgeneral = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
    let black = Image::new_color(&eng, [0, 0, 0, u8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut fpscnt = UItext::new_from_file(&mut eng, matt, "assets/textlat.png", "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%");

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/test1.glb", matgeneral);

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;

    let mut cvec = vec![];

    //let mut relpos = Vec2::new();

    //let mut savpos = Vec2::new();

    //let mut relposx = 0.0;

    let mut tm: i32 = 0;

    eng.render.shadow_map_resolution = 4000;

    let mut pu = 0usize;

    let mut pivotr = 0.0f32;

    let mut pkbf = 1f32;

    for i in 0..scn.objects.len(){
      scn.objects[i].draw_distance = 1000f32;
      if scn.objects[i].name == "Pivot"{
        pu = i;
      }
      else{
        let bt = scn.objects[i].name.as_bytes();
        if bt[0] == b'c' && bt[1] == b'a' && bt[2] == b'm'{
          scn.objects[i].physic_object.gravity = false;
          scn.objects[i].physic_object.is_static = false;
          scn.objects[i].physic_object.solid = false;
          cvec.push(Colectable{
            index: i,
            ctype: 0,
            consumed: false,
          });
        }else if bt[0] == b'b' && bt[1] == b'w' && bt[2] == b'f'{
          scn.objects[i].physic_object.gravity = false;
          scn.objects[i].physic_object.is_static = false;
          scn.objects[i].physic_object.solid = false;
          cvec.push(Colectable{
            index: i,
            ctype: 1,
            consumed: false,
          });
        }else if bt[0] == b'c' && bt[1] == b'l' && bt[2] == b'f'{
          scn.objects[i].physic_object.gravity = false;
          scn.objects[i].physic_object.is_static = false;
          scn.objects[i].physic_object.solid = false;
          cvec.push(Colectable{
            index: i,
            ctype: 2,
            consumed: false,
          });
        }
      }
    }

    println!("{}", cvec.len());

    scn.objects[pu].physic_object.gravity = true;
    scn.objects[pu].physic_object.is_static = false;
    scn.objects[pu].physic_object.solid = true;
    scn.objects[pu].physic_object.step_height = 0.1;

    let mut bwfilm = 0u32;
    let mut clfilm = 0u32;
    let mut cme = false;

    while eng.work(){
      if tm > 0{
        tm -= eng.times_to_calculate_physics as i32;
      }

      viewport.ubo_index = 51;
      viewport.object.mesh.ubo[49] = scn.objects[pu].physic_object.pos.x;
      viewport.object.mesh.ubo[50] = scn.objects[pu].physic_object.pos.z;
      viewport.object.mesh.ubo[51] = pkbf;

      if pkbf < 1f32{
        pkbf += SPEED*5.0*eng.times_to_calculate_physics as f32;
      }

      if eng.control.get_key_state(40){
        scn.objects[pu].physic_object.acceleration.x += -SPEED*eng.times_to_calculate_physics as f32;
        //scn.objects[pu].physic_object.rot.y = PI/2.0;
        pivotr = PI/2.0;
      }
      else if eng.control.get_key_state(44){
        scn.objects[pu].physic_object.acceleration.x += SPEED*eng.times_to_calculate_physics as f32;
        //scn.objects[pu].physic_object.rot.y = (PI/2.0)*3.0;
        pivotr = (PI/2.0)*3.0;
      }
      else if eng.control.get_key_state(25){
        scn.objects[pu].physic_object.acceleration.z += SPEED*eng.times_to_calculate_physics as f32;
        //scn.objects[pu].physic_object.rot.y = PI;
        pivotr = PI;
      }
      else if eng.control.get_key_state(22){
        scn.objects[pu].physic_object.acceleration.z += -SPEED*eng.times_to_calculate_physics as f32;
        //scn.objects[pu].physic_object.rot.y = 0.0;
        pivotr = 0.0;
      }

      let step = SPEED * eng.times_to_calculate_physics as f32 * 20.0;
      let error_margin = SPEED * 5.0;
      let mut delta = (pivotr - scn.objects[pu].physic_object.rot.y + std::f32::consts::PI) % (2.0 * std::f32::consts::PI) - std::f32::consts::PI;
      if delta < -std::f32::consts::PI {
          delta += 2.0 * std::f32::consts::PI;
      }
      if delta.abs() <= error_margin{
        scn.objects[pu].physic_object.rot.y = pivotr;
      } else {
        let direction = delta.signum(); 
        let movement = direction * step;
        if step > delta.abs(){
          scn.objects[pu].physic_object.rot.y = pivotr;
        }else{
          scn.objects[pu].physic_object.rot.y += movement;
        }
      }

      eng.cameras[0].physic_object.pos = Vec3 { 
        x: scn.objects[pu].physic_object.pos.x - 7.5f32, 
        y: 10f32, 
        z: scn.objects[pu].physic_object.pos.z - 7.5f32 
      };
      eng.cameras[0].fov = 35f32;
      eng.cameras[0].physic_object.rot.x = 0.7f32;
      eng.cameras[0].physic_object.rot.y = 2.355f32;

      eng.lights[0].camera.physic_object.pos = Vec3 { 
        x: scn.objects[pu].physic_object.pos.x - 47.5f32, 
        y: 55f32, 
        z: scn.objects[pu].physic_object.pos.z - 47.5f32 
      };
      eng.lights[0].color = Vec3 { x: 1.0, y: 0.9, z: 0.8 };
      eng.lights[0].light_type = engine::light::LightType::Directional;
      eng.lights[0].direction = Vec3 { x: 1.0f32, y: -1.0f32, z: 1.0f32 };
      eng.lights[0].pos = eng.lights[0].camera.physic_object.pos;
      eng.lights[0].rot.x = 0.7f32;
      eng.lights[0].rot.y = 2.355f32;
      eng.lights[0].camera.fov = 20f32;

      for i in 0..cvec.len(){
        if !cvec[i].consumed{
          let p1 = scn.objects[pu].physic_object.pos;
          let p2 = scn.objects[cvec[i].index].physic_object.pos;
          let d = distance(p1, p2);
          if d <= 5.0 && d > 0.5 {
            if p2.x > p1.x{
              scn.objects[cvec[i].index].physic_object.acceleration.x -= 2.0*SPEED*eng.times_to_calculate_physics as f32;
            }else{
              scn.objects[cvec[i].index].physic_object.acceleration.x += 2.0*SPEED*eng.times_to_calculate_physics as f32;
            }
            if p2.z > p1.z{
              scn.objects[cvec[i].index].physic_object.acceleration.z -= 2.0*SPEED*eng.times_to_calculate_physics as f32;
            }else{
              scn.objects[cvec[i].index].physic_object.acceleration.z += 2.0*SPEED*eng.times_to_calculate_physics as f32;
            }
          }else if d <= 0.5{
            cvec[i].consumed = true;
            scn.objects[cvec[i].index].draw = false;
            scn.objects[cvec[i].index].draw_shadow = false;
            pkbf = 0.0;
            match cvec[i].ctype {
                0 => {cme = true},
                1 => {bwfilm += 12},
                2 => {clfilm += 12},
                _ => {}
            }
          }
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