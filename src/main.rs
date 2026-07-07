use crate::engine::{engine::Engine, image::Image, loader::rw::readfs, material::Material, math::vec2::Vec2, render::render::TextureFormat, scene::Scene, ui::{UIplane, UItext}};

mod engine;

fn main() {
    let mut eng = Engine::new(false);
    eng.render.set_title("rttest");
    eng.render.set_new_resolution(1280, 720);

    let vert = readfs("shaders/vert");
    let frag = readfs("shaders/frag");
    let dvert = readfs("shaders/vdeffered");
    let dfrag = readfs("shaders/fdeffered");
    let lfrag = readfs("shaders/flight");
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

    let black3d = Image::new(&eng, [2, 1, 1], vec![11, 23, 40, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX], true, TextureFormat::R8g8b8a8Unorm);

    let mut viewport = UIplane::new(&mut eng, mat, black, engine::render::render::MeshUsage::PostPass);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut ltviewport = UIplane::new(&mut eng, lightmat, black3d, engine::render::render::MeshUsage::LightingPass);
    ltviewport.object.physic_object.pos.z = 1.0;
    ltviewport.signal = false;

    let mut fpscnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/lat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
        engine::render::render::MeshUsage::PostPass
    );

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/scene.glb", matgeneral);

    let mut relpos = Vec2::new();

    let mut savpos = Vec2::new();

    let mut relposx = 0.0;

    const SPEED: f32 = 0.0025f32;

    let mut tm = 0;

    for i in 0..scn.objects.len() {
        scn.objects[i].draw_distance = 500.0;
    }

    eng.render.shadow_map_count = 0;
    eng.used_light_count = 0;

    let mut fcnt = 0u32;

    while eng.work(){
        eng.cameras[0].physic_object.gravity = false;
        eng.cameras[0].physic_object.solid = false;

        if tm > 0{
          tm -= eng.times_to_calculate_physics as i32;
        }

        if !eng.control.mouse_lock {
          relpos.x = (eng.control.ypos) as f32/eng.render.resolution_y as f32 - savpos.x;
          relpos.y = (eng.control.xpos) as f32/eng.render.resolution_x as f32 - savpos.y;
          relposx = 0.0;
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
        }

        if eng.control.get_key_state(49) && tm <= 0{
          eng.control.mouse_lock = !eng.control.mouse_lock;
          tm = 100;
        }

        let fpstxt = format!("fps:{}", eng.fps);
        fpscnt.size.x = 15_f32;
        fpscnt.size.y = 30_f32;
        fpscnt.pos.x = eng.render.resolution_x as f32 - fpstxt.len() as f32*fpscnt.size.x;
        fpscnt.pos.y = 0.0;
        fpscnt.draw = true;
        fpscnt.exec(&mut eng, &fpstxt);

        scn.exec(&mut eng);

        ltviewport.object.physic_object.scale.x = eng.render.resolution_x as f32 * eng.render.resolution_scale;
        ltviewport.object.physic_object.scale.y = eng.render.resolution_y as f32 * eng.render.resolution_scale;
        ltviewport.object.mesh.ubo[48] = fcnt as f32;
        ltviewport.exec(&mut eng);

        viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
        viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
        viewport.object.mesh.ubo[48] = 0.2;
        viewport.object.mesh.ubo[49] = 1.0;
        viewport.exec(&mut eng);
        fcnt += 1;
    }
    eng.end();
}