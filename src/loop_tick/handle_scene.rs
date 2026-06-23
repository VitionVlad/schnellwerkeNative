use std::f32::consts::PI;

use crate::{
    app_state::*,
    engine::{engine::Engine, light::LightType, math::vec3::Vec3}, loop_tick::save_load::save_progress,
};

pub fn reset_final_door_game(state: &mut AppState) {
    state.cstop = 0;
    state.current_light_scene = 0;
    state.tm = 0;
    state.intram = false;
    state.sc3state = 0;
    state.switched_1_4 = false;
    state.switched_5_6 = false;
    state.switch_states = [false; 6];
    state.selp = 0;
    state.cme = false;
    state.bwfilm = 0;
    state.clfilm = 0;
    state.locls = 1;

    for button in state.btns.iter_mut() {
        button.pressed = false;
    }

    // reset collectibles to initial positions and state
    for c in state.cvec.iter_mut() {
        state.scn.objects[c.index].physic_object.pos = c.initial_pos;
        c.consumed = false;
        state.scn.objects[c.index].draw = true;
        state.scn.objects[c.index].draw_shadow = true;
        state.scn.objects[c.index].physic_object.reset_states();
    }

    // reset destructables to initial positions and state
    for d in state.destructables.iter_mut() {
        state.scn.objects[d.index].physic_object.pos = d.initial_pos;
        state.scn.objects[d.index].draw = true;
        state.scn.objects[d.index].draw_shadow = true;
        d.destroyed = false;
        state.scn.objects[d.index].physic_object.reset_states();
    }

    // restore key indices
    state.ekey = state.initial_ekey;
    state.gkey = state.initial_gkey;

    let player_pos = &mut state.scn.objects[state.pu].physic_object.pos;
    player_pos.x = state.initial_pivot_pos.x;
    player_pos.y = state.initial_pivot_pos.y;
    player_pos.z = state.initial_pivot_pos.z;

    state.scn.objects[state.tramin].physic_object.pos.x = 6.34336;

    if state.dbg {
        println!("final door activated: resetting game state and player position");
    }

    let _ = save_progress("save.json", state);
}

fn handle_end(eng: &mut Engine, state: &mut AppState){
    if state.simtim > 0{
        state.simtim -= eng.times_to_calculate_physics as i32;
    }

    if state.pkbf >= 10f32 && state.pkbf < 11f32{
        state.pkbf += SPEED * eng.times_to_calculate_physics as f32;
    }
    if state.pkbf > 11f32{
        state.pkbf = 11.0;
    }

    if state.pkbf >= 11.0{
        state.blacktxt[state.abc].draw = true;
        state.blacktxt[state.abc].size.x = 10_f32;
        state.blacktxt[state.abc].size.y = 20_f32;    
        state.blacktxt[state.abc].max_text_width = 16;
        state.blacktxt[state.abc].pos.y = 25.0;
        state.blacktxt[state.abc].pos.x = eng.render.resolution_x as f32 / 2.0 - 150.0;
        state.blacktxt[state.abc].next_line_on_whitespace = true;
        state.blacktxt[state.abc].new_line_symbol = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[1].strval.bytes().nth(0).unwrap_or(0);
        let ort = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[2].other_nodes[3].strval.clone();
        let text: String = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[2].other_nodes[3].strval.clone().chars().take(state.lastltsim).collect();
        state.blacktxt[state.abc].exec(eng, &text);
        if state.simtim <= 0 && state.lastltsim != ort.len(){
            state.lastltsim += 1;
            state.sfx[10].play = true;
            state.sfx[10].move_sound_cursor(0.0);
            state.simtim = 25;
        }
        if state.simtim <= 0 && state.lastltsim == ort.len(){
            state.current_light_scene = 0;
            state.drbtn.object.draw = false;
            state.drbtn.exec(eng);
            state.gameending = false;
            state.pkbf = 2.0;
            state.sfx[1].move_sound_cursor(0.0);
            state.sfx[1].play = true;
            reset_final_door_game(state);
        }
    }

    state.drbtn.object.draw = false;
    state.nkbtn.object.draw = false;
    state.nebtn.object.draw = false;
    state.nkbtn.exec(eng);
    state.drbtn.exec(eng);
    state.nebtn.exec(eng);
}

fn handle_final_door_interaction(eng: &mut Engine, state: &mut AppState) {
    let player_pos = state.scn.objects[state.pu].physic_object.pos;
    let door_pos = state.scn.objects[state.finaldooridx].physic_object.pos;
    let dist = distance(player_pos, door_pos);

    if dist <= 1.5 && state.selp == 0 {
        let can_open = state.gkey == usize::MAX && state.sc3state == 2;
        let icon = if can_open { 
            &mut state.drbtn 
        } else if state.gkey == usize::MAX && state.sc3state != 2 { 
            &mut state.nebtn 
        } else { 
            &mut state.nkbtn 
        };
        icon.object.physic_object.scale.x = 80.0;
        icon.object.physic_object.scale.y = 80.0;
        icon.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0 - icon.object.physic_object.scale.x / 2.0;
        icon.object.physic_object.pos.y = eng.render.resolution_y as f32 - icon.object.physic_object.scale.y * 2.0 - 20.0;
        icon.object.draw = true;
        let icon_pressed = icon.exec(eng) && can_open && eng.control.mousebtn[2];

        let abtn_pressed = eng.control.gamepad_button_count > 0 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && can_open;

        if can_open && ((eng.control.get_key_state(state.keycodes[0]) || icon_pressed || abtn_pressed) && state.tm <= 0) {
            //state.sfx[3].move_sound_cursor(0.0);
            //state.sfx[3].play = true;
            //state.current_light_scene = 0;
            //state.drbtn.object.draw = false;
            //state.drbtn.exec(eng);
            //reset_final_door_game(state);
            //state.cstop = 0;
            //state.current_light_scene = 0;
            //state.tm = 0;
            state.intram = false;
            //state.sc3state = 0;
            state.selp = 0;
            state.cme = false;
            state.bwfilm = 0;
            state.clfilm = 0;
            state.locls = 1;
            state.pkbf = 10.0;
            state.gameending = true;
            state.drbtn.object.draw = false;
            state.nkbtn.object.draw = false;
            state.nebtn.object.draw = false;
            state.nkbtn.exec(eng);
            state.drbtn.exec(eng);
            state.nebtn.exec(eng);
        }
    } else {
        state.drbtn.object.draw = false;
        state.nkbtn.object.draw = false;
        state.nebtn.object.draw = false;
        state.nkbtn.exec(eng);
        state.drbtn.exec(eng);
        state.nebtn.exec(eng);
    }
}

fn process_lighting(eng: &mut Engine, state: &mut AppState) {
    match state.current_light_scene {
        0 => {
            eng.render.shadow_map_resolution = state.shadowmapquality;
            eng.lights[0].camera.physic_object.pos = Vec3 {
                x: state.scn.objects[state.pu].physic_object.pos.x - 47.5_f32,
                y: 55_f32,
                z: state.scn.objects[state.pu].physic_object.pos.z - 47.5_f32,
            };
            eng.lights[0].light_type = LightType::Directional;
            eng.lights[0].direction = Vec3 {
                x: 1.0_f32,
                y: -1.0_f32,
                z: 1.0_f32,
            };
            eng.lights[0].pos = eng.lights[0].camera.physic_object.pos;
            eng.lights[0].rot.x = 0.7_f32;
            eng.lights[0].rot.y = 2.355_f32;
            eng.lights[0].camera.fov = 20_f32;
            eng.lights[0].shadow = true;
            eng.used_light_count = 1;

            if state.selp == 1{
                eng.used_light_count = 2;
                eng.lights[0].color = Vec3 {
                    x: 0.08,
                    y: 0.09,
                    z: 0.1,
                };

                eng.lights[1].rot.y = -state.scn.objects[state.pu].physic_object.rot.y;
                eng.lights[1].rot.x = 0.0;
                eng.lights[1].pos.x =
                    state.scn.objects[state.pu].physic_object.pos.x - state.scn.objects[state.pu].physic_object.rot.y.sin() * 0.3;
                eng.lights[1].pos.y = state.scn.objects[state.pu].physic_object.pos.y;
                eng.lights[1].pos.z =
                    state.scn.objects[state.pu].physic_object.pos.z - state.scn.objects[state.pu].physic_object.rot.y.cos() * 0.3;
                eng.lights[1].light_type = crate::engine::light::LightType::Spot;
                eng.lights[1].camera.fov = 90.0;
                eng.lights[1].shadow = true;
                eng.lights[1].color = Vec3 {
                    x: 5.0,
                    y: 5.0,
                    z: 5.0,
                };
            }
        }
        1 => {
            eng.render.shadow_map_resolution = state.shadowmapquality / 2;

            eng.lights[0].camera.physic_object.pos = Vec3 {
                x: state.scn.objects[state.pu].physic_object.pos.x - 47.5_f32,
                y: 55_f32,
                z: state.scn.objects[state.pu].physic_object.pos.z - 47.5_f32,
            };
            eng.lights[0].light_type = LightType::Directional;
            eng.lights[0].direction = Vec3 {
                x: 0.0_f32,
                y: 0.0_f32,
                z: 0.0_f32,
            };

            match state.selp {
                1 => {
                    eng.lights[0].color = Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                },
                2 => {
                    eng.lights[0].color = Vec3 {
                        x: 0.5,
                        y: 0.45,
                        z: 0.5,
                    };
                },
                _ => {
                     eng.lights[0].color = Vec3 {
                        x: 0.08,
                        y: 0.09,
                        z: 0.1,
                    };
                }
            }
            eng.lights[0].shadow = false;

            eng.lights[1].rot.y = -state.scn.objects[state.pu].physic_object.rot.y;
            eng.lights[1].rot.x = 0.0;
            eng.lights[1].pos.x =
                state.scn.objects[state.pu].physic_object.pos.x - state.scn.objects[state.pu].physic_object.rot.y.sin() * 0.3;
            eng.lights[1].pos.y = state.scn.objects[state.pu].physic_object.pos.y;
            eng.lights[1].pos.z =
                state.scn.objects[state.pu].physic_object.pos.z - state.scn.objects[state.pu].physic_object.rot.y.cos() * 0.3;
            eng.lights[1].light_type = crate::engine::light::LightType::Spot;
            eng.lights[1].camera.fov = 90.0;
            eng.lights[1].color = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            eng.used_light_count = 2;

            if state.switched_1_4 && state.switched_5_6 {
                for i in 0..state.scenelightsources.len() {
                    if distance(state.scenelightsources[i].pos, state.scn.objects[state.pu].physic_object.pos) < 20.0 {
                        eng.lights[eng.used_light_count as usize].light_type = LightType::Spot;
                        eng.lights[eng.used_light_count as usize].rot.y = 0.0;
                        eng.lights[eng.used_light_count as usize].rot.x = PI / 2.0;
                        eng.lights[eng.used_light_count as usize].pos.x = state.scenelightsources[i].pos.x;
                        eng.lights[eng.used_light_count as usize].pos.y = state.scenelightsources[i].pos.y;
                        eng.lights[eng.used_light_count as usize].pos.z = state.scenelightsources[i].pos.z;
                        eng.lights[eng.used_light_count as usize].light_type = crate::engine::light::LightType::Spot;
                        eng.lights[eng.used_light_count as usize].camera.fov = 110.0;
                        eng.lights[eng.used_light_count as usize].color = Vec3 {
                            x: 1.0,
                            y: 0.9,
                            z: 0.5,
                        };
                        eng.lights[eng.used_light_count as usize].shadow = true;
                        eng.used_light_count+=1;
                    }
                }
            }
        }
        2 => {
            eng.render.shadow_map_resolution = state.shadowmapquality / 2;

            eng.lights[0].camera.physic_object.pos = Vec3 {
                x: state.scn.objects[state.pu].physic_object.pos.x - 47.5_f32,
                y: 55_f32,
                z: state.scn.objects[state.pu].physic_object.pos.z - 47.5_f32,
            };
            eng.lights[0].light_type = LightType::Directional;
            eng.lights[0].direction = Vec3 {
                x: 0.0_f32,
                y: 0.0_f32,
                z: 0.0_f32,
            };

            match state.selp {
                1 => {
                    eng.lights[0].color = Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                },
                2 => {
                    eng.lights[0].color = Vec3 {
                        x: 0.5,
                        y: 0.45,
                        z: 0.5,
                    };
                },
                _ => {
                     eng.lights[0].color = Vec3 {
                        x: 0.08,
                        y: 0.09,
                        z: 0.1,
                    };
                }
            }
            eng.lights[0].shadow = false;

            eng.used_light_count = 1;

            for i in 0..state.scenelightsources.len() {
                if distance(state.scenelightsources[i].pos, state.scn.objects[state.pu].physic_object.pos) < 20.0 {
                    eng.lights[eng.used_light_count as usize].light_type = LightType::Spot;
                    eng.lights[eng.used_light_count as usize].rot.y = 0.0;
                    eng.lights[eng.used_light_count as usize].rot.x = PI / 2.0;
                    eng.lights[eng.used_light_count as usize].pos.x = state.scenelightsources[i].pos.x;
                    eng.lights[eng.used_light_count as usize].pos.y = state.scenelightsources[i].pos.y;
                    eng.lights[eng.used_light_count as usize].pos.z = state.scenelightsources[i].pos.z;
                    eng.lights[eng.used_light_count as usize].light_type = crate::engine::light::LightType::Spot;
                    eng.lights[eng.used_light_count as usize].camera.fov = 110.0;
                    eng.lights[eng.used_light_count as usize].color = Vec3 {
                        x: 1.0,
                        y: 0.9,
                        z: 0.5,
                    };
                    eng.used_light_count+=1;
                }
            }

            if state.selp == 1{
                eng.lights[0].color = Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };

                eng.lights[eng.used_light_count as usize].rot.y = -state.scn.objects[state.pu].physic_object.rot.y;
                eng.lights[eng.used_light_count as usize].rot.x = 0.0;
                eng.lights[eng.used_light_count as usize].pos.x =
                    state.scn.objects[state.pu].physic_object.pos.x - state.scn.objects[state.pu].physic_object.rot.y.sin() * 0.3;
                eng.lights[eng.used_light_count as usize].pos.y = state.scn.objects[state.pu].physic_object.pos.y;
                eng.lights[eng.used_light_count as usize].pos.z =
                    state.scn.objects[state.pu].physic_object.pos.z - state.scn.objects[state.pu].physic_object.rot.y.cos() * 0.3;
                eng.lights[eng.used_light_count as usize].light_type = crate::engine::light::LightType::Spot;
                eng.lights[eng.used_light_count as usize].camera.fov = 90.0;
                eng.lights[eng.used_light_count as usize].color = Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };
                eng.used_light_count = eng.used_light_count + 1;
            }
        }
        _ => {
        }
    }
}

fn process_button_interactions(eng: &mut Engine, state: &mut AppState) {
    let mut button_status: Vec<(u32, u32, bool)> = state.btns.iter().map(|b| (b.in_scene_index, b.scene_index, b.pressed)).collect();
    for i in 0..state.btns.len() {
        let button = &mut state.btns[i];
        let player_pos = state.scn.objects[state.pu].physic_object.pos;
        let button_pos = state.scn.objects[button.index].physic_object.pos;
        let dist = distance(player_pos, button_pos);
        if button.axis < 4 {
            let can_use = dist <= 1.0 && (button.scene_index != 3 || state.ekey == usize::MAX);
            let show_nk = dist <= 1.0 && button.scene_index == 3 && state.ekey != usize::MAX;
            if (can_use || show_nk) && state.selp == 0 {
                let icon = if show_nk { &mut state.nkbtn } else { &mut state.btnbtn };
                icon.object.physic_object.scale.x = 80.0;
                icon.object.physic_object.scale.y = 80.0;
                icon.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0 - icon.object.physic_object.scale.x / 2.0;
                icon.object.physic_object.pos.y = eng.render.resolution_y as f32 - icon.object.physic_object.scale.y * 2.0 - 20.0;
                icon.object.draw = true;
                let icon_pressed = icon.exec(eng) && !show_nk && eng.control.mousebtn[2];
                let abtn_pressed = eng.control.gamepad_button_count > 0 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && !show_nk;
                if can_use && ((eng.control.get_key_state(state.keycodes[0]) || icon_pressed || abtn_pressed) && state.tm <= 0) {
                        let rot_axis = button.axis;
                        match rot_axis {
                            0 => state.scn.objects[button.index].physic_object.rot.x += PI,
                            1 => state.scn.objects[button.index].physic_object.rot.y += PI,
                            2 => state.scn.objects[button.index].physic_object.rot.z += PI,
                            _ => {}
                        }
                        state.sfx[3].move_sound_cursor(0.0);
                        state.sfx[3].play = true;
                        
                        match button.scene_index {
                            2 => {
                                let play_powered = match button.in_scene_index {
                                    5 | 4 | 3 => true,
                                    1 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 2 && pressed),
                                    2 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 2 && pressed),
                                    6 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 5 && scene_idx == 2 && pressed),
                                    _ => false,
                                };
                                if play_powered && !button.pressed{
                                    state.sfx[4].move_sound_cursor(0.0);
                                    state.sfx[4].play = true;
                                }
                                button.pressed = !button.pressed;
                                button_status[i] = (button.in_scene_index, button.scene_index, button.pressed);
                            
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 1 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 2 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 2 && pressed)&& 
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 2 && pressed){
                                    state.switched_1_4 = true;
                                    if state.dbg {
                                        println!("switched 1, 2, 3, 4");
                                    }
                                }else {
                                    state.switched_1_4 = false;
                                }
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 5 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 6 && scene_idx == 2 && pressed){
                                    state.switched_5_6 = true;
                                    if state.dbg {
                                        println!("switched 5 and 6");
                                    }
                                }else{
                                    state.switched_5_6 = false;
                                }
                            },
                            3 =>{
                                button.pressed = !button.pressed;
                                button_status[i] = (button.in_scene_index, button.scene_index, button.pressed);

                                if state.dbg {
                                    println!("scene 3 button, idx = {}, in_scene_index = {}, pressed = {}", button.index, button.in_scene_index, button.pressed);
                                }
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 3 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 2 && scene_idx == 3 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 3 && pressed){
                                    state.sc3state = 1;
                                    if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 1 && scene_idx == 3 && pressed){
                                        state.sc3state = 2;
                                    }
                                    if state.dbg {
                                        println!("scene 3 state = {}", state.sc3state);
                                    }
                                }else {
                                    state.sc3state = 0;
                                }
                            },
                            _ => (),
                        }
                        state.tm = 50;
                    }
                break;
            } else {
                state.btnbtn.object.draw = false;
                state.nkbtn.object.draw = false;
                state.btnbtn.exec(eng);
                state.nkbtn.exec(eng);
            }
        } else {
            let same_index_pressed = button_status.iter().any(|&(idx, scene_idx, pressed)| idx == button.in_scene_index && scene_idx == button.scene_index && pressed);
            let rot_axis = if button.axis >= 4 { button.axis - 3 } else { button.axis };
            if same_index_pressed {
                match rot_axis {
                    0 => state.scn.objects[button.index].physic_object.rot.x = PI,
                    1 => state.scn.objects[button.index].physic_object.rot.y = PI,
                    2 => state.scn.objects[button.index].physic_object.rot.z = PI,
                    _ => {}
                }
            } else {
                match rot_axis {
                    0 => state.scn.objects[button.index].physic_object.rot.x = 0.0,
                    1 => state.scn.objects[button.index].physic_object.rot.y = 0.0,
                    2 => state.scn.objects[button.index].physic_object.rot.z = 0.0,
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_scene(eng: &mut Engine, state: &mut AppState) {
    if state.gameending{
        handle_end(eng, state);
    }else{
        state.blacktxt[state.abc].draw = false;
        state.blacktxt[state.abc].exec(eng, " ");

        match state.cstop {
            1 => {
                if state.skp2 {
                    state.switched_5_6 = true;
                    state.switched_1_4 = true; 
                }
                if state.switched_1_4{
                    match state.doors[0].axis {
                        0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x - state.doors[0].movement,
                        1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y - state.doors[0].movement,
                        2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z - state.doors[0].movement,
                        _ => {}
                    }
                    state.sfx[2].play = true;
                }else{
                    match state.doors[0].axis {
                        0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x,
                        1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y,
                        2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z,
                        _ => {}
                    }
                }
            }
            2 => {
                if state.dbg {
                    state.ekey = usize::MAX;
                    state.gkey = usize::MAX;
                }
                if state.sc3state != 0{
                    match state.doors[1].axis {
                        0 => state.scn.objects[state.doors[1].index].physic_object.pos.x = state.doors[1].initial_pos.x - state.doors[1].movement,
                        1 => state.scn.objects[state.doors[1].index].physic_object.pos.y = state.doors[1].initial_pos.y - state.doors[1].movement,
                        2 => state.scn.objects[state.doors[1].index].physic_object.pos.z = state.doors[1].initial_pos.z - state.doors[1].movement,
                        _ => {}
                    }
                }else{
                    match state.doors[1].axis {
                        0 => state.scn.objects[state.doors[1].index].physic_object.pos.x = state.doors[1].initial_pos.x,
                        1 => state.scn.objects[state.doors[1].index].physic_object.pos.y = state.doors[1].initial_pos.y,
                        2 => state.scn.objects[state.doors[1].index].physic_object.pos.z = state.doors[1].initial_pos.z,
                        _ => {}
                    }
                }

                handle_final_door_interaction(eng, state);
            }
            _ => {
            }
        }
    }
    //state.btnbtn.object.draw = false;
    //state.nkbtn.object.draw = false;

    process_lighting(eng, state);

    if !state.pausemn && !state.gameending{
        process_button_interactions(eng, state);
    }else{
        state.btnbtn.object.draw = false;
        state.nkbtn.object.draw = false;
        state.nebtn.object.draw = false;
        state.drbtn.object.draw = false;
        state.btnbtn.exec(eng);
        state.nkbtn.exec(eng);
        state.nebtn.exec(eng);
        state.drbtn.exec(eng);
    }

    for i in 0..state.destructables.len(){
        if state.destructables[i].destroyed{
            state.scn.objects[state.destructables[i].index].draw = false;
            state.scn.objects[state.destructables[i].index].physic_object.pos.y = -1000.0;
        }
    }
}