use crate::{
    app_state::AppState,
    engine::{engine::Engine, math::{vec2::Vec2, vec3::Vec3}}, loop_tick::save_load::{load_progress, save_progress, save_settings},
};

use super::handle_scene::reset_final_door_game;

pub fn menu_handle(eng: &mut Engine, state: &mut AppState) {
    if state.pausemn{
        state.selp = 0;
        match state.menusel{
            0 => {
                state.menumasel = 6;

                state.logo.object.physic_object.scale.x = 200f32;
                state.logo.object.physic_object.scale.y = 200f32;
                state.logo.object.physic_object.pos.x = 0.0;
                state.logo.object.physic_object.pos.y = eng.render.resolution_y as f32/2.0 - 241.0;
                state.logo.object.draw = true;
                state.logo.signal = false;
                state.logo.exec(eng);

                let mut lg = state.logo.object.physic_object.scale.x;

                for i in 0..6{
                    state.ruitxt[state.abc][i].signal = true;
                    state.ruitxt[state.abc][i].per_symbol = false;
                    state.ruitxt[state.abc][i].draw = true;
                    state.ruitxt[state.abc][i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[state.abc][i].pos = Vec3{ x: 0f32, y: state.logo.object.physic_object.pos.y+state.logo.object.physic_object.scale.y+6.0+i as f32*(state.ruitxt[state.abc][i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[i+1].strval.clone();
                    if txt.len() as f32 * state.ruitxt[state.abc][i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[state.abc][i].size.x;
                    }
                    if (i == 2 || i == 1 || i == 3) && (state.intram || state.gameending){
                        state.ruitxt[state.abc][i].signal_on_value = 11.0;
                        state.ruitxt[state.abc][i].signal_off_value = 11.0;
                    }else{
                        state.ruitxt[state.abc][i].signal_on_value = 1.0;
                        state.ruitxt[state.abc][i].signal_off_value = 0.0;
                    }

                    if state.controlt == 2 && !((i == 2 || i == 1 || i == 3) && (state.intram || state.gameending)){
                        if i == state.gamepadmenusel as usize{
                            state.ruitxt[state.abc][i].signal_off_value = 1.0;
                        }else{
                            state.ruitxt[state.abc][i].signal_off_value = 0.0;
                        }
                    }

                    if (state.ruitxt[state.abc][i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (i == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                        match i {
                            0 => {
                                state.pausemn = !state.pausemn;
                                state.tm = 50;
                            },
                            1 => {
                                if !state.gameending && !state.intram{
                                    reset_final_door_game(state);
                                    state.pkbf = 2.0;
                                    state.tm = 50;
                                    state.pausemn = false;
                                    state.sfx[1].move_sound_cursor(0.0);
                                    state.sfx[1].play = true;
                                }
                            }
                            2 => {
                                if !state.intram && !state.gameending{
                                    let _ = save_progress("save.json", state);
                                    state.pausemn = false;
                                    state.tm = 50;
                                }
                            }
                            3 => {
                                if !state.intram && !state.gameending{
                                    let _ = load_progress("save.json", state);
                                    state.pausemn = false;
                                    state.pkbf = 2.0;
                                    state.sfx[1].move_sound_cursor(0.0);
                                    state.sfx[1].play = true;
                                    state.tm = 50;
                                }
                            }
                            4 => {
                                state.menusel = 1;
                                state.gamepadmenusel = 0;
                                state.tm = 50;
                            }
                            5 => {
                                state.close = true;
                            },
                            _ => {}
                        }
                    }
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            1 => {
                state.menumasel = 5;
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[state.abc][0].signal = false;
                state.ruitxt[state.abc][0].per_symbol = false;
                state.ruitxt[state.abc][0].draw = true;
                state.ruitxt[state.abc][0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[state.abc][0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 148.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[5].strval.clone();
                state.ruitxt[state.abc][0].exec(eng, &txt);

                let mut lg = state.ruitxt[state.abc][0].size.x*txt.len() as f32;

                for i in 1..5{
                    let newind = i+6;
                    state.ruitxt[state.abc][i].signal = true;
                    if state.controlt == 2{
                        if i-1 == state.gamepadmenusel as usize{
                            state.ruitxt[state.abc][i].signal_off_value = 1.0;
                        }else{
                            state.ruitxt[state.abc][i].signal_off_value = 0.0;
                        }
                    }
                    state.ruitxt[state.abc][i].per_symbol = false;
                    state.ruitxt[state.abc][i].draw = true;
                    state.ruitxt[state.abc][i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[state.abc][i].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+(i-1) as f32*(state.ruitxt[state.abc][i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[newind].strval.clone();
                    if txt.len() as f32 * state.ruitxt[state.abc][i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[state.abc][i].size.x;
                    }
                    if (state.ruitxt[state.abc][i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                        match i {
                            1 => {
                                state.menusel = 2;
                                state.gamepadmenusel = 0;
                                state.tm = 50;
                            },
                            2 => {
                                state.menusel = 3;
                                state.gamepadmenusel = 0;
                                state.tm = 50;
                            },
                            3 => {
                                state.menusel = 4;
                                state.gamepadmenusel = 0;
                                state.tm = 50;
                            },
                            4 => {
                                state.menusel = 5;
                                state.gamepadmenusel = 0;
                                state.tm = 50;
                            },
                            _ => {}
                        }
                    }
                }

                state.ruitxt[state.abc][5].signal = true;
                if state.controlt == 2{
                    if 4 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][5].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][5].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][5].per_symbol = false;
                state.ruitxt[state.abc][5].draw = true;
                state.ruitxt[state.abc][5].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][5].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+4.0*(state.ruitxt[state.abc][5].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[state.abc][5].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[state.abc][5].size.x;
                }
                if (state.ruitxt[state.abc][5].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (4 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.menusel = 0;
                    state.gamepadmenusel = 0;
                    state.tm = 50;
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            }
            2 => {
                state.menumasel = 5;
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[state.abc][0].signal = false;
                state.ruitxt[state.abc][0].per_symbol = false;
                state.ruitxt[state.abc][0].draw = true;
                state.ruitxt[state.abc][0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[state.abc][0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 148.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[7].strval.clone();
                state.ruitxt[state.abc][0].exec(eng, &txt);

                let mut lg = state.ruitxt[state.abc][0].size.x*txt.len() as f32;

                for i in 1..5{
                    let newind = i+10;
                    state.ruitxt[state.abc][i].signal = true;
                    if state.controlt == 2{
                        if i-1 == state.gamepadmenusel as usize{
                            state.ruitxt[state.abc][i].signal_off_value = 1.0;
                        }else{
                            state.ruitxt[state.abc][i].signal_off_value = 0.0;
                        }
                    }
                    state.ruitxt[state.abc][i].per_symbol = false;
                    state.ruitxt[state.abc][i].draw = true;
                    state.ruitxt[state.abc][i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[state.abc][i].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+(i-1) as f32*(state.ruitxt[state.abc][i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[newind].strval.clone();
                    if (txt.len() + 5) as f32 * state.ruitxt[state.abc][i].size.x > lg{
                        lg = (txt.len() + 5) as f32 * state.ruitxt[state.abc][i].size.x;
                    }
                    match i {
                        1 => {
                            //state.menusel = 2;
                            if (state.ruitxt[state.abc][i].exec(eng, &format!("{}{}", txt, eng.render.fullscreen as u32)) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                eng.render.fullscreen = !eng.render.fullscreen;
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        },
                        2 => {
                            if (state.ruitxt[state.abc][i].exec(eng, &format!("{}{}", txt, (eng.render.resolution_scale*100.0) as u32)) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                eng.render.resolution_scale = ((eng.render.resolution_scale*100.0) as u32 - 10) as f32 / 100.0;
                                if eng.render.resolution_scale < 0.2{
                                    eng.render.resolution_scale = 1.0;
                                }
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        },
                        3 => {
                            if (state.ruitxt[state.abc][i].exec(eng, &format!("{}{}", txt, match state.shadowmapquality{
                                    1000 => "1",
                                    2000 => "2",
                                    4000 => "3",
                                    _ => "?",
                                })) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                state.shadowmapquality = match state.shadowmapquality{
                                    1000 => 2000,
                                    2000 => 4000,
                                    4000 => 1000,
                                    _ => 1000,
                                };
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        },
                        4 => {
                            if (state.ruitxt[state.abc][i].exec(eng, &format!("{}{}", txt, state.showfps as u32)) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                state.showfps = !state.showfps;
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        },
                        _ => {}
                    }
                }

                state.ruitxt[state.abc][5].signal = true;
                if state.controlt == 2{
                    if 4 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][5].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][5].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][5].per_symbol = false;
                state.ruitxt[state.abc][5].draw = true;
                state.ruitxt[state.abc][5].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][5].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+4.0*(state.ruitxt[state.abc][5].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[state.abc][5].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[state.abc][5].size.x;
                }
                if (state.ruitxt[state.abc][5].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (4 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.menusel = 1;
                    state.gamepadmenusel = 0;
                    state.tm = 50;
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            3 => {
                state.menumasel = 2;
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[state.abc][0].signal = false;
                state.ruitxt[state.abc][0].per_symbol = false;
                state.ruitxt[state.abc][0].draw = true;
                state.ruitxt[state.abc][0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[state.abc][0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 79.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[8].strval.clone();
                state.ruitxt[state.abc][0].exec(eng, &txt);

                let mut lg = state.ruitxt[state.abc][0].size.x*txt.len() as f32;

                state.ruitxt[state.abc][1].signal = true;
                if state.controlt == 2{
                    if 0 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][1].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][1].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][1].per_symbol = false;
                state.ruitxt[state.abc][1].draw = true;
                state.ruitxt[state.abc][1].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][1].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[15].strval.clone();
                if (txt.len()+3) as f32 * state.ruitxt[state.abc][1].size.x > lg{
                    lg = (txt.len()+3) as f32 * state.ruitxt[state.abc][1].size.x;
                }
                if (state.ruitxt[state.abc][1].exec(eng, &format!("{}{}", txt, (eng.audio.vol*100.0) as i32)) && eng.control.mousebtn[2] && state.tm <= 0) || (0 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    eng.audio.vol = ((eng.audio.vol*100.0) as i32 - 10) as f32 / 100.0;
                    if eng.audio.vol < 0.0{
                        eng.audio.vol = 1.0;
                    }
                    let _ = save_settings("settings.json", eng, state);
                    state.tm = 50;
                }

                state.ruitxt[state.abc][2].signal = true;
                if state.controlt == 2{
                    if 1 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][2].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][2].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][2].per_symbol = false;
                state.ruitxt[state.abc][2].draw = true;
                state.ruitxt[state.abc][2].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][2].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+(state.ruitxt[state.abc][2].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[state.abc][2].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[state.abc][5].size.x;
                }
                if (state.ruitxt[state.abc][2].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.menusel = 1;
                    state.gamepadmenusel = 0;
                    state.tm = 50;
                }

                for i in 3..state.ruitxt[state.abc].len(){
                    state.ruitxt[state.abc][i].draw = false;
                    state.ruitxt[state.abc][i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            4 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[state.abc][0].signal = false;
                state.ruitxt[state.abc][0].per_symbol = false;
                state.ruitxt[state.abc][0].draw = true;
                state.ruitxt[state.abc][0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[state.abc][0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 79.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[9].strval.clone();
                state.ruitxt[state.abc][0].exec(eng, &txt);

                let mut lg = state.ruitxt[state.abc][0].size.x*txt.len() as f32;

                state.ruitxt[state.abc][1].signal = true;
                if state.controlt == 2{
                    if 0 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][1].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][1].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][1].per_symbol = false;
                state.ruitxt[state.abc][1].draw = true;
                state.ruitxt[state.abc][1].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][1].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[16].strval.clone();
                if (txt.len() + 5) as f32 * state.ruitxt[state.abc][1].size.x > lg{
                    lg = (txt.len() + 5) as f32 * state.ruitxt[state.abc][1].size.x;
                }
                if (state.ruitxt[state.abc][1].exec(eng, &format!("{}{}", txt, state.left_hand as u32)) && eng.control.mousebtn[2] && state.tm <= 0) || (0 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.left_hand = !state.left_hand;
                    let _ = save_settings("settings.json", eng, state);
                    state.tm = 50;
                }

                state.ruitxt[state.abc][2].signal = true;
                if state.controlt == 2{
                    if 1 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][2].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][2].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][2].per_symbol = false;
                state.ruitxt[state.abc][2].draw = true;
                state.ruitxt[state.abc][2].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][2].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+(state.ruitxt[state.abc][2].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[state.abc][2].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[state.abc][5].size.x;
                }
                if (state.ruitxt[state.abc][2].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.menusel = 1;
                    state.gamepadmenusel = 0;
                    state.tm = 50;
                }

                for i in 3..state.ruitxt[state.abc].len(){
                    state.ruitxt[state.abc][i].draw = false;
                    state.ruitxt[state.abc][i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            5 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[state.abc][0].signal = false;
                state.ruitxt[state.abc][0].per_symbol = false;
                state.ruitxt[state.abc][0].draw = true;
                state.ruitxt[state.abc][0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[state.abc][0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 102.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[9].strval.clone();
                state.ruitxt[state.abc][0].exec(eng, &txt);

                let mut lg = state.ruitxt[state.abc][0].size.x*txt.len() as f32;

                for i in 1..3{
                    let newind = i+16;
                    state.ruitxt[state.abc][i].signal = true;
                    if state.controlt == 2{
                        if i-1 == state.gamepadmenusel as usize{
                            state.ruitxt[state.abc][i].signal_off_value = 1.0;
                        }else{
                            state.ruitxt[state.abc][i].signal_off_value = 0.0;
                        }
                    }
                    state.ruitxt[state.abc][i].per_symbol = false;
                    state.ruitxt[state.abc][i].draw = true;
                    state.ruitxt[state.abc][i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[state.abc][i].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+(i-1) as f32*(state.ruitxt[state.abc][i].size.y+6.0), z: 0.1};
                    match i {
                        1 => {
                            //state.menusel = 2;
                            let txt = format!("{}{}", state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[newind].strval.clone(), state.autosaves as u32);
                            if txt.len() as f32 * state.ruitxt[state.abc][i].size.x > lg{
                                lg = txt.len() as f32 * state.ruitxt[state.abc][i].size.x;
                            }
                            if (state.ruitxt[state.abc][i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                state.autosaves = !state.autosaves;
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        },
                        2 => {
                            //state.menusel = 3;
                            let txt = format!("{}", state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[newind].strval.clone());
                            if txt.len() as f32 * state.ruitxt[state.abc][i].size.x > lg{
                                lg = txt.len() as f32 * state.ruitxt[state.abc][i].size.x;
                            }
                            if (state.ruitxt[state.abc][i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (i-1 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                                //let mln = jsontext.other_nodes[0].other_nodes.len();
                                //let abc = jsontext.other_nodes[0].other_nodes[0].other_nodes[0].numeral_val as usize;
                                state.current_lang += 1;
                                if state.current_lang >= state.max_lang_nm{
                                    state.current_lang = 0;
                                }
                                state.abc = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[0].numeral_val as usize;
                                let _ = save_settings("settings.json", eng, state);
                                state.tm = 50;
                            }
                        }
                        _ => {}
                    }
                }

                state.ruitxt[state.abc][3].signal = true;
                if state.controlt == 2{
                    if 2 == state.gamepadmenusel as usize{
                        state.ruitxt[state.abc][3].signal_off_value = 1.0;
                    }else{
                        state.ruitxt[state.abc][3].signal_off_value = 0.0;
                    }
                }
                state.ruitxt[state.abc][3].per_symbol = false;
                state.ruitxt[state.abc][3].draw = true;
                state.ruitxt[state.abc][3].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[state.abc][3].pos = Vec3{ x: 0f32, y: state.ruitxt[state.abc][0].pos.y+state.ruitxt[state.abc][0].size.y+6.0+2.0*(state.ruitxt[state.abc][3].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[state.abc][3].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[state.abc][3].size.x;
                }
                if (state.ruitxt[state.abc][3].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0) || (2 == state.gamepadmenusel as usize && state.controlt == 2 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.tm <= 0){
                    state.menusel = 1;
                    state.gamepadmenusel = 0;
                    state.tm = 50;
                }

                for i in 4..state.ruitxt[state.abc].len(){
                    state.ruitxt[state.abc][i].draw = false;
                    state.ruitxt[state.abc][i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            _ => {},
        }
    }else{
        for i in 1..8{
            state.ruitxt[state.abc][i].draw = false;
            state.ruitxt[state.abc][i].exec(eng, " ");
        }
        state.psbtn.object.draw = true;
    }
}