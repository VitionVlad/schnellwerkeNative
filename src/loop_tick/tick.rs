use crate::{
    app_state::{distance, AppState, SPEED},
    engine::{engine::Engine, math::vec3::Vec3},
};

pub fn tick(eng: &mut Engine, state: &mut AppState) {
    if state.tm > 0 {
        state.tm -= eng.times_to_calculate_physics as i32;
    }
    if state.ttm > 0 {
        state.ttm -= eng.times_to_calculate_physics as i32;
        if state.ttm <= 0 {
            //state.intram = true;
            if state.dbg {
                println!("tram is starting");
            }
        }
    }

    for i in 0..state.scn.objects.len() {
        let lb = *state.scn.objects[i].name.as_bytes().last().unwrap();
        if lb != (state.current_light_scene+1).to_string().as_bytes()[0] && lb != b'0'{
            state.scn.objects[i].draw = false;
            state.scn.objects[i].draw_shadow = false;
        }else{
            state.scn.objects[i].draw = true;
            state.scn.objects[i].draw_shadow = true;
        }
    }

    state.scn.objects[state.pu].draw = true;
    state.scn.objects[state.pu].draw_shadow = true;
    state.scn.objects[state.tramin].draw = true;
    state.scn.objects[state.tramin].draw_shadow = true;

    state.viewport.ubo_index = 51;
    state.viewport.object.mesh.ubo[49] = state.scn.objects[state.pu].physic_object.pos.x;
    state.viewport.object.mesh.ubo[50] = state.scn.objects[state.pu].physic_object.pos.z;
    state.viewport.object.mesh.ubo[51] = state.pkbf;

    if !state.intram && state.framecnt > 10 && !state.gameending{
        if state.pkbf >= 5f32 {
            if state.pkbf >= 10f32{
                state.pkbf -= SPEED * eng.times_to_calculate_physics as f32;
            }else{
                state.pkbf = 1f32;
            }
        }else{
            if state.pkbf < 1_f32 {
                state.pkbf += SPEED * 5.0 * eng.times_to_calculate_physics as f32;
            }
            if state.pkbf > 1_f32 {
                state.pkbf -= SPEED * 5.0 * eng.times_to_calculate_physics as f32;
            }
            if (1.0 - SPEED * 5.0 * eng.times_to_calculate_physics as f32) < state.pkbf
                && (1.0 + SPEED * 5.0 * eng.times_to_calculate_physics as f32) > state.pkbf
            {
                state.pkbf = 1.0;
            }
        }
    }

    eng.cameras[0].physic_object.pos = Vec3 {
        x: state.scn.objects[state.pu].physic_object.pos.x - 7.5_f32,
        y: 10_f32,
        z: state.scn.objects[state.pu].physic_object.pos.z - 7.5_f32,
    };
    eng.cameras[0].fov = 37.5_f32;
    eng.cameras[0].physic_object.rot.x = 0.7_f32;
    eng.cameras[0].physic_object.rot.y = 2.355_f32;

    for i in 0..state.cvec.len() {
        if !state.cvec[i].consumed {
            let p1 = state.scn.objects[state.pu].physic_object.pos;
            let p2 = state.scn.objects[state.cvec[i].index].physic_object.pos;
            let d = distance(p1, p2);
            if d <= 5.0 && d > 0.5 {
                if p2.x > p1.x {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.x -=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                } else {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.x +=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                }
                if p2.z > p1.z {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.z -=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                } else {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.z +=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                }
            } else if d <= 0.5 {
                state.cvec[i].consumed = true;
                state.scn.objects[state.cvec[i].index].draw = false;
                state.scn.objects[state.cvec[i].index].draw_shadow = false;
                state.pkbf = 0.0;
                state.sfx[6].move_sound_cursor(0.0);
                state.sfx[6].play = true;
                match state.cvec[i].ctype {
                    0 => state.cme = true,
                    1 => state.bwfilm += 2,
                    2 => state.clfilm += 2,
                    _ => {}
                }
            }
        }else{
            state.scn.objects[state.cvec[i].index].draw = false;
            state.scn.objects[state.cvec[i].index].draw_shadow = false;
        }
    }

    state.scn.exec(eng);

    state.viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
    state.viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
    state.viewport.exec(eng);

    if state.showfps{
        let fpstxt = format!("fps:{}", eng.fps);
        state.fpscnt.size.x = 15_f32;
        state.fpscnt.size.y = 30_f32;
        state.fpscnt.pos.x = eng.render.resolution_x as f32 - fpstxt.len() as f32*state.fpscnt.size.x;
        state.fpscnt.pos.y = 0.0;
        state.fpscnt.draw = true;
        state.fpscnt.exec(eng, &fpstxt);
    }else{
        state.fpscnt.draw = false;
        state.fpscnt.exec(eng, " ");
    }

    if !state.pausemn{
        state.psbtn.object.physic_object.scale.x = 80.0;
        state.psbtn.object.physic_object.scale.y = 80.0;
        state.psbtn.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0;
        state.psbtn.object.physic_object.pos.y = eng.render.resolution_y as f32 - state.psbtn.object.physic_object.scale.y;
        if !state.cme || state.intram || state.selp == 3 {
            state.psbtn.object.physic_object.pos.x =
                eng.render.resolution_x as f32 / 2.0 - state.psbtn.object.physic_object.scale.x / 2.0;
        }
        if state.psbtn.exec(eng) && eng.control.mousebtn[2] && state.tm <= 0{
            state.pausemn = true;
            state.menusel = 0;
            state.selp = 0;
            state.tm = 50;
            state.current_letter = -1;
        }
        state.logo.object.draw = false;
        state.logo.exec(eng);
    }else{
        state.psbtn.object.draw = false;
        state.psbtn.exec(eng);
    }

    state.sfx[9].play = true;

    for i in 0..state.blacktxt.len(){
        if i != state.abc{
            state.blacktxt[i].draw = false;
            state.blacktxt[i].exec(eng, " ");
            state.phcnt[i].draw = false;
            state.phcnt[i].exec(eng, " ");
            for j in 0..state.ruitxt[i].len(){
                state.ruitxt[i][j].draw = false;
                state.ruitxt[i][j].exec(eng, " ");
            }
        }
    }
}
