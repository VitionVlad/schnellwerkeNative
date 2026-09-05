use std::println;

use crate::engine::{engine::Engine, image::Image, loader::rw::readfs, material::Material, math::{vec2::Vec2, vec3::Vec3, vec4::Vec4}, render::render::TextureFormat, scene::Scene, ui::{UIplane, UItext}};

mod engine;

fn main() {
    let mut eng = Engine::new(false);
    eng.render.set_title("rttest");
    eng.render.set_new_resolution(1280, 720);

    let vert = readfs("shaders/vert");
    let frag = readfs("shaders/frag");
    let dvert = readfs("shaders/vdeffered");
    let dfrag = readfs("shaders/fdeffered");
    let lfrag = readfs("shaders/klight");
    let shadow = readfs("shaders/shadow");
    let textf = readfs("shaders/ftext");
    //let imgf = readfs("shaders/fimg");

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mat = Material::new(
        &eng,
        vert.clone(),
        frag,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let lightmat = Material::new(
        &eng,
        vert.clone(),
        lfrag,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let matgeneral = Material::new(
        &eng,
        dvert.clone(),
        dfrag,
        shadow.clone(),
        [
            engine::render::render::CullMode::CullModeBackBit,
            engine::render::render::CullMode::CullModeFrontBit,
        ],
    );

    let black = Image::new_color(&eng, [11, 23, 40, u8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, vec![black], engine::render::render::MeshUsage::PostPass);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut fpscnt = UItext::new_from_file(
      &mut eng,
      matt,
      "assets/lat.png",
      "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
      engine::render::render::MeshUsage::PostPass
    );
    fpscnt.new_line_symbol = b'|';

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/test_field.glb", matgeneral, false, 0.2f32);

    //println!("{}, {}, {}, decomp_size: {}", scn.voxel_representation.size[0], scn.voxel_representation.size[1], scn.voxel_representation.size[2], scn.voxel_representation.data.len());

    //let d3d = unpack_svo_to_texture(&scn.voxel_representation.data, 5, 32);

    //let black3d = Image::new(&eng, scn.voxel_representation.size, scn.voxel_representation.data.clone(), true, TextureFormat::R8g8b8a8Unorm, false);

    //let black3d = Image::new(&eng, [scn.voxel_representation.data.len() as u32, 1, 1], scn.voxel_representation.data.clone(), false, TextureFormat::R8, false);

    let black3d = Image::new(&eng, [1, 1, 1], vec![u8::MAX, u8::MAX, u8::MAX, u8::MAX], true, TextureFormat::R8g8b8a8Unorm, false);

    let bluenoise = Image::new_from_files(&eng, vec!["assets/noise.png".to_string()]);

    let mut ltviewport = UIplane::new(&mut eng, lightmat, vec![black3d, bluenoise], engine::render::render::MeshUsage::LightingPass);
    ltviewport.object.physic_object.pos.z = 1.0;
    ltviewport.signal = false;

    scn.voxel_representation.data.resize(0, 0);

    let mut relpos = Vec2::new();

    let mut savpos = Vec2::new();

    let mut relposx = 0.0;

    const SPEED: f32 = 25.0f32;

    let mut tm = 0.0;

    //for i in 0..scn.objects.len() {
    //  scn.objects[i].draw_distance = f32::MAX;
    //  scn.objects[i].render_in_behind = false;
    //}

    eng.render.shadow_map_count = 0;
    eng.used_light_count = 1;

    eng.lights[0].shadow = false;
    eng.lights[0].light_type = engine::light::LightType::Spot;
    eng.lights[0].pos.x = 0.0f32;
    eng.lights[0].pos.y = 5.0f32;
    eng.lights[0].pos.z = 0.0f32;
    eng.lights[0].color.x = 1.25f32;
    eng.lights[0].color.y = 1.25f32;
    eng.lights[0].color.z = 1.25f32;

    eng.cameras[0].physic_object.pos.y = 5.0;

    let mut fcnt = 0u32;

    //eng.render.resolution_scale = 0.5;

    println!("shadowmapresolution(ignore if shadowmaps are off): {}", eng.render.shadow_map_resolution);

    let mut doors = vec![];

    let mut po = vec![];

    let mut lastmsmv = eng.control.xpos;

    let mut lastmsmvy = eng.control.ypos;

    let mut mousexmv = 0.0;

    let mut interactingph = false;

    let mut holdingph = false;

    let mut objrt = false;

    let mut active_object = -1;

    let mut rawrt = Vec3::new();

    for i in 0..scn.objects.len(){
      if scn.objects[i].name.contains("door_"){
        println!("object {} is door", scn.objects[i].name);
        scn.objects[i].physic_object.is_static = false;
        scn.objects[i].physic_object.gravity = false;
        scn.objects[i].physic_object.compute_torque = false;
        scn.objects[i].physic_object.pin_pos = true;
        doors.push((i, false, scn.objects[i].physic_object.rot, scn.objects[i].physic_object.rot));
      }else if scn.objects[i].name.contains("ph_"){
        println!("vertices count for object {}: {}", scn.objects[i].name, scn.objects[i].physic_object.collider.vertices.len());
        scn.objects[i].physic_object.is_static = false;
        po.push(i);
      }
    }

    eng.cameras[0].physic_object.collider.local_min.y = -1.5f32;

    while eng.work(){
        if tm > 0.0{
          tm -= eng.logic_frametime;
        }

        if !eng.control.mouse_lock {
          relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
          relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
          relposx = 0.0;
        }

        if eng.control.mouse_lock{
          if !interactingph && !objrt{
            eng.cameras[0].physic_object.rot = Vec3{x: -rawrt.x, y: -rawrt.y, z: -rawrt.z}.to_quat();
            rawrt.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
            rawrt.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - relpos.y;
            savpos.x = rawrt.x;
            savpos.y = rawrt.y;
          }else{
            relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
            relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
            relposx = 0.0;
          }
          if rawrt.x < -1.5 {
            relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x + 1.5;
            rawrt.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
          }
          if rawrt.x > 1.5 {
            relposx = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - 1.5;
            rawrt.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - relpos.x - relposx;
          }
          if eng.control.get_key_state(40){
            eng.cameras[0].physic_object.acceleration.z += f32::cos(rawrt.y) * SPEED;
            eng.cameras[0].physic_object.acceleration.x += f32::sin(rawrt.y) * -SPEED;
          }
          if eng.control.get_key_state(44){
            eng.cameras[0].physic_object.acceleration.z += f32::cos(rawrt.y) * -SPEED;
            eng.cameras[0].physic_object.acceleration.x += f32::sin(rawrt.y) * SPEED;
          }
          if eng.control.get_key_state(25){
            eng.cameras[0].physic_object.acceleration.x += f32::cos(rawrt.y) * SPEED;
            eng.cameras[0].physic_object.acceleration.z += f32::sin(rawrt.y) * SPEED;
          }
          if eng.control.get_key_state(22){
            eng.cameras[0].physic_object.acceleration.x += f32::cos(rawrt.y) * -SPEED;
            eng.cameras[0].physic_object.acceleration.z += f32::sin(rawrt.y) * -SPEED;
          }
          if !eng.control.mousebtn[2]{
            mousexmv = 0.0;
            holdingph = false;
            objrt = false;
            active_object = -1;
            for i in 0..doors.len(){
              interactingph = false;
              doors[i].1 = false;
            }
          }else{
            if active_object == -1{
              for i in 0..doors.len(){
                if (scn.objects[doors[i].0].is_looking_at || doors[i].1) && !holdingph{
                  interactingph = true;
                  doors[i].1 = true;
                  let delta = lastmsmv - eng.control.xpos;
                  mousexmv += delta;
                  scn.objects[doors[i].0].physic_object.angular_velocity.y = -(mousexmv as f32)/1000.0;
                  break;
                }
              }
              for i in 0..po.len(){
                if scn.objects[po[i]].is_looking_at && !interactingph{
                  scn.objects[po[i]].physic_object.pos.y += 0.001;
                  scn.objects[po[i]].physic_object.acceleration = Vec3{
                    x: (eng.cameras[0].physic_object.pos.x + f32::sin(rawrt.y) - scn.objects[po[i]].physic_object.pos.x)*100.0,
                    y: (eng.cameras[0].physic_object.pos.y + f32::sin(-rawrt.x) - scn.objects[po[i]].physic_object.pos.y)*100.0 + scn.objects[po[i]].physic_object.mass,
                    z: (eng.cameras[0].physic_object.pos.z - f32::cos(rawrt.y) - scn.objects[po[i]].physic_object.pos.z)*100.0,
                  };
                  holdingph = true;
                  active_object = i as i32;
                  break;
                }
              }
            }else{
              scn.objects[po[active_object as usize]].physic_object.acceleration = Vec3{
                x: (eng.cameras[0].physic_object.pos.x + f32::sin(rawrt.y) - scn.objects[po[active_object as usize]].physic_object.pos.x)*100.0,
                y: (eng.cameras[0].physic_object.pos.y + f32::sin(-rawrt.x) - scn.objects[po[active_object as usize]].physic_object.pos.y)*100.0 + scn.objects[po[active_object as usize]].physic_object.mass,
                z: (eng.cameras[0].physic_object.pos.z - f32::cos(rawrt.y) - scn.objects[po[active_object as usize]].physic_object.pos.z)*100.0,
              };
              holdingph = true;
              objrt = false;
              if eng.control.mousebtn[0]{
                objrt = true;
                let delta = lastmsmv - eng.control.xpos;
                let deltay = lastmsmvy - eng.control.ypos;
                scn.objects[po[active_object as usize]].physic_object.angular_velocity.y += (deltay as f32)/10.0;
                scn.objects[po[active_object as usize]].physic_object.angular_velocity.x += (delta as f32)/10.0;
              }
            }
          }
      }
      for i in 0..doors.len(){
        let door = &mut scn.objects[doors[i].0].physic_object;
        let relative_rot = door.rot.multiply(doors[i].3.conjugate());
        let relative_length = relative_rot.magnitude();

        if relative_length > f32::EPSILON {
          let relative_w = relative_rot.w / relative_length;
          let relative_y = relative_rot.y / relative_length;
          let current_angle = 2.0 * relative_y.atan2(relative_w);
          let clamped_angle = current_angle.clamp(-2.0943951, 0.0);

          if (current_angle - clamped_angle).abs() > f32::EPSILON {
            let clamped_rot = Vec4::from_axis_angle(
              Vec3 { x: 0.0, y: 1.0, z: 0.0 },
              clamped_angle,
            );
            door.rot = clamped_rot.multiply(doors[i].3);
            door.angular_velocity = Vec3::new();
          }
        }
      }
      if eng.control.get_key_state(49) && tm <= 0.0{
        eng.control.mouse_lock = !eng.control.mouse_lock;
        tm = 0.25;
      }

      let fpstxt = format!("CPU_fps:{}|GPU_fps:{}|CPU_frametime:{}", eng.cpu_fps, eng.gpu_fps, eng.logic_frametime);
      fpscnt.size.x = 15_f32;
      fpscnt.size.y = 30_f32;
      fpscnt.pos.x = 0.0;
      fpscnt.pos.y = 0.0;
      fpscnt.pos.z = 0.1;
      fpscnt.draw = true;
      fpscnt.exec(&mut eng, &fpstxt);

      scn.exec(&mut eng);
      ltviewport.object.physic_object.scale.x = eng.render.resolution_x as f32 * eng.render.resolution_scale;
      ltviewport.object.physic_object.scale.y = eng.render.resolution_y as f32 * eng.render.resolution_scale;
      ltviewport.object.physic_object.pos.z = 0.9;
      ltviewport.object.mesh.ubo[48] = fcnt as f32;
      ltviewport.object.mesh.ubo[49] = scn.voxel_representation.size[0] as f32;
      ltviewport.object.mesh.ubo[50] = scn.voxel_representation.size[1] as f32;
      ltviewport.object.mesh.ubo[51] = scn.voxel_representation.size[2] as f32;
      ltviewport.object.mesh.ubo[52] = scn.voxel_representation.origin.x as f32;
      ltviewport.object.mesh.ubo[53] = scn.voxel_representation.origin.y as f32;
      ltviewport.object.mesh.ubo[54] = scn.voxel_representation.origin.z as f32;
      ltviewport.object.mesh.ubo[55] = scn.voxel_representation.voxel_size;
      ltviewport.exec(&mut eng);

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.object.mesh.ubo[48] = fcnt as f32;
      viewport.object.mesh.ubo[49] = 0.0;
      viewport.object.mesh.ubo[50] = 0.2;
      viewport.object.mesh.ubo[51] = 0.18;
      viewport.exec(&mut eng);
      
      fcnt += 1;
      if fcnt > 100000{
        fcnt = 0;
      }
      lastmsmv = eng.control.xpos;
      lastmsmvy = eng.control.ypos;
    }
    eng.end();
}